use serde::Deserialize;
use serde::Serialize;

use crate::models::deck::Deck;

#[derive(Serialize, Deserialize)]
pub struct Collection {
    name: String,
    decks: Vec<Deck>,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Collection { name, decks: Vec::new() }
    }
    pub fn add_deck(&mut self, deck: Deck) {
        self.decks.push(deck);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::deck::Deck;

    #[test]
    fn test_collection_creation() {
        let collection = Collection::new(String::from("My Collection"));
        assert_eq!(collection.name, "My Collection");
        assert!(collection.decks.is_empty());
    }

    #[test]
    fn test_add_deck() {
        let mut collection = Collection::new(String::from("My Collection"));
        let deck = Deck::new(String::from("My Deck")); // Assuming Deck has a `new` method
        collection.add_deck(deck);
        assert_eq!(collection.decks.len(), 1);
    }
}
