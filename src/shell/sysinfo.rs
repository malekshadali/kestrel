use sysinfo::System;

pub fn get_sysinfo() -> Vec<String> {
    let mut sys = System::new_all();
    sys.refresh_memory();
    sys.refresh_cpu_all();

    let cpu = sys.global_cpu_usage();
    let total = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    vec![
        format!("CPU {:.1}%", cpu),
        format!("RAM: {:.2}GB used / {:.2}GB total", used, total),
    ]
}