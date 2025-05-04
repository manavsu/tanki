use std::cmp::min;

use ratatui::layout::Rect;
use textwrap::{Options, WrapAlgorithm, wrap};

use super::INPUT_PROMPT;

#[derive(Clone)]
pub struct InputState {
    input: String,
    cursor_position: usize,
}
impl InputState {
    pub fn new() -> Self {
        Self { input: String::new(), cursor_position: 0 }
    }

    pub fn get_input(&self) -> String {
        self.input.clone()
    }

    pub fn push(&mut self, c: char) {
        // self.input = self.input[..self.curor_position].to_string() + &c.to_string() + &self.input[self.curor_position..];
        self.input.insert(self.cursor_position, c);
        self.cursor_right();
    }

    pub fn cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn input_display(&self) -> String {
        String::from(INPUT_PROMPT) + &self.input
    }

    pub fn pop(&mut self) {
        if self.cursor_position > 0 {
            self.input.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn calculate_cursor_coordinates_wrapped(&self, area: Rect) -> (u16, u16) {
        const BORDER_WIDTH: u16 = 1;
        let text_width = (area.width - BORDER_WIDTH * 2) as usize;

        let text = self.input_display();
        // TODO: handle trailing whitespace, removed by wrap, handle left right and up down
        let options = Options::new(text_width).wrap_algorithm(WrapAlgorithm::FirstFit);
        let wrapped: Vec<_> = wrap(&text, options).into_iter().collect();

        let last_row = wrapped.len().saturating_sub(1);
        let last_line = &wrapped[last_row];
        let col_offset = last_line.chars().count() as u16;

        let x = area.x + BORDER_WIDTH + min(col_offset, text_width.saturating_sub(1) as u16);
        let y = area.y + BORDER_WIDTH + min(last_row as u16, (area.height - BORDER_WIDTH * 2).saturating_sub(1));

        (x, y)
    }
}
