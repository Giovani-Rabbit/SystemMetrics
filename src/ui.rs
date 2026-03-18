use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![" <Q> To quit ".blue().bold()]);
        let padding = Padding::symmetric(1, 1);
        let block = Block::new()
            .padding(padding)
            .title_bottom(instructions.left_aligned());

        let cpu = &self.sysmon.cpu_info().usage_percent;
        let cpu_usage_text = Text::from(vec![Line::from(vec![
            "CPU usage: ".into(),
            format!("{:.2}%", cpu).yellow(),
        ])]);

        Paragraph::new(cpu_usage_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
