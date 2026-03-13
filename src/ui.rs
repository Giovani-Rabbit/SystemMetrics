use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

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
