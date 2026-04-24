use std::{io, time::Duration, cmp::Ordering};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState, Paragraph},
    Terminal, Frame,
};
use sysinfo::{Pid, ProcessStatus, System};

#[derive(Clone)]
struct ProcEntry {
    pid: u32,
    ppid: u32,
    name: String,
    cpu: f32,
    mem: u64,
    mem_str: String,
    status: String,
    user: String,
}

#[derive(PartialEq)]
enum SortField { Cpu, Mem, Name, Pid, User }

struct App {
    sys: System,
    procs: Vec<ProcEntry>,
    sort: SortField,
    selected: usize,
    search: String,
    searching: bool,
    running: bool,
    total_cpu: f32,
    total_mem: u64,
    used_mem: u64,
    task_count: usize,
}

impl App {
    fn new() -> Self {
        let mut app = Self {
            sys: System::new(),
            procs: Vec::new(),
            sort: SortField::Cpu,
            selected: 0,
            search: String::new(),
            searching: false,
            running: true,
            total_cpu: 0.0,
            total_mem: 0,
            used_mem: 0,
            task_count: 0,
        };
        app.refresh();
        app
    }

    fn refresh(&mut self) {
        self.sys.refresh_all();
        self.total_cpu = self.sys.global_cpu_usage();
        self.total_mem = self.sys.total_memory();
        self.used_mem = self.sys.used_memory();
        self.task_count = self.sys.processes().len();

        self.procs.clear();
        for (pid, p) in self.sys.processes() {
            self.procs.push(ProcEntry {
                pid: pid.as_u32(),
                ppid: p.parent().map(|p| p.as_u32()).unwrap_or(0),
                name: p.name().to_string_lossy().into_owned(),
                cpu: p.cpu_usage(),
                mem: p.memory(),
                mem_str: format_mem(p.memory()),
                status: status_str(&p.status()).to_string(),
                user: p.user_id().map(|u| format!("{:?}", u)).unwrap_or_default(),
            });
        }
        self.sort_procs();
        if self.selected >= self.procs.len() {
            self.selected = self.procs.len().saturating_sub(1);
        }
    }

    fn sort_procs(&mut self) {
        self.procs.sort_by(|a, b| match self.sort {
            SortField::Cpu => b.cpu.partial_cmp(&a.cpu).unwrap_or(Ordering::Equal),
            SortField::Mem => b.mem.cmp(&a.mem),
            SortField::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            SortField::Pid => a.pid.cmp(&b.pid),
            SortField::User => a.user.cmp(&b.user),
        });
    }

    fn filtered(&self) -> Vec<&ProcEntry> {
        if self.search.is_empty() {
            self.procs.iter().collect()
        } else {
            let q = self.search.to_lowercase();
            self.procs.iter().filter(|p|
                p.name.to_lowercase().contains(&q)
                || p.pid.to_string().contains(&q)
                || p.user.to_lowercase().contains(&q)
            ).collect()
        }
    }

    fn selected_proc(&self) -> Option<&ProcEntry> {
        let filtered = self.filtered();
        if self.selected < filtered.len() {
            Some(filtered[self.selected])
        } else {
            None
        }
    }

    fn kill_selected(&mut self) {
        if let Some(proc) = self.selected_proc() {
            let pid = Pid::from_u32(proc.pid);
            if let Some(p) = self.sys.process(pid) {
                p.kill();
            }
            self.refresh();
        }
    }

    fn toggle_search(&mut self) {
        self.searching = !self.searching;
        if self.searching {
            self.search.clear();
        }
    }

    fn cycle_sort(&mut self) {
        self.sort = match self.sort {
            SortField::Cpu => SortField::Mem,
            SortField::Mem => SortField::Name,
            SortField::Name => SortField::Pid,
            SortField::Pid => SortField::User,
            SortField::User => SortField::Cpu,
        };
        self.sort_procs();
    }
}

fn format_mem(bytes: u64) -> String {
    if bytes < 1024 * 1024 {
        format!("{:.0}K", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}M", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2}G", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn status_str(s: &ProcessStatus) -> &'static str {
    match s {
        ProcessStatus::Run => "R",
        ProcessStatus::Sleep => "S",
        ProcessStatus::Idle => "I",
        ProcessStatus::Stop => "T",
        ProcessStatus::Zombie => "Z",
        _ => "?",
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Header - CPU/Mem info
    let cpu_pct = if app.total_cpu.is_nan() { 0.0 } else { app.total_cpu };
    let mem_pct = if app.total_mem > 0 {
        app.used_mem as f64 / app.total_mem as f64 * 100.0
    } else { 0.0 };

    let sort_label = match app.sort {
        SortField::Cpu => "CPU%",
        SortField::Mem => "MEM%",
        SortField::Name => "NAME",
        SortField::Pid => "PID",
        SortField::User => "USER",
    };

    let header_text = format!(
        " Tasks: {} | CPU: {:.1}% | Mem: {:.1}% | Sort: {} | F5:Refresh  F6:Sort  F3:Search  F9:Kill  q:Quit",
        app.task_count, cpu_pct, mem_pct, sort_label
    );
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" Process Manager "));
    frame.render_widget(header, chunks[0]);

    // Process table
    let filtered = app.filtered();
    let header_cells = ["PID", "User", "Status", "CPU%", "MEM", "Name"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));

    let rows: Vec<Row> = filtered.iter().enumerate().map(|(i, p)| {
        let style = if i == app.selected {
            Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::BOLD)
        } else if i % 2 == 0 {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::Gray)
        };
        Row::new(vec![
            Cell::from(p.pid.to_string()),
            Cell::from(p.user.clone()),
            Cell::from(p.status.clone()),
            Cell::from(format!("{:.1}", p.cpu)),
            Cell::from(p.mem_str.clone()),
            Cell::from(p.name.clone()),
        ]).style(style)
    }).collect();

    let widths = [
        Constraint::Length(7),
        Constraint::Length(12),
        Constraint::Length(8),
        Constraint::Length(7),
        Constraint::Length(10),
        Constraint::Min(20),
    ];

    let table = Table::new(rows, widths)
        .header(Row::new(header_cells).style(Style::default().fg(Color::Yellow)))
        .block(Block::default().borders(Borders::ALL).title(" Processes "))
        .row_highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut state = TableState::default();
    state.select(Some(app.selected));
    frame.render_stateful_widget(table, chunks[1], &mut state);

    // Footer - search bar or status
    if app.searching {
        let search_text = format!(" Search: {}_", app.search);
        let search_bar = Paragraph::new(search_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title(" Search (Esc to cancel) "));
        frame.render_widget(search_bar, chunks[2]);
    } else {
        if let Some(p) = app.selected_proc() {
            let info = format!(
                " PID: {} | PPID: {} | User: {} | CPU: {:.1}% | MEM: {} | Status: {} | {}",
                p.pid, p.ppid, p.user, p.cpu, format_mem(p.mem), p.status, p.name
            );
            let footer = Paragraph::new(info)
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title(" Process Info "));
            frame.render_widget(footer, chunks[2]);
        } else {
            let footer = Paragraph::new("No process selected")
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(footer, chunks[2]);
        }
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(1000);

    while app.running {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if app.searching {
                    match key.code {
                        KeyCode::Esc => app.searching = false,
                        KeyCode::Enter => app.searching = false,
                        KeyCode::Char(c) => app.search.push(c),
                        KeyCode::Backspace => { app.search.pop(); }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => app.running = false,
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.selected < app.filtered().len() - 1 {
                                app.selected += 1;
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.selected > 0 {
                                app.selected -= 1;
                            }
                        }
                        KeyCode::PageDown => {
                            app.selected = (app.selected + 20).min(app.filtered().len().saturating_sub(1));
                        }
                        KeyCode::PageUp => {
                            app.selected = app.selected.saturating_sub(20);
                        }
                        KeyCode::Home => app.selected = 0,
                        KeyCode::End => app.selected = app.filtered().len().saturating_sub(1),
                        KeyCode::F(5) => app.refresh(),
                        KeyCode::F(6) => app.cycle_sort(),
                        KeyCode::F(3) | KeyCode::Char('/') => app.toggle_search(),
                        KeyCode::F(9) | KeyCode::Char('K') => app.kill_selected(),
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
