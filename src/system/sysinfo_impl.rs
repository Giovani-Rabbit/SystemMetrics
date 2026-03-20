use sysinfo::System;

use crate::system::traits::{CpuCore, CpuInfo, MemoryInfo, SystemMonitor};

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
        let cores = self
            .sys
            .cpus()
            .iter()
            .map(|c| CpuCore {
                usage: c.cpu_usage(),
                frequency: c.frequency(),
            })
            .collect();

        CpuInfo {
            usage_percent: self.sys.global_cpu_usage(),
            cores,
        }
    }

    fn memory_info(&self) -> super::traits::MemoryInfo {
        MemoryInfo {
            available: bytes_to_mb(self.sys.available_memory()),
            total: self.sys.total_memory(),
            used: self.sys.used_memory(),

            total_swap: self.sys.total_swap(),
            used_swap: self.sys.used_swap(),
            free_swap: self.sys.free_swap(),
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

pub fn bytes_to_g(b: u64) -> u64 {
    b / 1024 / 1024 / 1024
}
