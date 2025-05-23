use crate::components::title;
use std::collections::HashSet;

use color_eyre::Result;
use deck_panel::InsertNoteState;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::action::{self, Action, Screen};
use crate::models::collection::Collection;
use crate::models::deck::Deck;

mod command_bar;
mod deck_panel;
mod input_state;

static DECK_SYMBOL: &str = "";
static CARD_SYMBOL: &str = "";
static COLLAPSED_SYMBOL: &str = "";
static EXPANDED_SYMBOL: &str = "";
static CURSOR: &str = "█";
static INPUT_PROMPT: &str = ">> ";

#[derive(Clone)]
enum Mode {
    Normal(Option<Uuid>),
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
    expanded: HashSet<Uuid>,
    mode: Mode,
}

impl HomeScreen {
    pub fn new(tx: UnboundedSender<Action>) -> Self {
        Self { tx, state: ListState::default(), num_options: 0, expanded: HashSet::new(), options: Vec::new(), mode: Mode::Normal(None) }
    }

    pub fn update(&mut self, collection: &mut Collection, action: Action) -> Result<Option<Action>> {
        match &self.mode {
            Mode::Normal(_) => self.update_normal(collection, action),
            Mode::InsertDeck(uuid, input) => self.update_insert(collection, action, *uuid, input.clone()),
            Mode::InsertNote(state) => {
                let state = deck_panel::update_deck_panel_note_insert(action, state.clone(), self.get_selected_deck_mut(collection).unwrap());
                if state.completed {
                    self.mode = Mode::Normal(self.get_selected_deck(collection).map(|d| d.uuid));
                    Ok(Some(Action::Save))
                } else {
                    self.mode = Mode::InsertNote(state);
                    Ok(None)
                }
            }
        }
    }
    pub fn update_normal(&mut self, collection: &mut Collection, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Char('n') => {
                if self.get_selected_deck(collection).is_some() {
                    self.mode = Mode::InsertNote(InsertNoteState::new());
                }
            }
            Action::Char('s') => {
                let parent_uuid = if let Some(deck) = self.get_selected_deck(collection) { deck.uuid } else { collection.uuid };
                if !self.expanded.contains(&parent_uuid) {
                    self.expanded.insert(parent_uuid);
                }
                self.mode = Mode::InsertDeck(parent_uuid, String::new());
            }
            Action::Char('a') => {
                self.mode = Mode::InsertDeck(collection.uuid, String::new());
            }
            Action::Char('q') => return Ok(Some(Action::Quit)),
            Action::Char('D') => {
                if self.get_selected_deck(collection).is_some() {
                    if let Some(selected) = self.state.selected() {
                        if let Options::DeckItem(uuid) = &self.options[selected] {
                            collection.remove_deck(*uuid);
                        }
                    }
                }
            }
            Action::Up | Action::Down => {
                update_list_selection(action, &mut self.state, self.num_options);
                self.mode = Mode::Normal(self.get_selected_deck(collection).map(|d| d.uuid));
            }
            Action::Space => {
                if let Some(selected) = self.state.selected() {
                    match self.options[selected] {
                        Options::DeckItem(uuid) => {
                            if self.expanded.contains(&uuid) {
                                self.expanded.remove(&uuid);
                            } else {
                                self.expanded.insert(uuid);
                            }
                        }
                        Options::AddToItem(_) => {}
                    }
                }
            }
            Action::Enter => {
                if let Some(selected) = self.get_selected_deck(collection) {
                    return Ok(Some(Action::Screen(Screen::Practice(selected.uuid))));
                }
            }
            _ => {}
        };
        Ok(None)
    }

    fn update_insert(&mut self, collection: &mut Collection, action: Action, uuid: Uuid, input: String) -> Result<Option<Action>> {
        match action {
            Action::Space => self.mode = Mode::InsertDeck(uuid, input + " "),
            Action::Char(c) => self.mode = Mode::InsertDeck(uuid, input + &c.to_string()),
            Action::Backspace => {
                self.mode = Mode::InsertDeck(uuid, input[..input.len().saturating_sub(1)].to_string());
            }
            Action::Enter => {
                self.mode = Mode::Normal(self.get_selected_deck(collection).map(|d| d.uuid));
                if !input.is_empty() {
                    collection.add_deck_to(uuid, Deck::new(input));
                    return Ok(Some(Action::Save));
                }
            }
            Action::Esc => {
                self.mode = Mode::Normal(self.get_selected_deck(collection).map(|d| d.uuid));
            }
            _ => {}
        };
        Ok(None)
    }

    fn select_add_item(&mut self) {
        for (i, opt) in self.options.iter().enumerate() {
            if let Options::AddToItem(_) = opt {
                self.state.select(Some(i));
            }
        }
    }

    fn get_selected_deck(&self, collection: &Collection) -> Option<Deck> {
        if let Some(selected) = self.state.selected() {
            if selected < self.options.len() {
                match &self.options[selected] {
                    Options::DeckItem(uuid) => return collection.find_deck(*uuid).cloned(),
                    Options::AddToItem(uuid) => return collection.find_deck(*uuid).cloned(),
                }
            }
        }
        None
    }

    fn get_selected_deck_mut<'a>(&mut self, collection: &'a mut Collection) -> Option<&'a mut Deck> {
        if let Some(selected) = self.state.selected() {
            if selected < self.options.len() {
                match &self.options[selected] {
                    Options::DeckItem(uuid) => return collection.find_deck_mut(*uuid),
                    Options::AddToItem(uuid) => return collection.find_deck_mut(*uuid),
                }
            }
        }
        None
    }

    fn build_deck_list_items(&self, collection: &Collection, parent_uuid: Uuid, _depth: u32) -> (Vec<ListItem<'static>>, Vec<Options>) {
        let decks = match collection.find_deck(parent_uuid) {
            Some(deck) => deck.get_subdecks(),
            None => {
                if parent_uuid == collection.uuid {
                    collection.get_decks()
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
                let (d_items, o_items) = self.build_deck_list_items(collection, deck.uuid, _depth + 1);
                deck_items.extend(d_items);
                options.extend(o_items);
            }
        }
        if let Mode::InsertDeck(uuid, input) = &self.mode {
            if uuid == &parent_uuid {
                deck_items.push(ListItem::new(Text::from(spacing + ">> " + &input.clone() + CURSOR)));
                options.push(Options::AddToItem(*uuid));
            }
        }
        (deck_items, options)
    }

    pub fn draw(&mut self, collection: &Collection, frame: &mut Frame, area: Rect) -> Result<()> {
        let (decks, options) = self.build_deck_list_items(collection, collection.uuid, 0);
        self.options = options;
        self.num_options = self.options.len();
        self.select_add_item();

        let chunks = Layout::vertical([Constraint::Length(7), Constraint::Min(0), Constraint::Length(3)]).split(area);
        title::draw_title(frame, chunks[0])?;

        let horizontal_chunks = Layout::horizontal(Constraint::from_percentages([25, 75])).split(chunks[1]);
        deck_panel::draw_deck_panel(
            frame,
            horizontal_chunks[1],
            self.get_selected_deck(collection),
            if let Mode::InsertNote(state) = &self.mode { Some(state.clone()) } else { None },
        )?;
        command_bar::draw_command_bar(frame, chunks[2], self.mode.clone());
        frame.render_widget(Block::bordered(), horizontal_chunks[0]);

        let list =
            List::new(decks).highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)).direction(ListDirection::TopToBottom);
        frame.render_stateful_widget(list.block(Block::bordered().title("[collection]")), horizontal_chunks[0], &mut self.state);
        Ok(())
    }
}

fn update_list_selection(action: Action, state: &mut ListState, num_options: usize) {
    if state.selected().is_none() {
        state.select(Some(0));
        return;
    }
    match action {
        Action::Up => {
            state.select(Some(state.selected().unwrap().wrapping_sub(1)));
        }
        Action::Down => {
            state.select(Some(state.selected().unwrap().wrapping_add(1) % num_options));
        }
        _ => {}
    }
}

fn build_deck_label(deck: &Deck, header: String) -> Text<'static> {
    Text::from(
        header
            + " "
            + CARD_SYMBOL
            + " "
            + &deck.get_cards().len().to_string()
            + " "
            + DECK_SYMBOL
            + " "
            + &deck.get_subdecks().len().to_string()
            + " "
            + &deck.name.clone(),
    )
}
