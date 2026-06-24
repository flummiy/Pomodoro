use std::{
    io,
    time::{Duration, Instant},
};

use crate::settings::SettingsState;
use crate::timer::TimerState;
use crate::views;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug)]
pub struct Pomodoro {
    pub should_exit: bool,
    pub active_view: View,

    pub timer: TimerState,
    pub settings: SettingsState,
}

impl Default for Pomodoro {
    fn default() -> Self {
        let settings = SettingsState::default();
        let timer = TimerState::from_minutes(settings.work_minutes);

        Self {
            should_exit: false,
            active_view: View::default(),
            timer,
            settings,
        }
    }
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
        let mut last_tick = Instant::now();

        while !self.should_exit {
            self.update_timer(&mut last_tick);
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
        if !event::poll(Duration::from_millis(100))? {
            return Ok(());
        }

        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn update_timer(&mut self, last_tick: &mut Instant) {
        if !self.timer.running {
            *last_tick = Instant::now();
            return;
        }

        let elapsed_seconds = last_tick.elapsed().as_secs();
        let timer_finished = self.timer.tick(elapsed_seconds);

        if elapsed_seconds > 0 {
            *last_tick += Duration::from_secs(elapsed_seconds);
        }

        if timer_finished {
            self.timer.advance_after_completion(
                self.settings.work_minutes,
                self.settings.short_break_minutes,
                self.settings.long_break_minutes,
            );
        }
    }
}
