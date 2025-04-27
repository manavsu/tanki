use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, components::component::Component};

use super::home::Home;

pub enum Screen {
    Home,
}

pub struct Base {
    tx: UnboundedSender<Action>,
    home: Home,
    screen: Screen,
}

impl Base {
    pub fn new(tx: UnboundedSender<Action>) -> Self {
        let tx_clone = tx.clone();
        Self { tx, home: Home::new(tx_clone.clone()), screen: Screen::Home }
    }
}

impl Component for Base {
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> color_eyre::eyre::Result<Option<crate::action::Action>> {
        self.home.handle_key_event(key)
    }

    fn handle_mouse_event(&mut self, mouse: crossterm::event::MouseEvent) -> color_eyre::eyre::Result<Option<crate::action::Action>> {
        self.home.handle_mouse_event(mouse)
    }

    fn update(&mut self, action: crate::action::Action) -> color_eyre::eyre::Result<Option<crate::action::Action>> {
        self.home.update(action)
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> color_eyre::eyre::Result<()> {
        match self.screen {
            Screen::Home => self.home.draw(frame, area),
        }
    }
}
