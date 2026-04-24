use std::io::{stdout, Write};
use std::time::Duration;
use std::thread;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use comfy_table::{Table, ContentArrangement, Cell};
use crate::process_info::{collect_processes, format_bytes};
use crate::commands::ps::sort_processes;

pub fn run(interval: u64, sort_by: &str) {
    println!("Starting top mode (interval: {}s, sort: {}). Press Ctrl+C to exit.\n", interval, sort_by);

    let mut sys = sysinfo::System::new_all();

    loop {
        let mut procs = collect_processes(&mut sys);
        sort_processes(&mut procs, sort_by);

        // Take top 30
        procs.truncate(30);

        let mut table = Table::new();
        table
            .set_header(vec!["PID", "Name", "CPU %", "Memory", "Status"])
            .set_content_arrangement(ContentArrangement::Dynamic);

        for p in &procs {
            table.add_row(vec![
                Cell::new(p.pid.to_string()),
                Cell::new(&p.name),
                Cell::new(format!("{:.1}", p.cpu_usage)),
                Cell::new(format_bytes(p.memory_bytes)),
                Cell::new(&p.status),
            ]);
        }

        // Clear screen and move cursor to top
        let mut stdout = stdout();
        let _ = execute!(stdout, MoveTo(0, 0), Clear(ClearType::All));

        let total_cpu = sys.global_cpu_usage();
        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();

        println!("=== Process Monitor | CPU: {:.1}% | Memory: {:.1}% ===",
            total_cpu,
            (used_mem as f64 / total_mem as f64) * 100.0
        );
        println!("{table}");
        println!("\nPress Ctrl+C to exit.");

        stdout.flush().unwrap();
        thread::sleep(Duration::from_secs(interval));
    }
}
