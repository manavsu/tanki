use color_eyre::eyre::Result;
use crossterm::event;
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, models::collection::Collection};

use super::home_screen::HomeScreen;
use super::utils;

pub enum Screen {
    Home,
}

pub struct Base {
    tx: UnboundedSender<Action>,
    home: HomeScreen,
    screen: Screen,
    collection: Collection,
}

impl Base {
    pub fn new(tx: UnboundedSender<Action>) -> Self {
        let tx_clone = tx.clone();
        Self { tx, home: HomeScreen::new(tx_clone.clone()), screen: Screen::Home, collection: Collection::load_from_file(utils::save_file_location())}
    }
}

impl Base {
    pub fn handle_key_event(&mut self, key: event::KeyEvent) -> Result<Option<Action>> {
        let action = match key.code {
            event::KeyCode::Char(' ') if event::KeyModifiers::CONTROL == key.modifiers => Some(Action::CtrlSpace),
            event::KeyCode::Char(' ') => Some(Action::Space),
            event::KeyCode::Backspace => Some(Action::Backspace),
            event::KeyCode::Enter => Some(Action::Enter),
            event::KeyCode::Left => Some(Action::Left),
            event::KeyCode::Right => Some(Action::Right),
            event::KeyCode::Up => Some(Action::Up),
            event::KeyCode::Down => Some(Action::Down),
            event::KeyCode::Home => None,
            event::KeyCode::End => None,
            event::KeyCode::PageUp => None,
            event::KeyCode::PageDown => None,
            event::KeyCode::Tab => Some(Action::Tab),
            event::KeyCode::BackTab => None,
            event::KeyCode::Delete => todo!(),
            event::KeyCode::Insert => None,
            event::KeyCode::F(_) => None,
            event::KeyCode::Char(c) => Some(Action::Char(c)),
            event::KeyCode::Null => None,
            event::KeyCode::Esc => Some(Action::Esc),
            event::KeyCode::CapsLock => None,
            event::KeyCode::ScrollLock => None,
            event::KeyCode::NumLock => None,
            event::KeyCode::PrintScreen => None,
            event::KeyCode::Pause => None,
            event::KeyCode::Menu => None,
            event::KeyCode::KeypadBegin => None,
            event::KeyCode::Media(_) => None,
            event::KeyCode::Modifier(_) => None,
        };
        Ok(action)
    }

    pub fn handle_mouse_event(&mut self, mouse: event::MouseEvent) -> Result<Option<Action>> {
        let _ = mouse;
        Ok(None)
    }

    pub fn update(&mut self, action: Action) -> Result<Option<Action>> {
        if action == Action::Save {self.collection.save_to_file(utils::save_file_location());}
        match self.screen {
            Screen::Home => self.home.update(&mut self.collection, action),
        }
    }

    pub fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        match self.screen {
            Screen::Home => self.home.draw(&self.collection, frame, area),
        }
    }
}
