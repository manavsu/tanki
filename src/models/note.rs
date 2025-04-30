use serde::Deserialize;
use serde::Serialize;

use crate::models::card::Card;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct Note {
    pub front: String,
    pub back: String,
    pub note_type: NoteType,
}

impl Note {
    pub fn new(front: String, back: String, note_type: NoteType) -> Self {
        Note { front, back, note_type }
    }

    pub fn get_cards(&self) -> Vec<Card> {
        match self.note_type {
            NoteType::Basic => vec![Card { front: self.front.clone(), back: self.back.clone() }],
            NoteType::BasicAndReverse => {
                vec![Card { front: self.front.clone(), back: self.back.clone() }, Card { front: self.back.clone(), back: self.front.clone() }]
            }
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum NoteType {
    Basic,
    BasicAndReverse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_creation() {
        let note = Note::new("Front text".to_string(), "Back text".to_string(), NoteType::Basic);

        assert_eq!(note.front, "Front text");
        assert_eq!(note.back, "Back text");
        matches!(note.note_type, NoteType::Basic);
    }

    #[test]
    fn test_get_cards_basic() {
        let note = Note::new("Question".to_string(), "Answer".to_string(), NoteType::Basic);

        let cards = note.get_cards();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].front, "Question");
        assert_eq!(cards[0].back, "Answer");
    }

    #[test]
    fn test_get_cards_basic_and_reverse() {
        let note = Note::new("Question".to_string(), "Answer".to_string(), NoteType::BasicAndReverse);

        let cards = note.get_cards();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].front, "Question");
        assert_eq!(cards[0].back, "Answer");
        assert_eq!(cards[1].front, "Answer");
        assert_eq!(cards[1].back, "Question");
    }
}
