use crate::system::os_info;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::prelude::*;
use ratatui::widgets::BorderType;
use ratatui::{
    Frame,
    widgets::{Block, Borders},
};

pub fn entry_layout(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Percentage(2), Constraint::Percentage(30)]);
    let horizontal = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);

    let [top, main] = frame.area().layout(&vertical);
    let [first] = main
        // INFO: If without inner margin, the paragraph will attach with the borderline
        .inner(Margin {
            horizontal: 1,
            vertical: 1,
        })
        .layout(&horizontal);

    let center_rect = center_container(first, Constraint::Fill(1), Constraint::Fill(1));

    let title = Span::from("Top Title").bold().into_centered_line();
    frame.render_widget(title, top);

    let os_info_block = Block::default()
        .fg(Color::Blue)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    frame.render_widget(&os_info_block, main);
    let mut title: Vec<String> = Vec::new();
    let mut info: Vec<String> = Vec::new();

    let info_list = os_info::system_info();
    for x in info_list.iter() {
        title.push(x.title.to_string());
        info.push(x.info.to_string());
    }

    let length = info_list.clone().len();
    let inner_title_layout =
        Layout::vertical([Constraint::Length(length as u16); 6]).split(center_rect);
    let inner_info_layout =
        Layout::vertical([Constraint::Length(length as u16); 6]).split(center_rect);

    // TODO: Need to correct the constraint , and not using Length for the for-loop index [range]
    // let info_layout = Layout::vertical([Constraint::Length(6); 6])
    //     .spacing(1)
    //     .split(center);
    for range in 0..info_list.len() {
        if range <= info_list.len() {
            let title = Span::from(title[range].to_string());
            let info = Span::from(info[range].to_string());
            frame.render_widget(Text::from(title).left_aligned(), inner_title_layout[range]);
            frame.render_widget(Text::from(info).right_aligned(), inner_info_layout[range]);
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
