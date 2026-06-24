#[derive(Debug, Default)]
pub struct TimerState {
    pub time: u64,
    pub running: bool,
}

impl TimerState {
    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn resume(&mut self) {
        self.running = true;
    }

    pub fn tick(&mut self) {
        if self.running && self.time > 0 {
            self.time -= 1;
        }
    }
}