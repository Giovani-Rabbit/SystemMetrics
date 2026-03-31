use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

use crate::system::{
    sysinfo_impl::bytes_to_g,
    traits::{CpuInfo, MemoryInfo, SystemMonitor},
};

pub fn render(frame: &mut Frame, sys: &Box<dyn SystemMonitor>) {
    let cpu = sys.cpu_info();
    let mem = sys.memory_info();

    let cpu_height = 2 + (cpu.cores.len() / 2) as u16;
    let mem_height = 4u16;

    let main_height = cpu_height.max(mem_height);
    let total_height = 1 + main_height + 1 + 2;

    let constrained = centered_rect(120, total_height, frame.area());
    let outer = Block::bordered();
    let inner = outer.inner(constrained);
    frame.render_widget(outer, constrained);

    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(main_height),
        Constraint::Length(1),
    ]);
    let [title_area, main_row, status_area] = inner.layout(&vertical);

    let [cpu_area, mem_area] =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
            .areas(main_row);

    let title = Paragraph::new(Line::from(vec![
        "◆  ".into(),
        "SYSMON".bold().cyan(),
        "  ◆".into(),
    ]))
    .alignment(Alignment::Center);

    let status = Paragraph::new(Line::from(vec![
        " press ".into(),
        "<q>".bold().red(),
        " to quit".into(),
    ]));
    frame.render_widget(status, status_area);

    frame.render_widget(title, title_area);
    cpu_block(frame, cpu_area, cpu);
    memory_block(frame, mem_area, mem);
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

    let header_lines = 2u16;
    let cores_lines: Vec<Line> = cpu
        .cores
        .iter()
        .enumerate()
        .map(|(i, c)| Line::from(format!("core {} - {:.2}%", i + 1, c.usage)))
        .collect();

    let mid = cores_lines.len() / 2;
    let (left_cores, right_cores) = cores_lines.split_at(mid);

    let cores_height = mid as u16;

    let [top_area, bottom_area] = Layout::vertical([
        Constraint::Length(header_lines),
        Constraint::Length(cores_height),
    ])
    .areas(inner_area);

    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(bottom_area);

    let lines = Text::from(vec![
        Line::from(vec![" ▪ ".into(), cpu.name.bold()]),
        Line::from(vec![
            "CPU usage: ".into(),
            format!("{:.2}%", cpu.usage_percent).yellow().into(),
        ]),
    ]);

    frame.render_widget(Paragraph::new(lines), top_area);
    frame.render_widget(Paragraph::new(Text::from(left_cores.to_vec())), left_area);
    frame.render_widget(Paragraph::new(Text::from(right_cores.to_vec())), right_area);
}

fn memory_block(frame: &mut Frame, area: Rect, memory: MemoryInfo) {
    let lines = vec![
        Line::from(vec![
            "Usage: ".into(),
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

    let block = Block::bordered().title(" RAM ");
    let p = Paragraph::new(Text::from(lines)).block(block);
    frame.render_widget(p, area);
}
