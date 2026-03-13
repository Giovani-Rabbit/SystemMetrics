use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, read};
use ratatui::{DefaultTerminal, Frame};

use crate::system::traits::SystemMonitor;

pub struct App {
    pub sysmon: Arc<Mutex<Box<dyn SystemMonitor + Send>>>,
    exit: bool,
}

impl App {
    pub fn new(sysmon: Box<dyn SystemMonitor + Send>) -> Self {
        Self {
            sysmon: Arc::new(Mutex::new(sysmon)),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.metrics_thread();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn metrics_thread(&self) {
        let sysmon_thread = Arc::clone(&self.sysmon);

        thread::spawn(move || {
            loop {
                sysmon_thread.lock().unwrap().refresh();
                thread::sleep(Duration::from_secs(1));
            }
        });
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
        self.exit = true;
    }
}
