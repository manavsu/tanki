use strum::Display;

use crate::components::home;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
}
