use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph},
};

use crate::system::{
    sysinfo_impl::bytes_to_g,
    traits::{CpuInfo, MemoryInfo, SystemMonitor},
};

pub fn render(frame: &mut Frame, sys: &Box<dyn SystemMonitor>) {
    let instructions = Line::from(vec![" <Q> To quit ".blue().bold()]);
    let padding = Padding::proportional(2);
    let area = Block::new().padding(padding).title_bottom(instructions);

    let content_area = area.inner(frame.area());
    frame.render_widget(area, frame.area());

    let horizontal = Layout::vertical([Constraint::Percentage(50); 2]).spacing(1);
    let [top, bottom] = content_area.layout(&horizontal);

    cpu_block(frame, top, sys.cpu_info());
    memory_block(frame, bottom, sys.memory_info());
}

fn cpu_block(frame: &mut Frame, area: Rect, cpu: CpuInfo) {
    let cpu_usage_text = Text::from(vec![Line::from(vec![
        "CPU usage: ".into(),
        format!("{:.2}%", cpu.usage_percent).yellow(),
    ])]);

    let block = Block::new();
    let p = Paragraph::new(cpu_usage_text).block(block);

    frame.render_widget(p, area);
}

fn memory_block(frame: &mut Frame, area: Rect, memory: MemoryInfo) {
    let lines = vec![
        Line::from(vec![
            "Memory: ".into(),
            format!("{}/{}G", bytes_to_g(memory.used), bytes_to_g(memory.total)).yellow(),
        ]),
        Line::from(vec![
            "Swap: ".into(),
            format!(
                "{}/{}G",
                bytes_to_g(memory.used_swap),
                bytes_to_g(memory.total_swap)
            )
            .yellow(),
            "  Free: ".into(),
            format!("{}G", bytes_to_g(memory.free_swap)).green(),
        ]),
    ];

    let block = Block::new();
    let p = Paragraph::new(Text::from(lines)).block(block);
    frame.render_widget(p, area);
}
