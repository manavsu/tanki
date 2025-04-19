use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::components::component::Component;

pub struct Home {
    tx: UnboundedSender<Action>,
    state: ListState,
    num_options: usize,
}

impl Home {
    pub fn new(tx: UnboundedSender<Action>) -> Self {
        Self { tx, state: ListState::default(), num_options: 0 }
    }
}

impl Component for Home {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Up | KeyCode::Down => {
                update_list_selection(key.code, &mut self.state, self.num_options);
            }
            KeyCode::Enter => {
                if let Some(selected) = self.state.selected() {
                    return Ok(None); // TODO
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let items = vec![ListItem::new(""), ListItem::new("Option 2"), ListItem::new("Option 3")];
        self.num_options = items.len();

        let list = List::new(items)
            .block(Block::bordered())
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        frame.render_stateful_widget(list, area, &mut self.state);
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
