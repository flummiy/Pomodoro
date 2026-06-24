use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use crate::timer::TimerState;

pub fn draw(frame: &mut Frame, timer: &TimerState) {
    let area = frame.area();
    let title = Line::from(" Pomodoro Timer ").bold();

    let instructions = Line::from(vec![
        " Pause ".into(),
        "<Space>".blue().bold(),
        " Exit ".into(),
        "<Q>".blue().bold(),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered());

    let timer_text = Text::from(vec![Line::from(vec![
        "Time: ".into(),
        timer.time.to_string().yellow(),
    ])]);

    frame.render_widget(Paragraph::new(timer_text).centered().block(block), area);
}