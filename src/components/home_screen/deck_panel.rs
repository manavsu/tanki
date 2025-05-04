use crate::{
    action::Action,
    models::{
        deck::Deck,
        note::{Note, NoteType},
    },
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{self, Rect},
    text::Text,
    widgets::{Block, Cell, Paragraph, Row, Table, Wrap},
};

use super::input_state::InputState;

#[derive(Clone)]
pub struct InsertNoteState {
    front: InputState,
    back: InputState,
    focused_front: bool,
    pub completed: bool,
}

impl InsertNoteState {
    pub fn new() -> Self {
        Self { front: InputState::new(), back: InputState::new(), focused_front: true, completed: false }
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

    let (front_text, back_text) = if insert_state.focused_front {
        (insert_state.front.input_display(), insert_state.back.get_input())
    } else {
        (insert_state.front.get_input(), insert_state.back.input_display())
    };

    if insert_state.focused_front {
        frame.set_cursor_position(insert_state.front.calculate_cursor_coordinates_wrapped(sections[0]));
    } else {
        frame.set_cursor_position(insert_state.back.calculate_cursor_coordinates_wrapped(sections[1]));
    }

    let front = Paragraph::new(Text::from(front_text))
        .block(Block::bordered().title("Front"))
        .wrap(Wrap { trim: true })
        .style(if insert_state.focused_front { prelude::Style::default().fg(prelude::Color::Yellow) } else { prelude::Style::default() });
    let back = Paragraph::new(Text::from(back_text))
        .block(Block::bordered().title("Back"))
        .wrap(Wrap { trim: true })
        .style(if !insert_state.focused_front { prelude::Style::default().fg(prelude::Color::Yellow) } else { prelude::Style::default() });
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
        Action::CtrlSpace => {
            if state.focused_front {
                state.front.push('\n');
            } else {
                state.back.push('\n');
            }
        }
        Action::Enter => {
            if state.focused_front && state.back.get_input().is_empty() {
                state.focused_front = false;
            } else {
                deck.add_note(Note::new(state.front.get_input(), state.back.get_input(), NoteType::Basic));
                state.completed = true;
            }
        }
        Action::Tab => {
            state.focused_front = !state.focused_front;
        }
        Action::Esc => {
            state.completed = true;
        }
        Action::Right => {
            if state.focused_front {
                state.front.cursor_right();
            } else {
                state.back.cursor_right();
            }
        }
        Action::Left => {
            if state.focused_front {
                state.front.cursor_left();
            } else {
                state.back.cursor_left();
            }
        }
        _ => {}
    };
    state
}
