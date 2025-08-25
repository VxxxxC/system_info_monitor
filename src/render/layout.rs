use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Borders, Paragraph},
};
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::prelude::*;

pub fn main_layout(frame: &mut Frame) {
    let layout = Layout::default().direction(Direction::Vertical).constraints(vec![Constraint::Percentage(30), Constraint::Percentage(20)]).split(frame.area());
    
    Block::bordered().fg(Color::Blue).title("System Info").render(layout[0], frame.buffer_mut());

    frame.render_widget(
        Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        layout[1]
    );

}

// Create custom function to center both horizontal and vertical
fn center_container(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);

    area
}
