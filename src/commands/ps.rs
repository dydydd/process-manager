use comfy_table::{Table, ContentArrangement, Cell, CellAlignment};
use crate::process_info::{collect_processes, format_bytes, format_start_time, ProcessSnapshot};

pub fn list_processes(filter: &Option<String>, sort_by: &str, limit: Option<usize>) {
    let mut sys = sysinfo::System::new_all();
    let mut procs = collect_processes(&mut sys);

    // Apply filter
    if let Some(ref keyword) = filter {
        let lower = keyword.to_lowercase();
        procs.retain(|p| p.name.to_lowercase().contains(&lower) || p.pid.to_string().contains(&lower));
    }

    // Apply sort
    sort_processes(&mut procs, sort_by);

    // Apply limit
    if let Some(n) = limit {
        procs.truncate(n);
    }

    // Build table
    let mut table = Table::new();
    table
        .set_header(vec!["PID", "Name", "CPU %", "Memory", "Status", "Start Time"])
        .set_content_arrangement(ContentArrangement::Dynamic);

    for p in &procs {
        table.add_row(vec![
            Cell::new(p.pid.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(&p.name),
            Cell::new(format!("{:.1}", p.cpu_usage)).set_alignment(CellAlignment::Right),
            Cell::new(format_bytes(p.memory_bytes)).set_alignment(CellAlignment::Right),
            Cell::new(&p.status),
            Cell::new(format_start_time(p.start_time)),
        ]);
    }

    println!("\nTotal: {} processes\n", procs.len());
    println!("{table}");
}

pub fn sort_processes(procs: &mut Vec<ProcessSnapshot>, sort_by: &str) {
    match sort_by.to_lowercase().as_str() {
        "name" => procs.sort_by(|a, b| a.name.cmp(&b.name)),
        "cpu" => procs.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
        "memory" | "mem" => procs.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes)),
        "status" => procs.sort_by(|a, b| a.status.cmp(&b.status)),
        _ => procs.sort_by(|a, b| a.pid.cmp(&b.pid)),
    }
}
