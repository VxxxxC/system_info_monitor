use crate::system::os_info;
use crate::system::os_info::*;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::prelude::*;
use ratatui::widgets::BorderType;
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Borders, Paragraph},
};

pub fn main_layout(frame: &mut Frame) {
    let container = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(20)])
        .split(frame.area());

    let os_info_block = Block::bordered()
        .fg(Color::Blue)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .title(Line::from("System Info").centered());

    let inner_block = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(container[0]);

    let center = center_container(frame.area(), Constraint::Length(1), Constraint::Length(1));

    let mut info_list = os_info::system_info();
    let mut title: Vec<String> = Vec::new();
    let mut info: Vec<String> = Vec::new();

    for x in info_list.iter_mut() {
        title.push(x.title.clone());
        info.push(x.info.clone());
    }

    frame.render_widget(os_info_block, container[0]);

    let title_layout = Layout::vertical([Constraint::Length(20)]).split(inner_block[0]);
    let info_layout = Layout::vertical([Constraint::Length(20)]).split(inner_block[1]);

    for range in 0..info_list.len() {
        if range <= info_list.len() {
            let title = Text::from(Line::from(title[range].clone()));
            let info = Text::from(Line::from(info[range].clone()));
            frame.render_widget( Paragraph::new(title).alignment(Alignment::Left), title_layout[0]);
            frame.render_widget( Paragraph::new(info).alignment(Alignment::Right), info_layout[0]);
        }
    }

}

// Create custom function to center both horizontal and vertical
fn center_container(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);

    area
}
