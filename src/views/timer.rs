use crate::app::Pomodoro;
use crate::timer::TimerState;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
};

pub fn draw(frame: &mut Frame, timer: &TimerState) {
    let area = frame.area();
    let title = Line::from(" Pomodoro Timer ").bold();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);

    let block = Block::bordered().title(title.centered());

    let timer_text = Text::from(vec![Line::from(vec![
        "Time: ".into(),
        timer.time.to_string().yellow(),
    ])]);

    frame.render_widget(
        Paragraph::new(timer_text).centered().block(block),
        layout[0],
    );

    let help = Paragraph::new(Line::from(vec![
        Span::raw("Pause "),
        Span::styled("Space", Style::new().blue().add_modifier(Modifier::BOLD)),
        Span::raw("  Settings "),
        Span::styled("s", Style::new().blue().add_modifier(Modifier::BOLD)),
        Span::raw("  Quit "),
        Span::styled("q", Style::new().blue().add_modifier(Modifier::BOLD)),
    ]))
    .centered()
    .block(Block::bordered());

    frame.render_widget(help, layout[1]);
}

pub fn handle_key(app: &mut Pomodoro, key: KeyEvent) {
    match key.code {
        KeyCode::Char(' ') => app.timer.running = false,
        _ => {}
    }
}
