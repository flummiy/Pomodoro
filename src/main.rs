pub mod app;
pub mod timer;
pub mod settings;
pub mod views;
pub mod event;

use app::Pomodoro;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| Pomodoro::default().run(terminal))
}
