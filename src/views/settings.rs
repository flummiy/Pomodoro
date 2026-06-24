use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::Pomodoro;
use crate::settings::SettingsState;

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

    let block = Block::default()
        .title(" Settings ")
        .borders(Borders::ALL);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(area);

    let items = vec![
        setting_item(
            settings,
            0,
            "Work duration",
            settings.work_minutes,
        ),
        setting_item(
            settings,
            1,
            "Short break",
            settings.short_break_minutes,
        ),
        setting_item(
            settings,
            2,
            "Long break",
            settings.long_break_minutes,
        ),
    ];

    let list = List::new(items)
        .block(block)
        .highlight_symbol("> ");

    frame.render_widget(list, layout[0]);

    let help = Paragraph::new(Line::from(vec![
        Span::raw("Move "),
        Span::styled("↑/↓ or j/k", Style::new().blue().add_modifier(Modifier::BOLD)),
        Span::raw("  Change "),
        Span::styled("←/→ or h/l", Style::new().blue().add_modifier(Modifier::BOLD)),
        Span::raw("  Timer "),
        Span::styled("t", Style::new().blue().add_modifier(Modifier::BOLD)),
        Span::raw("  Quit "),
        Span::styled("q", Style::new().blue().add_modifier(Modifier::BOLD)),
    ]))
        .centered()
        .block(Block::bordered());

    frame.render_widget(help, layout[1]);
}

fn setting_item(settings: &SettingsState, index: usize, label: &str, value: u64) -> ListItem<'static> {
    let text = format!("{label}: {value} min");

    if settings.selected == index {
        ListItem::new(format!("> {text}").yellow().bold())
    } else {
        ListItem::new(text)
    }
}