use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph},
};

use crate::system::{
    sysinfo_impl::bytes_to_g,
    traits::{CpuInfo, MemoryInfo, SystemMonitor},
};

pub fn render(frame: &mut Frame, sys: &Box<dyn SystemMonitor>) {
    let constrained = centered_rect(120, 40, frame.area());
    let outer = Block::bordered();
    let inner = outer.inner(constrained);
    frame.render_widget(outer, constrained);

    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(10),
        Constraint::Length(8),
        Constraint::Length(1),
    ]);

    let [title_area, main_row, bottom_row, status] = inner.layout(&vertical);

    let [cpu_area, mem_area] =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
            .areas(main_row);

    let title = Paragraph::new(Line::from(vec![
        "◆  ".into(),
        "SYSMON".bold().cyan(),
        "  ◆".into(),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(title, title_area);

    cpu_block(frame, cpu_area, sys.cpu_info());
    memory_block(frame, mem_area, sys.memory_info());
}

fn centered_rect(max_width: u16, max_height: u16, area: Rect) -> Rect {
    let width = max_width.min(area.width);
    let height = max_height.min(area.height);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    Rect {
        x,
        y,
        width,
        height,
    }
}

fn cpu_block(frame: &mut Frame, area: Rect, cpu: CpuInfo) {
    let title = Text::from(vec![Line::from(vec!["CPU".into()])]);
    frame.render_widget(title, area);

    let text = Text::from(vec![
        Line::from(format!(" ▪ CPU - {}", cpu.name.bold())),
        Line::from(vec![
            "CPU usage: ".into(),
            format!("{:.2}%", cpu.usage_percent).yellow(),
        ]),
    ]);

    let block = Block::bordered();
    let p = Paragraph::new(text).block(block);

    frame.render_widget(p, area);
}

fn memory_block(frame: &mut Frame, area: Rect, memory: MemoryInfo) {
    let lines = vec![
        Line::from(vec![
            "Memory: ".into(),
            format!("{}/{} G", bytes_to_g(memory.used), bytes_to_g(memory.total)).yellow(),
        ]),
        Line::from(vec![
            "Swap: ".into(),
            format!(
                "{}/{} G",
                bytes_to_g(memory.used_swap),
                bytes_to_g(memory.total_swap)
            )
            .yellow(),
            "  Free: ".into(),
            format!("{} G", bytes_to_g(memory.free_swap)).green(),
        ]),
    ];

    let block = Block::bordered();
    let p = Paragraph::new(Text::from(lines)).block(block);
    frame.render_widget(p, area);
}
