use std::fs;
use std::path::PathBuf;

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

    pub fn load_from_file(path: PathBuf) -> Self {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(collection) = serde_json::from_str::<Collection>(&content) {
                return collection;
            }
        }
        Collection { decks: Vec::new(), uuid: uuid::Uuid::new_v4() }
    }

    pub fn save_to_file(&self, path: PathBuf) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }

    pub fn add_deck(&mut self, deck: Deck) {
        self.decks.push(deck);
    }

    pub fn add_deck_to(&mut self, uuid: Uuid, deck: Deck) {
        if let Some(found_deck) = self.find_deck_mut(uuid) {
            found_deck.add_subdeck(deck);
        } else {
            self.add_deck(deck);
        }
    }

    pub fn remove_deck(&mut self, uuid: Uuid) {
        if let Some(pos) = self.decks.iter().position(|deck| deck.uuid == uuid) {
            self.decks.remove(pos);
        } else {
            for deck in &mut self.decks {
                deck.remove_deck(uuid);
            }
        }
    }

    pub fn find_deck_mut(&mut self, uuid: Uuid) -> Option<&mut Deck> {
        for deck in &mut self.decks {
            if let Some(found) = deck.find_deck_mut(uuid) {
                return Some(found);
            }
        }
        None
    }

    pub fn find_deck(&self, uuid: Uuid) -> Option<&Deck> {
        for deck in &self.decks {
            if let Some(found) = deck.find_deck(uuid) {
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
