use serde::Deserialize;
use serde::Serialize;

use crate::models::deck::Deck;

#[derive(Serialize, Deserialize)]
pub struct Collection {
    decks: Vec<Deck>,
}

impl Collection {
    pub fn new() -> Self {
        Collection {  decks: Vec::new() }
    }
    pub fn add_deck(&mut self, deck: Deck) {
        self.decks.push(deck);
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
