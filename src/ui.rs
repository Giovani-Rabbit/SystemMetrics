use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols::block,
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
        Constraint::Length(20),
        Constraint::Length(1),
    ]);

    let [title_area, main_row, status] = inner.layout(&vertical);

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
    let outer_block = Block::bordered().title("CPU");
    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    let [top_area, bottom_area] =
        Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)])
            .areas(inner_area);
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(bottom_area);

    let lines = Text::from(vec![
        Line::from(vec![" ▪ ".into(), cpu.name.bold()]),
        Line::from(vec![
            "CPU usage: ".into(),
            format!(
                "{:.2}% core amount - {}",
                cpu.usage_percent,
                cpu.cores.len()
            )
            .yellow()
            .into(),
        ]),
    ]);
    frame.render_widget(Paragraph::new(lines), top_area);

    let cores_lines: Vec<Line> = cpu
        .cores
        .iter()
        .enumerate()
        .map(|(i, c)| Line::from(format!("core {} - {:.2}%", i + 1, c.usage)))
        .collect();

    let mid = cores_lines.len() / 2;
    let (left_cores, right_cores) = cores_lines.split_at(mid);

    frame.render_widget(Paragraph::new(Text::from(left_cores.to_vec())), left_area);
    frame.render_widget(Paragraph::new(Text::from(right_cores.to_vec())), right_area);
}

fn memory_block(frame: &mut Frame, area: Rect, memory: MemoryInfo) {
    let lines = vec![
        Line::from(vec![
            "RAM".into(),
            format!("{}/{} G", bytes_to_g(memory.used), bytes_to_g(memory.total)).yellow(),
        ]),
        Line::from(vec![
            "Usage: ".into(),
            format!("{}/{} G", bytes_to_g(memory.used), bytes_to_g(memory.total)).yellow(),
        ]),
        Line::from(vec!["\n".into()]),
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
