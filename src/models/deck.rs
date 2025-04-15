use serde::Deserialize;
use serde::Serialize;

use crate::models::card::Card;
use crate::models::note::Note;

#[derive(Serialize, Deserialize)]
pub struct Deck {
    pub(crate) name: String,
    pub(crate) subdecks: Vec<Deck>,
    pub(crate) notes: Vec<Note>,
}

impl Deck {
    pub fn new(name: String) -> Self {
        Deck { name, subdecks: Vec::new(), notes: Vec::new() }
    }

    pub fn add_subdeck(&mut self, deck: Deck) {
        self.subdecks.push(deck);
    }

    pub fn add_card(&mut self, card: Note) {
        self.notes.push(card);
    }

    pub(crate) fn get_cards(&self) -> Vec<Card> {
        self.notes.iter().flat_map(|n| n.get_cards()).collect()
    }

    pub fn get_all_cards(&self) -> Vec<Card> {
        self.subdecks.iter().flat_map(|d| d.get_all_cards()).chain(self.get_cards()).collect()
    }

    pub fn get_subdecks(&self) -> &Vec<Deck> {
        &self.subdecks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::note::{Note, NoteType};

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new("Test Deck".to_string());

        assert_eq!(deck.name, "Test Deck");
        assert_eq!(deck.subdecks.len(), 0);
        assert_eq!(deck.notes.len(), 0);
    }

    #[test]
    fn test_add_subdeck() {
        let mut parent_deck = Deck::new("Parent".to_string());
        let child_deck = Deck::new("Child".to_string());

        parent_deck.add_subdeck(child_deck);

        assert_eq!(parent_deck.subdecks.len(), 1);
        assert_eq!(parent_deck.subdecks[0].name, "Child");
    }

    #[test]
    fn test_add_note() {
        let mut deck = Deck::new("Test Deck".to_string());
        let note = Note::new("Front".to_string(), "Back".to_string(), NoteType::Basic);

        deck.add_card(note);

        assert_eq!(deck.notes.len(), 1);
        assert_eq!(deck.notes[0].front, "Front");
    }

    #[test]
    fn test_get_cards() {
        let mut deck = Deck::new("Test Deck".to_string());
        deck.add_card(Note::new("Q1".to_string(), "A1".to_string(), NoteType::Basic));
        deck.add_card(Note::new("Q2".to_string(), "A2".to_string(), NoteType::BasicAndReverse));

        let cards = deck.get_cards();

        // 1 card from Basic note + 2 cards from BasicAndReverse note
        assert_eq!(cards.len(), 3);
    }

    #[test]
    fn test_get_all_cards() {
        let mut parent_deck = Deck::new("Parent".to_string());
        let mut child_deck = Deck::new("Child".to_string());

        parent_deck.add_card(Note::new("PQ".to_string(), "PA".to_string(), NoteType::Basic));
        child_deck.add_card(Note::new("CQ".to_string(), "CA".to_string(), NoteType::Basic));
        parent_deck.add_subdeck(child_deck);

        let all_cards = parent_deck.get_all_cards();

        assert_eq!(all_cards.len(), 2);
    }
}
