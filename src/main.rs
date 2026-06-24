pub mod app;
pub mod event;
pub mod settings;
pub mod timer;
pub mod views;

use app::Pomodoro;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| Pomodoro::default().run(terminal))
}
