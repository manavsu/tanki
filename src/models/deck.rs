use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::models::card::Card;
use crate::models::note::Note;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Deck {
    pub name: String,
    pub uuid: Uuid,
    subdecks: Vec<Deck>,
    notes: Vec<Note>,
    parent: Option<String>,
}

impl Deck {
    pub fn new(name: String) -> Self {
        Deck { name, subdecks: Vec::new(), notes: Vec::new(), uuid: Uuid::new_v4(), parent: None }
    }

    pub fn qualified_name(&self) -> String {
        match self.parent {
            Some(ref parent_name) => format!("{}:{}", parent_name, self.name),
            None => self.name.clone(),
        }
    }

    pub fn add_subdeck(&mut self, mut deck: Deck) {
        deck.parent = Some(self.qualified_name());
        self.subdecks.push(deck);
    }

    pub fn add_note(&mut self, card: Note) {
        self.notes.push(card);
    }

    pub(crate) fn get_cards(&self) -> Vec<Card> {
        self.notes.iter().flat_map(|n| n.get_cards()).collect()
    }

    pub fn get_all_cards(&self) -> Vec<Card> {
        self.subdecks.iter().flat_map(|d| d.get_all_cards()).chain(self.get_cards()).collect()
    }

    pub fn get_notes(&self) -> &[Note] {
        &self.notes
    }

    pub fn get_subdecks(&self) -> &[Deck] {
        &self.subdecks
    }

    pub fn get_all_subdecks(&self) -> Vec<&Deck> {
        self.subdecks.iter().flat_map(|d| d.get_all_subdecks()).chain(self.get_subdecks()).collect()
    }

    pub fn remove_deck(&mut self, uuid: Uuid) {
        if let Some(pos) = self.subdecks.iter().position(|deck| deck.uuid == uuid) {
            self.subdecks.remove(pos);
        } else {
            for deck in &mut self.subdecks {
                deck.remove_deck(uuid);
            }
        }
    }

    pub fn find_deck_mut(&mut self, uuid: Uuid) -> Option<&mut Deck> {
        if self.uuid == uuid {
            return Some(self);
        }
        for subdeck in &mut self.subdecks {
            if let Some(deck) = subdeck.find_deck_mut(uuid) {
                return Some(deck);
            }
        }
        None
    }

    pub fn find_deck(&self, uuid: Uuid) -> Option<&Deck> {
        if self.uuid == uuid {
            return Some(self);
        }
        for subdeck in &self.subdecks {
            if let Some(deck) = subdeck.find_deck(uuid) {
                return Some(deck);
            }
        }
        None
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

        deck.add_note(note);

        assert_eq!(deck.notes.len(), 1);
        assert_eq!(deck.notes[0].get_cards()[0].front, "Front");
    }

    #[test]
    fn test_get_cards() {
        let mut deck = Deck::new("Test Deck".to_string());
        deck.add_note(Note::new("Q1".to_string(), "A1".to_string(), NoteType::Basic));
        deck.add_note(Note::new("Q2".to_string(), "A2".to_string(), NoteType::BasicAndReverse));

        let cards = deck.get_cards();

        // 1 card from Basic note + 2 cards from BasicAndReverse note
        assert_eq!(cards.len(), 3);
    }

    #[test]
    fn test_get_all_cards() {
        let mut parent_deck = Deck::new("Parent".to_string());
        let mut child_deck = Deck::new("Child".to_string());

        parent_deck.add_note(Note::new("PQ".to_string(), "PA".to_string(), NoteType::Basic));
        child_deck.add_note(Note::new("CQ".to_string(), "CA".to_string(), NoteType::Basic));
        parent_deck.add_subdeck(child_deck);

        let all_cards = parent_deck.get_all_cards();

        assert_eq!(all_cards.len(), 2);
    }
}
