use ratatui::DefaultTerminal;
use crossterm::event::{self, Event};
use crate::render;

pub fn run_widget(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(render::layout::main_layout)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}