use crate::system::os_info;
use crate::system::os_info::*;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::prelude::*;
use ratatui::widgets::{BorderType, Padding, Wrap};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Borders, Paragraph},
};

pub fn main_layout(frame: &mut Frame) {
    let container = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(20)])
        .split(frame.area());

    let os_info_outer_block = Block::default()
        .fg(Color::Blue)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .title(Span::from("System Info").bold().into_centered_line());

    let inner = &os_info_outer_block.inner(container[0]);
    let os_info_inner_block = Block::default().padding(Padding::symmetric(10, 20));

    frame.render_widget(os_info_outer_block, container[0]);
    frame.render_widget(os_info_inner_block, *inner);

    let mut info_list = os_info::system_info();
    let mut title: Vec<String> = Vec::new();
    let mut info: Vec<String> = Vec::new();

    for x in info_list.iter_mut() {
        title.push(x.title.clone());
        info.push(x.info.clone());
    }

    const LENGTH: usize = 6;

    let title_layout = Layout::vertical([Constraint::Length(5); LENGTH]).split(*inner);
    let info_layout = Layout::vertical([Constraint::Length(5); LENGTH]).split(*inner);

    for range in 0..info_list.len() {
        if range <= info_list.len() {
            let title = Line::from(title[range].clone());
            let info = Line::from(info[range].clone());
            frame.render_widget(
                Paragraph::new(title).alignment(Alignment::Left),
                title_layout[range],
            );
            frame.render_widget(
                Paragraph::new(info).alignment(Alignment::Right),
                info_layout[range],
            );
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
