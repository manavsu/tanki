use crate::{
    action::{Action, Screen},
    models::{card::Card, deck::Deck},
};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    widgets::{Block, Borders, Paragraph},
};

use super::title;

const DIVIDER_TEXT: &str = "\n\n──────────\n\n";

pub struct PracticeScreen {
    cnt: usize,
    cards: Vec<Card>,
    mode: Mode,
}

enum Mode {
    Front,
    Back,
    Complete,
}

impl Default for PracticeScreen {
    fn default() -> Self {
        Self { cnt: 0, cards: Vec::new(), mode: Mode::Front }
    }
}

impl PracticeScreen {
    pub fn update(&mut self, deck: &Deck, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Screen(Screen::Practice(_)) => {
                self.cnt = 0;
                self.mode = Mode::Front;
                self.cards = deck.get_cards();
            }
            Action::Char('c') => return Ok(Some(Action::Screen(Screen::Home))),
            Action::Char('q') => return Ok(Some(Action::Quit)),
            Action::Space => match self.mode {
                Mode::Front => self.mode = Mode::Back,
                Mode::Back => {
                    if self.cnt == self.cards.len() - 1 {
                        self.mode = Mode::Complete;
                    } else {
                        self.cnt = self.cnt.wrapping_add(1) % self.cards.len();
                        self.mode = Mode::Front;
                    }
                }
                Mode::Complete => return Ok(Some(Action::Screen(Screen::Home))),
            },
            _ => {}
        }
        Ok(None)
    }

    pub fn draw(&mut self, deck: &Deck, frame: &mut Frame, area: Rect) -> Result<()> {
        let card = self.cards[self.cnt].clone();
        let chunks = Layout::vertical([Constraint::Length(7), Constraint::Min(0), Constraint::Length(3)]).split(area);
        title::draw_title(frame, chunks[0])?;
        match self.mode {
            Mode::Front => self.draw_front(card, frame, chunks[1]),
            Mode::Back => self.draw_back(card, frame, chunks[1]),
            Mode::Complete => self.draw_complete(frame, chunks[1]),
        };
        Ok(())
    }

    fn draw_front(&self, card: Card, frame: &mut Frame, area: Rect) {
        let front = Paragraph::new("\n".to_string() + &card.front + "\n")
            .centered()
            .block(Block::default().title(format!("[Practice][{}/{}]", self.cnt + 1, self.cards.len())).borders(Borders::ALL));
        frame.render_widget(front, area);
    }

    fn draw_back(&self, card: Card, frame: &mut Frame<'_>, area: Rect) {
        let front = Paragraph::new("\n".to_string() + &card.front + DIVIDER_TEXT + &card.back + "\n")
            .centered()
            .block(Block::default().title(format!("[Practice][{}/{}]", self.cnt + 1, self.cards.len())).borders(Borders::ALL));
        frame.render_widget(front, area);
    }

    fn draw_complete(&self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Paragraph::new("\nCompleted!\n").centered().block(Block::default().title("[Practice][Complete]").borders(Borders::ALL)),
            area,
        );
    }
}
