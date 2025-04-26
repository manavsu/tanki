use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, components::component::Component};

use super::home::Home;

pub struct Base {
    tx: UnboundedSender<Action>,
    home: Home,
}

impl Base {
    pub fn new(tx: UnboundedSender<Action>) -> Self {
        let tx_clone = tx.clone();
        Self { tx, home: Home::new(tx_clone.clone()) }
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
        self.home.draw(frame, area)
    }
}
