use crate::app::Pomodoro;
use crate::settings::SettingsState;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

pub fn handle_key(app: &mut Pomodoro, key: KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => app.settings.previous(),
        KeyCode::Down | KeyCode::Char('j') => app.settings.next(),
        KeyCode::Left | KeyCode::Char('h') => app.settings.decrease_selected(),
        KeyCode::Right | KeyCode::Char('l') => app.settings.increase_selected(),
        _ => {}
    }
}

pub fn draw(frame: &mut Frame, settings: &SettingsState) {
    let area = frame.area();

    let block = Block::default().title(" Settings ").borders(Borders::ALL);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);

    let settings_text = Text::from(vec![
        setting_line(settings, 0, "Work duration", settings.work_minutes),
        setting_line(settings, 1, "Short break", settings.short_break_minutes),
        setting_line(settings, 2, "Long break", settings.long_break_minutes),
    ]);

    frame.render_widget(
        Paragraph::new(settings_text).centered().block(block),
        layout[0],
    );

    let help = Paragraph::new(Line::from(vec![
        Span::raw("Move "),
        Span::styled(
            "↑/↓ or j/k",
            Style::new().blue().add_modifier(Modifier::BOLD),
        ),
        Span::raw("  Change "),
        Span::styled(
            "←/→ or h/l",
            Style::new().blue().add_modifier(Modifier::BOLD),
        ),
        Span::raw("  Timer "),
        Span::styled("t", Style::new().blue().add_modifier(Modifier::BOLD)),
        Span::raw("  Quit "),
        Span::styled("q", Style::new().blue().add_modifier(Modifier::BOLD)),
    ]))
    .centered()
    .block(Block::bordered());

    frame.render_widget(help, layout[1]);
}

fn setting_line(settings: &SettingsState, index: usize, label: &str, value: u64) -> Line<'static> {
    let text = format!("{label}: {value} min");

    if settings.selected == index {
        Line::from(format!("> {text}").yellow().bold())
    } else {
        Line::from(text)
    }
}
