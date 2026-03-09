pub mod app;
pub mod system;
pub mod ui;

use crate::{app::App, system::sysinfo_impl::SysinfoMetrics};
use std::io;

fn main() -> io::Result<()> {
    let sysmon = Box::new(SysinfoMetrics::new());
    let mut app = App::new(sysmon);
    ratatui::run(|t| app.run(t))
}
