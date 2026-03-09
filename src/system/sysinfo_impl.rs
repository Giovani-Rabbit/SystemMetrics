use sysinfo::System;

use crate::system::traits::{CpuInfo, MemoryInfo, SystemMonitor};

pub struct SysinfoMetrics {
    sys: System,
}

impl SysinfoMetrics {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }
}

impl SystemMonitor for SysinfoMetrics {
    fn cpu_info(&self) -> super::traits::CpuInfo {
        CpuInfo {
            core_amount: self.sys.cpus().len(),
            usage_percent: self.sys.global_cpu_usage(),
        }
    }

    fn memory_info(&self) -> super::traits::MemoryInfo {
        MemoryInfo {
            available_mb: bytes_to_mb(self.sys.available_memory()),
            total_mb: self.sys.total_memory(),
            used_mb: self.sys.used_memory(),
        }
    }

    fn refresh(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
    }
}

fn bytes_to_mb(b: u64) -> u64 {
    b / 1024 / 1024
}
