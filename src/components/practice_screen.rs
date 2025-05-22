use crate::{
    action::{Action, Screen},
    models::{card::Card, deck::Deck},
};
use color_eyre::Result;
use ratatui::{Frame, layout::Rect};

#[derive(Default)]
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

impl Default for Mode {
    fn default() -> Self {
        Mode::Front
    }
}

impl PracticeScreen {
    pub fn update(&mut self, deck: &Deck, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Screen(Screen::Practice(_)) => {
                self.cnt = 0;
                self.cards = deck.get_cards();
            }
            Action::Char('c') => return Ok(Some(Action::Screen(Screen::Home))),
            Action::Char('q') => return Ok(Some(Action::Quit)),
            _ => {}
        }
        Ok(None)
    }

    pub fn draw(&mut self, deck: &Deck, frame: &mut Frame, area: Rect) -> Result<()> {
        let card = self.cards[self.cnt].clone();
        match self.mode {
            Mode::Front => draw_front(card, frame, area),
            Mode::Back => {}
            Mode::Complete => {}
        };
        Ok(())
    }
}

fn draw_status_bar(num_cards: usize, cnt: usize, frame: &mut Frame, area: Rect) {
    let status = format!("Card {}/{}", cnt + 1, num_cards);
}

fn draw_front(card: Card, frame: &mut Frame, area: Rect) {}
