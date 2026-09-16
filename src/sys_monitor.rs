use std::thread;
use std::time::Duration;
use sysinfo::System;

struct RamStats {
    total: u64,
    available: u64,
    used: u64,
}

impl RamStats {
    fn to_gb(bytes: u64) -> f64 {
        bytes as f64 / 1024.0 / 1024.0 / 1024.0
    }
    fn total_gb(&self) -> f64 {
        Self::to_gb(self.total)
    }
    fn available_gb(&self) -> f64 {
        Self::to_gb(self.available)
    }
    fn used_gb(&self) -> f64 {
        Self::to_gb(self.used)
    }
}

pub fn sysinfo() {
    let mut sys = System::new_all();
    loop {
        if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .args(["/C", "cls"])
                .status()
                .unwrap();
        } else {
            std::process::Command::new("clear").status().unwrap();
        }

        sys.refresh_memory();
        sys.refresh_cpu_all();

        let cpu_stats = sys.global_cpu_usage();

        let stats = RamStats {
            total: sys.total_memory(),
            available: sys.available_memory(),
            used: sys.used_memory(),
        };
        println!(
            "CPU Usage: {:.1}% | Total RAM: {:.2}GB | Available RAM: {:.2}GB | Used RAM: {:.2}GB",
            cpu_stats,
            stats.total_gb(),
            stats.available_gb(),
            stats.used_gb()
        );
        thread::sleep(Duration::from_secs(2))
    }
}
