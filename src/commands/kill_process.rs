use sysinfo::System;

pub fn kill(pid: u32, force: bool) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid_obj = sysinfo::Pid::from(pid as usize);
    match sys.process(pid_obj) {
        Some(proc_) => {
            let name = proc_.name().to_string_lossy();
            if proc_.kill() {
                if force {
                    println!("Process {} (PID: {}) has been force killed.", name, pid);
                } else {
                    println!("Process {} (PID: {}) has been terminated.", name, pid);
                }
            } else {
                eprintln!("Failed to terminate process {} (PID: {}).", name, pid);
            }
        }
        None => {
            eprintln!("Process with PID {} not found.", pid);
        }
    }
}
