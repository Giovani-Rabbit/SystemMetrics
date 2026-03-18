use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, read};
use ratatui::{DefaultTerminal, Frame};

use crate::system::traits::SystemMonitor;

pub struct App {
    pub sysmon: Box<dyn SystemMonitor>,
    mode: Mode,
}

#[derive(Debug, PartialEq)]
enum Mode {
    Running,
    Quit,
}

impl App {
    pub fn new(sysmon: Box<dyn SystemMonitor>) -> Self {
        Self {
            sysmon,
            mode: Mode::Running,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.sysmon.refresh();
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(1000))? {
            match read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.mode = Mode::Quit;
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }
}
