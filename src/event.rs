use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{Pomodoro, View};
use crate::views;

impl Pomodoro {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(' ') => self.timer.running = false,
            KeyCode::Char('t') => self.active_view = View::Timer,
            KeyCode::Char('s') => self.active_view = View::Settings,
            _ => match self.active_view {
                View::Timer => {},
                View::Settings => views::settings::handle_key(self, key_event),
            }
        }
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }
}