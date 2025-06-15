// Use the decks col in col table to determine decks and hieracy and then read the cards form other
// tngs
//
//

use std::{fs::File, path::PathBuf};

use tempfile::tempdir;
use zip::ZipArchive;

use crate::models::{deck::Deck, note::Note};

pub fn load_from_anki_package(path: PathBuf) -> Deck {
    let file = File::open(&path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    let mut collection_file = archive.by_name("collection.anki2").unwrap();

    let dir = tempdir().unwrap();
    let collection_path = dir.path().join("collection.anki2");
    {
        let mut out = File::create(&collection_path).unwrap();
        std::io::copy(&mut collection_file, &mut out).unwrap();
    }

    let conn = rusqlite::Connection::open(&collection_path).unwrap();
    let mut stmt = conn.prepare("SELECT flds FROM notes LIMIT 200;").unwrap();
    let rows = stmt
        .query_map([], |row| {
            let flds: String = row.get(0)?;
            Ok(flds)
        })
        .unwrap();

    let notes: Vec<Note> = rows
        .into_iter()
        .flatten()
        .map(|flds| {
            let fields: Vec<_> = flds.split('\x1f').collect();
            let front = *fields.first().unwrap_or(&"");
            let back = *fields.get(1).unwrap_or(&"");
            Note::new(front.into(), back.into(), crate::models::note::NoteType::Basic)
        })
        .collect();

    let mut deck = Deck::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or("Imported").to_string());
    notes.into_iter().for_each(|n| deck.add_note(n));

    deck
}
