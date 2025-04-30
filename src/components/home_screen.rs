use std::collections::HashSet;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use deck_panel::InsertNoteState;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::action::Action;
use crate::models::collection::Collection;
use crate::models::deck::Deck;

mod deck_panel;

mod title;

static DECK_SYMBOL: &str = "";
static CARD_SYMBOL: &str = "";
static COLLAPSED_SYMBOL: &str = "";
static EXPANDED_SYMBOL: &str = "";
static ADD_DECK_SYMBOL: &str = "  add deck";
static ADD_CARD_SYMBOL: &str = "";
static CURSOR: &str = "█";

#[derive(Clone)]
enum Mode {
    Normal,
    InsertDeck(Uuid, String),
    InsertNote(InsertNoteState),
}

enum Options {
    DeckItem(Uuid),
    AddToItem(Uuid),
}

pub struct HomeScreen {
    tx: UnboundedSender<Action>,
    state: ListState,
    num_options: usize,
    options: Vec<Options>,
    collection: Collection,
    expanded: HashSet<Uuid>,
    mode: Mode,
}

impl HomeScreen {
    pub fn new(tx: UnboundedSender<Action>) -> Self {
        Self {
            tx,
            state: ListState::default(),
            num_options: 0,
            collection: Collection::default(),
            expanded: HashSet::new(),
            options: Vec::new(),
            mode: Mode::Normal,
        }
    }
    pub fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match &self.mode {
            Mode::Normal => self.update_normal(action),
            Mode::InsertDeck(uuid, _) => Ok(None),
            Mode::InsertNote(state) => {
                let state = deck_panel::update_deck_panel_note_insert(action, state.clone(), self.get_selected_deck_mut().unwrap());
                if state.completed {
                    self.mode = Mode::Normal;
                } else {
                    self.mode = Mode::InsertNote(state);
                }
                Ok(None)
            }
        }
    }

    pub fn update_normal(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Char('n') => {
                self.mode = Mode::InsertNote(InsertNoteState::new());
            }
            Action::Up => {}
            _ => {}
        };
        Ok(None)
    }

    fn handle_key_event_normal(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('q') => {
                return Ok(Some(Action::Quit));
            }
            KeyCode::Up | KeyCode::Down => {
                update_list_selection(key.code, &mut self.state, self.num_options);
            }
            KeyCode::Enter => {
                if self.state.selected().is_some() {
                    return Ok(None); // TODO
                }
            }
            KeyCode::Char(' ') => {
                if let Some(selected) = self.state.selected() {
                    match self.options[selected] {
                        Options::DeckItem(uuid) => {
                            if self.expanded.contains(&uuid) {
                                self.expanded.remove(&uuid);
                            } else {
                                self.expanded.insert(uuid);
                            }
                        }
                        Options::AddToItem(uuid) => {
                            self.mode = Mode::InsertDeck(uuid, String::new());
                        }
                    }
                }
            }
            _ => {}
        };
        Ok(None)
    }

    fn get_selected_deck(&self) -> Option<&Deck> {
        if let Some(selected) = self.state.selected() {
            if selected < self.options.len() {
                match &self.options[selected] {
                    Options::DeckItem(uuid) => return self.collection.find_deck(*uuid),
                    Options::AddToItem(uuid) => return self.collection.find_deck(*uuid),
                }
            }
        }
        None
    }

    fn get_selected_deck_mut(&mut self) -> Option<&mut Deck> {
        if let Some(selected) = self.state.selected() {
            if selected < self.options.len() {
                match &self.options[selected] {
                    Options::DeckItem(uuid) => return self.collection.find_deck_mut(*uuid),
                    Options::AddToItem(uuid) => return self.collection.find_deck_mut(*uuid),
                }
            }
        }
        None
    }

    fn handle_key_event_insert(&mut self, key: KeyEvent, uuid: Uuid, input: String) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char(char) => self.mode = Mode::InsertDeck(uuid, input + &char.to_string()),
            KeyCode::Backspace => {
                self.mode = Mode::InsertDeck(uuid, input[..input.len().saturating_sub(1)].to_string());
            }
            KeyCode::Enter => {
                if !input.is_empty() {
                    self.collection.add_deck_to(uuid, Deck::new(input));
                }
                self.mode = Mode::Normal;
            }
            KeyCode::Esc => {
                self.mode = Mode::Normal;
            }
            _ => {}
        };
        Ok(None)
    }

    fn build_deck_list_items(&self, parent_uuid: Uuid, _depth: u32) -> (Vec<ListItem<'static>>, Vec<Options>) {
        let decks = match self.collection.find_deck(parent_uuid) {
            Some(deck) => deck.get_subdecks(),
            None => {
                if parent_uuid == self.collection.uuid {
                    self.collection.get_decks()
                } else {
                    return (Vec::new(), Vec::new());
                }
            }
        };

        let mut deck_items = Vec::new();
        let mut options = Vec::new();
        let spacing = " ".repeat(_depth as usize * 2);
        for deck in decks.iter() {
            let deck_expanded = self.expanded.contains(&deck.uuid);
            let header = spacing.clone() + &(if deck_expanded { EXPANDED_SYMBOL.to_string() } else { COLLAPSED_SYMBOL.to_string() });
            deck_items.push(ListItem::new(build_deck_label(deck, header)));
            options.push(Options::DeckItem(deck.uuid));
            if deck_expanded {
                let (d_items, o_items) = self.build_deck_list_items(deck.uuid, _depth + 1);
                deck_items.extend(d_items);
                options.extend(o_items);
            }
        }
        if let Mode::InsertDeck(uuid, input) = &self.mode {
            if uuid == &parent_uuid {
                deck_items.push(ListItem::new(Text::from(spacing + ">> " + &input.clone() + CURSOR)));
                options.push(Options::AddToItem(*uuid));
            } else {
                deck_items.push(ListItem::new(Text::from(spacing + ADD_DECK_SYMBOL)));
                options.push(Options::AddToItem(parent_uuid));
            }
        } else {
            deck_items.push(ListItem::new(Text::from(spacing + ADD_DECK_SYMBOL)));
            options.push(Options::AddToItem(parent_uuid));
        }
        (deck_items, options)
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match &self.mode {
            Mode::Normal => self.handle_key_event_normal(key),
            Mode::InsertDeck(uuid, input) => self.handle_key_event_insert(key, *uuid, input.clone()),
            Mode::InsertNote(_) => Ok(None),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let (decks, options) = self.build_deck_list_items(self.collection.uuid, 1);
        self.options = options;
        self.num_options = self.options.len();

        let chunks = Layout::vertical([Constraint::Length(8), Constraint::Min(0)]).split(area);
        title::draw_title(frame, chunks[0])?;

        let horizontal_chunks = Layout::horizontal(Constraint::from_percentages([50, 50])).split(chunks[1]);
        deck_panel::draw_deck_panel(
            frame,
            horizontal_chunks[1],
            self.get_selected_deck(),
            if let Mode::InsertNote(state) = &self.mode { Some(state.clone()) } else { None },
        )?;
        frame.render_widget(Block::bordered(), horizontal_chunks[0]);

        let list_chunks = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).margin(1).split(horizontal_chunks[0]);
        frame.render_widget(Text::from("collection"), list_chunks[0]);

        let list =
            List::new(decks).highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)).direction(ListDirection::TopToBottom);
        frame.render_stateful_widget(list, list_chunks[1], &mut self.state);
        Ok(())
    }
}

fn update_list_selection(key_code: KeyCode, state: &mut ListState, num_options: usize) {
    if state.selected().is_none() {
        state.select(Some(0));
        return;
    }
    match key_code {
        KeyCode::Up => {
            state.select(Some(state.selected().unwrap().wrapping_sub(1)));
        }
        KeyCode::Down => {
            state.select(Some(state.selected().unwrap().wrapping_add(1) % num_options));
        }
        _ => {}
    }
}

fn build_deck_label(deck: &Deck, header: String) -> Text<'static> {
    Text::from(
        header
            + " "
            + &deck.name.clone()
            + " "
            + CARD_SYMBOL
            + " "
            + &deck.get_cards().len().to_string()
            + " "
            + DECK_SYMBOL
            + " "
            + &deck.get_subdecks().len().to_string(),
    )
}
