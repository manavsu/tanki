use std::collections::HashSet;
use std::usize;

use color_eyre::Result;
use color_eyre::owo_colors::colors::xterm::DarkFeijoaGreen;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::action::Action;
use crate::components::component::Component;
use crate::models::collection::Collection;
use crate::models::deck::Deck;

static DECK_SYMBOL: &str = "";
static CARD_SYMBOL: &str = "";
static COLLAPSED_SYMBOL: &str = "";
static EXPANDED_SYMBOL: &str = "";
static ADD_DECK_SYMBOL: &str = "";
static ADD_CARD_SYMBOL: &str = "";

enum Mode {
    Normal,
    InsertDeck(Uuid, String),
}

enum Options {
    DeckItem(Uuid),
    AddToItem(Uuid),
}

pub struct Home {
    tx: UnboundedSender<Action>,
    state: ListState,
    num_options: usize,
    options: Vec<Options>,
    collection: Collection,
    expanded: HashSet<Uuid>,
    mode: Mode,
}

impl Home {
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
    fn handle_key_event_normal(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('q') => {
                return Ok(Some(Action::Quit));
            }
            KeyCode::Up | KeyCode::Down => {
                update_list_selection(key.code, &mut self.state, self.num_options);
            }
            KeyCode::Enter => {
                if let Some(selected) = self.state.selected() {
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

    fn handle_key_event_insert(&mut self, key: KeyEvent, uuid: Uuid, input: String) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char(char) => self.mode = Mode::InsertDeck(uuid, input + &char.to_string()),
            KeyCode::Backspace => {
                self.mode = Mode::InsertDeck(uuid, input[..input.len().saturating_sub(1)].to_string());
            }
            KeyCode::Enter => {
                if !input.is_empty() {
                    let new_deck = Deck::new(input);
                    self.collection.add_deck_to(uuid, new_deck);
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

    fn build_deck_list_items(&self, parent_uuid: Uuid, depth: u32) -> (Vec<ListItem<'static>>, Vec<Options>) {
        let decks = match self.collection.get_deck(parent_uuid) {
            Some(deck) => deck.get_subdecks(),
            None => if self.collection.uuid == parent_uuid {&self.collection.decks} else {&Vec::new()},
        };
        let mut deck_items = Vec::new();
        let mut options = Vec::new();
        for deck in decks.iter() {
            let deck_expanded = self.expanded.contains(&deck.uuid);
            deck_items.push(ListItem::new(build_deck_label(deck)));
            options.push(Options::DeckItem(deck.uuid));
            if deck_expanded {
                let (d_items, o_items) = self.build_deck_list_items(deck.uuid, depth + 1);
                deck_items.extend(d_items);
                options.extend(o_items);
            }
        }
        if let Mode::InsertDeck(uuid, input) = &self.mode {
            if uuid == &parent_uuid {
                deck_items.push(ListItem::new(Text::from(input.clone())));
                options.push(Options::AddToItem(*uuid));
            }
            return (deck_items, options);
        }
        deck_items.push(ListItem::new(Text::from(ADD_DECK_SYMBOL)));
        options.push(Options::AddToItem(parent_uuid));
        (deck_items, options)
    }
}

impl Component for Home {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        Ok(None)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match &self.mode {
            Mode::Normal => self.handle_key_event_normal(key),
            Mode::InsertDeck(uuid, input) => self.handle_key_event_insert(key, *uuid, input.clone()),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let (decks, options) = self.build_deck_list_items(self.collection.uuid, 0);
        self.options = options;
        self.num_options = self.options.len();

        let chunks = Layout::vertical([Constraint::Length(6), Constraint::Min(0)]).margin(1).split(area);

        frame.render_widget(create_logo_lext(), chunks[0]);
        let list = List::new(decks)
            .block(Block::bordered())
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .direction(ListDirection::TopToBottom);
        frame.render_stateful_widget(list, chunks[1], &mut self.state);
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
        deck.name.clone()
            + " "
            + CARD_SYMBOL
            + " "
            + &deck.get_cards().len().to_string()
            + " "
            + DECK_SYMBOL
            + " "
            + &deck.get_subdecks().len().to_string() + " " + &deck.uuid.as_u128().to_string(),
    )
}

fn create_logo_lext() -> Text<'static> {
    Text::from(
        "████████╗ █████╗ ███╗  ██╗██╗  ██╗██╗
╚══██╔══╝██╔══██╗████╗ ██║██║ ██╔╝██║
   ██║   ███████║██╔██╗██║█████═╝ ██║
   ██║   ██╔══██║██║╚████║██╔═██╗ ██║
   ██║   ██║  ██║██║ ╚███║██║ ╚██╗██║
   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚══╝╚═╝  ╚═╝╚═╝",
    )
}
