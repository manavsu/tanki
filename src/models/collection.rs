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

    pub fn add_deck_to(&mut self, uuid: Uuid, deck: Deck) {
        if let Some(existing_deck) = self.decks.iter_mut().find(|d| d.uuid == uuid) {
            existing_deck.add_subdeck(deck);
        } else {
            self.decks.push(deck);
        }
    }

    pub fn get_deck(&self, uuid: Uuid) -> Option<&Deck> {
        self.decks.iter().find(|d| d.uuid == uuid)
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
