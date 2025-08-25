use color_eyre::Result;

mod render;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = render::widget::run_widget(terminal);

    ratatui::restore();

    result
}
