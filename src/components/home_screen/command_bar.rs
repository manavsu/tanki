use ratatui::{
    layout::Rect,
    text::{self, Line, Span, Text},
    widgets::{Block, Paragraph},
};

use super::Mode;

pub fn draw_command_bar(frame: &mut ratatui::Frame, area: Rect, mode: Mode) {
    let commands: Vec<&str> = match mode {
        Mode::Normal(Some(_)) => {
            vec![
                "<Space> expand/collapse",
                "<Up> : up",
                "<Down> : down",
                "<n> : +note",
                "<s> : +subdeck",
                "<a> : +deck",
                "<d> : delete",
                "<q> : quit",
            ]
        }
        Mode::Normal(None) => {
            vec!["<Up> : up", "<Down> : down", "<a> : +deck", "<q> : quit"]
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
    frame.render_widget(Paragraph::new(line).centered().block(Block::default().title("[Commands]").borders(ratatui::widgets::Borders::ALL)), area);
}
