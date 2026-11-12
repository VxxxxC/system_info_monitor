use crate::render;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;

pub fn run_widget(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(render::layout::entry_layout)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}
