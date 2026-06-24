pub mod app;

use app::Pomodoro;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| Pomodoro::default().run(terminal))
}
