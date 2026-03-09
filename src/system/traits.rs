pub struct CpuInfo {
    pub usage_percent: f32,
    pub core_amount: usize,
}

pub struct MemoryInfo {
    pub total_mb: u64,
    pub used_mb: u64,
    pub available_mb: u64,
}

pub trait SystemMonitor {
    fn cpu_info(&self) -> CpuInfo;
    fn memory_info(&self) -> MemoryInfo;
    fn refresh(&mut self);
}
