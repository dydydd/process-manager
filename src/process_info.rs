use sysinfo::{System, ProcessStatus};

/// A lightweight snapshot of a running process.
pub struct ProcessSnapshot {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub status: String,
    pub parent_pid: u32,
    pub start_time: u64,
}

/// Refresh system and collect all process snapshots.
pub fn collect_processes(sys: &mut System) -> Vec<ProcessSnapshot> {
    sys.refresh_all();
    sys.processes()
        .iter()
        .map(|(pid, proc_)| ProcessSnapshot {
            pid: pid.as_u32(),
            name: proc_.name().to_string_lossy().into_owned(),
            cpu_usage: proc_.cpu_usage(),
            memory_bytes: proc_.memory(),
            status: status_to_string(proc_.status()),
            parent_pid: proc_.parent().map(|p| p.as_u32()).unwrap_or(0),
            start_time: proc_.start_time(),
        })
        .collect()
}

/// Convert ProcessStatus to a human-readable string.
pub fn status_to_string(status: ProcessStatus) -> String {
    match status {
        ProcessStatus::Run => "running",
        ProcessStatus::Sleep => "sleeping",
        ProcessStatus::Idle => "idle",
        ProcessStatus::Stop => "stopped",
        ProcessStatus::Zombie => "zombie",
        ProcessStatus::Tracing => "tracing",
        ProcessStatus::Dead => "dead",
        ProcessStatus::Wakekill => "wake_kill",
        ProcessStatus::Waking => "waking",
        ProcessStatus::Parked => "parked",
        ProcessStatus::LockBlocked => "lock_blocked",
        ProcessStatus::UninterruptibleDiskSleep => "disk_sleep",
        _ => "unknown",
    }
    .to_string()
}

/// Format bytes into human-readable string (KB, MB, GB).
pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Format start_time as a readable date string.
pub fn format_start_time(start_time: u64) -> String {
    chrono::DateTime::from_timestamp(start_time as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "N/A".into())
}
