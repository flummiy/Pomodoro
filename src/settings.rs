#[derive(Debug)]
pub struct SettingsState {
    pub selected: usize,
    pub work_minutes: u64,
    pub short_break_minutes: u64,
    pub long_break_minutes: u64,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            selected: 0,
            work_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
        }
    }
}

impl SettingsState {
    pub const ITEM_COUNT: usize = 3;

    pub fn selected_label(&self) -> &'static str {
        match self.selected {
            0 => "Work duration",
            1 => "Short break",
            2 => "Long break",
            _ => unreachable!(),
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % Self::ITEM_COUNT;
    }

    pub fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = Self::ITEM_COUNT - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn increase_selected(&mut self) {
        match self.selected {
            0 => self.work_minutes += 1,
            1 => self.short_break_minutes += 1,
            2 => self.long_break_minutes += 1,
            _ => {}
        }
    }

    pub fn decrease_selected(&mut self) {
        match self.selected {
            0 => self.work_minutes = self.work_minutes.saturating_sub(1),
            1 => self.short_break_minutes = self.short_break_minutes.saturating_sub(1),
            2 => self.long_break_minutes = self.long_break_minutes.saturating_sub(1),
            _ => {}
        }
    }
}
