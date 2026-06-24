use std::io;

use crate::settings::SettingsState;
use crate::timer::TimerState;
use crate::views;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct Pomodoro {
    pub should_exit: bool,
    pub active_view: View,

    pub timer: TimerState,
    pub settings: SettingsState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Timer,
    Settings,
}

impl Default for View {
    fn default() -> Self {
        View::Timer
    }
}

impl Pomodoro {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        match self.active_view {
            View::Timer => {
                views::timer::draw(frame, &self.timer);
            }
            View::Settings => {
                views::settings::draw(frame, &self.settings);
            }
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}
