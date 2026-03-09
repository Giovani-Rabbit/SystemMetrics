use std::io;

use sysmon::app::App;

fn main() -> io::Result<()> {
    ratatui::run(|t| App::new().run(t))
}
