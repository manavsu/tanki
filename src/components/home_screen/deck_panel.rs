use color_eyre::config::PanicReport;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{self, Rect},
    style::Stylize,
    text::Text,
    widgets::{self, Block, Cell, Paragraph, Row, Table},
};
use tracing::info;

use crate::{
    action::Action,
    models::{
        deck::Deck,
        note::{Note, NoteType},
    },
};

#[derive(Clone)]
pub struct InsertNoteState {
    front: String,
    back: String,
    focused_front: bool,
    pub completed: bool,
}

impl InsertNoteState {
    pub fn new() -> Self {
        Self { front: String::new(), back: String::new(), focused_front: true, completed: false }
    }
}

pub fn draw_deck_panel(
    frame: &mut ratatui::Frame,
    area: Rect,
    deck: Option<&Deck>,
    insert_state: Option<InsertNoteState>,
) -> color_eyre::eyre::Result<()> {
    match insert_state {
        None => {
            draw_deck_panel_normal_view(frame, area, deck);
        }
        Some(insert_sate) => match deck {
            None => return Err(color_eyre::eyre::eyre!("Error: Cannot insert note into a non-existent deck.")),
            Some(deck) => draw_deck_panel_insert_view(frame, area, deck, insert_sate),
        },
    }
    Ok(())
}

pub fn draw_deck_panel_insert_view(frame: &mut ratatui::Frame, area: Rect, deck: &Deck, insert_state: InsertNoteState) {
    let sections =
        Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).margin(2).split(area);

    let front = Paragraph::new(Text::from(insert_state.front)).block(Block::bordered().title("Front")).style(if insert_state.focused_front {
        prelude::Style::default().fg(prelude::Color::Yellow)
    } else {
        prelude::Style::default()
    });
    let back = Paragraph::new(Text::from(insert_state.back)).block(Block::bordered().title("Back")).style(if !insert_state.focused_front {
        prelude::Style::default().fg(prelude::Color::Yellow)
    } else {
        prelude::Style::default()
    });
    frame.render_widget(Block::bordered().title(format_title(&deck.qualified_name())), area);
    frame.render_widget(front, sections[0]);
    frame.render_widget(back, sections[1]);
}

fn format_title(title: &str) -> String {
    "[".to_string() + title + "]"
}

pub fn draw_deck_panel_normal_view(frame: &mut ratatui::Frame, area: Rect, deck: Option<&Deck>) {
    match deck {
        None => {
            frame.render_widget(Paragraph::new(Text::from("-----")).block(Block::bordered().title(format_title("*"))), area);
        }
        Some(deck) if deck.get_notes().is_empty() => {
            frame.render_widget(Paragraph::new(Text::from("-----")).block(Block::bordered().title(format_title(&deck.qualified_name()))), area);
        }
        Some(deck) => {
            let rows: Vec<Row> = deck
                .get_notes()
                .iter()
                .map(|note| Row::new([Cell::from(Text::from(note.front.clone())), Cell::from(Text::from(note.back.clone()))]))
                .collect();

            let widths = Constraint::from_percentages([50, 50]);
            let table = Table::new(rows, widths);

            frame.render_widget(table.block(Block::bordered().title(format_title(&deck.qualified_name()))), area);
        }
    }
}

pub fn update_deck_panel_note_insert(action: Action, mut state: InsertNoteState, deck: &mut Deck) -> InsertNoteState {
    match action {
        Action::Char(c) => {
            if state.focused_front {
                state.front.push(c);
            } else {
                state.back.push(c);
            }
        }
        Action::Space => {
            if state.focused_front {
                state.front.push(' ');
            } else {
                state.back.push(' ');
            }
        }
        Action::Backspace => {
            if state.focused_front {
                state.front.pop();
            } else {
                state.back.pop();
            }
        }
        Action::Enter => {
            deck.add_note(Note::new(state.front.clone(), state.back.clone(), NoteType::Basic));
            state.completed = true;
        }
        Action::Tab => {
            state.focused_front = !state.focused_front;
        }
        Action::Esc => {
            state.completed = true;
        }
        Action::Right => {}
        Action::Left => {}
        _ => {}
    };
    state
}
