use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::models::deck::Deck;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Collection {
    pub uuid: Uuid,
    pub decks: Vec<Deck>,
}

impl Collection {
    pub fn new() -> Self {
        Collection { decks: Vec::new(), uuid: uuid::Uuid::new_v4() }
    }

    pub fn add_deck(&mut self, deck: Deck) {
        self.decks.push(deck);
    }

    pub fn find_deck_mut(&mut self, uuid: Uuid) -> Option<&mut Deck> {
        for deck in &mut self.decks {
            if let Some(found) = deck.find_deck_mut(uuid) {
                return Some(found);
            }
        }
        None
    }

    pub fn get_decks(&self) -> &[Deck] {
        &self.decks
    }

    pub fn get_all_decks(&self) -> Vec<&Deck> {
        self.decks.iter().flat_map(|d| d.get_all_subdecks()).chain(self.get_decks()).collect()
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::deck::Deck;

    #[test]
    fn test_collection_creation() {
        let collection = Collection::new();
        assert!(collection.decks.is_empty());
    }

    #[test]
    fn test_add_deck() {
        let mut collection = Collection::new();
        let deck = Deck::new(String::from("My Deck")); // Assuming Deck has a `new` method
        collection.add_deck(deck);
        assert_eq!(collection.decks.len(), 1);
    }
}
