use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use super::Mode;

pub fn draw_command_bar(frame: &mut ratatui::Frame, area: Rect, mode: Mode) {
    let commands: Vec<&str> = match mode.clone() {
        Mode::Normal(Some(_)) => {
            vec![
                "<Space> : expand/collapse",
                "<CR> : practice",
                "<Up> : up",
                "<Down> : down",
                "<n> : +note",
                "<s> : +subdeck",
                "<a> : +deck",
                "<D> : delete",
                "<q> : quit",
                "<i> : import",
            ]
        }
        Mode::Normal(None) => {
            vec!["<Up> : up", "<Down> : down", "<a> : +deck", "<q> : quit", "<i> : import"]
        }
        Mode::InsertDeck(_, _) => vec!["<Esc> : cancel", "CR : submit"],
        Mode::InsertNote(insert_note_state) => match insert_note_state.focused_front {
            true => vec!["<C-Space> : newline", "<Esc> : cancel", "<CR> : back"],
            false => vec!["<C-Space> : newline", "<Esc> : cancel", "<CR> : submit", "<tab> : front"],
        },
    }
    .into_iter()
    .flat_map(|c| [c, "   "])
    .collect();
    let line = Line::from(commands.into_iter().map(Span::from).collect::<Vec<_>>());
    frame.render_widget(Paragraph::new(line).centered().block(Block::default().title("[commands]").borders(ratatui::widgets::Borders::ALL)), area);
}
