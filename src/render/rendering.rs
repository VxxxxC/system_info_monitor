use ratatui::{
    DefaultTerminal, Frame,
    layout::{self, Layout},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_widget(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        layout::Rect::default(),
    );
    frame.render_widget("Hello World", frame.area());
}
