pub struct CpuInfo {
    pub usage_percent: f32,
    pub cores: Vec<CpuCore>,
}

pub struct CpuCore {
    pub usage: f32,
    pub frequency: u64,
}

pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,

    pub total_swap: u64,
    pub used_swap: u64,
    pub free_swap: u64,
}

pub trait SystemMonitor {
    fn cpu_info(&self) -> CpuInfo;
    fn memory_info(&self) -> MemoryInfo;
    fn refresh(&mut self);
}
