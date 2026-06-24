use crate::app::{Pomodoro, View};
use crate::views;
use crossterm::event::{KeyCode, KeyEvent};

impl Pomodoro {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('t') => self.active_view = View::Timer,
            KeyCode::Char('s') => self.active_view = View::Settings,
            _ => match self.active_view {
                View::Timer => views::timer::handle_key(self, key_event),
                View::Settings => views::settings::handle_key(self, key_event),
            },
        }
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }
}
