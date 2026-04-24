use sysinfo::System;
use crate::process_info::{format_bytes, format_start_time, status_to_string};

pub fn show_info(pid: u32) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid_obj = sysinfo::Pid::from(pid as usize);
    match sys.process(pid_obj) {
        Some(proc_) => {
            println!("\n=== Process Information (PID: {}) ===\n", pid);
            println!("  Name:         {}", proc_.name().to_string_lossy());
            println!("  PID:          {}", pid);
            println!("  Parent PID:   {}", proc_.parent().map(|p| p.as_u32().to_string()).unwrap_or_else(|| "None".into()));
            println!("  Status:       {}", status_to_string(proc_.status()));
            println!("  CPU Usage:    {:.1}%", proc_.cpu_usage());
            println!("  Memory:       {} ({} bytes)", format_bytes(proc_.memory()), proc_.memory());
            println!("  Virtual:      {} ({} bytes)", format_bytes(proc_.virtual_memory()), proc_.virtual_memory());
            println!("  Start Time:   {}", format_start_time(proc_.start_time()));
            println!("  Run Time:     {}s", proc_.run_time());

            if let Some(root) = proc_.exe() {
                println!("  Executable:   {}", root.display());
            }
            if let Some(cwd) = proc_.cwd() {
                println!("  Working Dir:  {}", cwd.display());
            }

            let cmd = proc_.cmd();
            if !cmd.is_empty() {
                println!("  Command:      {}", cmd.iter().map(|s| s.to_string_lossy()).collect::<Vec<_>>().join(" "));
            }

            println!();
        }
        None => {
            eprintln!("Process with PID {} not found.", pid);
        }
    }
}
