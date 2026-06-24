#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerPhase {
    Work,
    ShortBreak,
    LongBreak,
}

impl TimerPhase {
    pub fn label(self) -> &'static str {
        match self {
            Self::Work => "Work",
            Self::ShortBreak => "Short Break",
            Self::LongBreak => "Long Break",
        }
    }
}

#[derive(Debug)]
pub struct TimerState {
    pub time: u64,
    pub running: bool,
    pub started: bool,
    pub phase: TimerPhase,
    pub completed_pomodoros: u64,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            time: 0,
            running: false,
            started: false,
            phase: TimerPhase::Work,
            completed_pomodoros: 0,
        }
    }
}
impl TimerState {
    pub fn from_minutes(minutes: u64) -> Self {
        Self {
            time: minutes * 60,
            ..Default::default()
        }
    }

    pub fn set_duration_if_not_started(
        &mut self,
        work_minutes: u64,
        short_break_minutes: u64,
        long_break_minutes: u64,
    ) {
        if !self.started {
            self.set_minutes(match self.phase {
                TimerPhase::Work => work_minutes,
                TimerPhase::ShortBreak => short_break_minutes,
                TimerPhase::LongBreak => long_break_minutes,
            });
        }
    }

    pub fn reset(&mut self, minutes: u64) {
        self.set_minutes(minutes);
        self.running = false;
        self.started = false;
        self.phase = TimerPhase::Work;
        self.completed_pomodoros = 0;
    }

    fn set_minutes(&mut self, minutes: u64) {
        self.time = minutes * 60;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn resume(&mut self) {
        self.started = true;
        self.running = true;
    }

    pub fn tick(&mut self, seconds: u64) -> bool {
        if !self.running || seconds == 0 {
            return false;
        }

        self.time = self.time.saturating_sub(seconds);

        if self.time == 0 {
            self.running = false;
            return true;
        }

        false
    }

    pub fn advance_after_completion(
        &mut self,
        work_minutes: u64,
        short_break_minutes: u64,
        long_break_minutes: u64,
    ) {
        match self.phase {
            TimerPhase::Work => {
                self.completed_pomodoros += 1;
                self.phase = if self.completed_pomodoros % 4 == 0 {
                    TimerPhase::LongBreak
                } else {
                    TimerPhase::ShortBreak
                };
            }
            TimerPhase::ShortBreak | TimerPhase::LongBreak => {
                self.phase = TimerPhase::Work;
            }
        }

        self.started = false;
        self.running = false;
        self.set_duration_if_not_started(work_minutes, short_break_minutes, long_break_minutes);
    }

    pub fn minutes(&self) -> u64 {
        self.time / 60
    }

    pub fn seconds(&self) -> u64 {
        self.time % 60
    }
}
