use crate::app::Pomodoro;
use crate::timer::TimerState;
use crossterm::event::{KeyCode, KeyEvent};
use figlet_rs::FIGlet;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
};

fn render_timer(minutes: u64, seconds: u64) -> String {
    let timer_font = FIGlet::big().unwrap();
    let text = format!("{:02}:{:02}", minutes, seconds);

    timer_font.convert(&text).unwrap().to_string()
}

pub fn draw(frame: &mut Frame, timer: &TimerState) {
    let area = frame.area();
    let title = Line::from(" Pomodoro Timer ").bold();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);

    let block = Block::bordered().title(title.centered());
    let timer_area = block.inner(layout[0]);
    frame.render_widget(block, layout[0]);

    let mut timer_lines = vec![Line::from(timer.phase.label()).bold(), Line::from("")];
    timer_lines.extend(
        render_timer(timer.minutes(), timer.seconds())
            .lines()
            .map(|line| Line::from(line.to_owned()).blue().bold()),
    );
    timer_lines.push(Line::from(vec![
        Span::raw("Pomodoros completed: "),
        Span::styled(
            timer.completed_pomodoros.to_string(),
            Style::new().yellow().add_modifier(Modifier::BOLD),
        ),
    ]));

    let timer_content_area = centered_vertically(timer_area, timer_lines.len() as u16);
    let timer_text = Text::from(timer_lines);

    let timer = Paragraph::new(timer_text).alignment(Alignment::Center);

    frame.render_widget(timer, timer_content_area);

    let help = Paragraph::new(Line::from(vec![
        Span::raw("Pause/Resume "),
        Span::styled("Space", Style::new().blue().add_modifier(Modifier::BOLD)),
        Span::raw("  Reset "),
        Span::styled("r", Style::new().blue().add_modifier(Modifier::BOLD)),
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
        KeyCode::Char(' ') => {
            if app.timer.running {
                app.timer.pause()
            } else {
                app.timer.resume()
            }
        }
        _ => {}
    }
}

fn centered_vertically(area: Rect, height: u16) -> Rect {
    if height >= area.height {
        return area;
    }

    let top_margin = (area.height - height) / 2;

    Rect {
        y: area.y + top_margin,
        height,
        ..area
    }
}
