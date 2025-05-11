use color_eyre::eyre::Result;
use ratatui::{
    text::Text,
    widgets::{Block, Paragraph},
};

pub fn draw_title(frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    frame.render_widget(Paragraph::new(create_small_logo_text()).block(Block::bordered()).centered(), area);
    Ok(())
}

fn _create_logo_lext() -> Text<'static> {
    Text::from(
        "████████╗ █████╗ ███╗  ██╗██╗  ██╗██╗
╚══██╔══╝██╔══██╗████╗ ██║██║ ██╔╝██║
   ██║   ███████║██╔██╗██║█████═╝ ██║
   ██║   ██╔══██║██║╚████║██╔═██╗ ██║
   ██║   ██║  ██║██║ ╚███║██║ ╚██╗██║
   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚══╝╚═╝  ╚═╝╚═╝",
    )
}

fn create_small_logo_text() -> Text<'static> {
    Text::from("▗▄▄▄▖▗▄▖ ▗▖  ▗▖▗▖ ▗▖▗▄▄▄▖
  █ ▐▌ ▐▌▐▛▚▖▐▌▐▌▗▞▘  █  
  █ ▐▛▀▜▌▐▌ ▝▜▌▐▛▚▖   █  
  █ ▐▌ ▐▌▐▌  ▐▌▐▌ ▐▌▗▄█▄▖"
    )
}
