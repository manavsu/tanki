use std::cmp::min;

use color_eyre::config::PanicReport;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{self, Rect},
    style::Stylize,
    text::Text,
    widgets::{self, Block, Cell, Paragraph, Row, Table, Wrap},
};
use textwrap::wrap;
use tracing::info;

use crate::{
    action::Action,
    models::{
        deck::Deck,
        note::{Note, NoteType},
    },
};

use super::{CURSOR, INPUT_PROMPT};

#[derive(Clone)]
pub struct InputState {
    input: String,
    curor_position: usize,
}

impl InputState {
    pub fn new() -> Self {
        Self { input: String::new(), curor_position: 0 }
    }

    pub fn get_input(&self) -> String {
        self.input.clone()
    }

    pub fn push(&mut self, c: char) {
        // self.input = self.input[..self.curor_position].to_string() + &c.to_string() + &self.input[self.curor_position..];
        self.input.insert(self.curor_position, c);
        self.cursor_right();
    }

    pub fn cursor_right(&mut self) {
        if self.curor_position < self.input.len() {
            self.curor_position += 1;
        }
    }

    pub fn cursor_left(&mut self) {
        if self.curor_position > 0 {
            self.curor_position -= 1;
        }
    }

    pub fn input_display(&self) -> String {
        String::from(INPUT_PROMPT) + &self.input
    }

    pub fn pop(&mut self) {
        if self.curor_position > 0 {
            self.input.remove(self.curor_position - 1);
            self.curor_position -= 1;
        }
    }

    pub fn calculate_cursor_coordinates(&self, area: Rect) -> (u16, u16) {
        const BORDER_WIDTH: u16 = 1;
        // available width for actual text (inside borders + after prompt)
        let text_width = (area.width - BORDER_WIDTH * 2) as usize;

        // wrap returns Vec<Cow<str>>—each entry is one visual line
        let text = self.input_display();
        let wrapped: Vec<_> = wrap(&text, text_width).into_iter().collect();

        // which line we end up on?
        let last_row = wrapped.len().saturating_sub(1);
        // how many chars on that last line?
        let last_line = &wrapped[last_row];
        let col_offset = last_line.chars().count() as u16;

        // clamp to the box size
        let x = area.x + BORDER_WIDTH + min(col_offset, area.width - BORDER_WIDTH * 2);
        let y = area.y + BORDER_WIDTH + min(last_row as u16, area.height - BORDER_WIDTH * 2);

        (x, y)
    }
}

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
        frame.set_cursor_position(insert_state.front.calculate_cursor_coordinates(sections[0]));
    } else {
        frame.set_cursor_position(insert_state.back.calculate_cursor_coordinates(sections[1]));
    }

    let front = Paragraph::new(Text::from(front_text))
        .block(Block::bordered().title("Front"))
        .wrap(Wrap { trim: false })
        .style(if insert_state.focused_front { prelude::Style::default().fg(prelude::Color::Yellow) } else { prelude::Style::default() });
    let back = Paragraph::new(Text::from(back_text))
        .block(Block::bordered().title("Back"))
        .wrap(Wrap { trim: false })
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
        Action::Enter => {
            if state.focused_front && state.back.get_input().is_empty() {
                state.focused_front = false;
            } else {
                deck.add_note(Note::new(state.front.input.clone(), state.back.input.clone(), NoteType::Basic));
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
