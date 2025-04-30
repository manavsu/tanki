use color_eyre::eyre::Result;
use ratatui::{
    text::Text,
    widgets::{Block, Paragraph},
};

pub fn draw_title(frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    frame.render_widget(Paragraph::new(create_logo_lext()).block(Block::bordered()).centered(), area);
    Ok(())
}

fn create_logo_lext() -> Text<'static> {
    Text::from(
        "████████╗ █████╗ ███╗  ██╗██╗  ██╗██╗
╚══██╔══╝██╔══██╗████╗ ██║██║ ██╔╝██║
   ██║   ███████║██╔██╗██║█████═╝ ██║
   ██║   ██╔══██║██║╚████║██╔═██╗ ██║
   ██║   ██║  ██║██║ ╚███║██║ ╚██╗██║
   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚══╝╚═╝  ╚═╝╚═╝",
    )
}
