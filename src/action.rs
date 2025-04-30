use strum::Display;

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
    Char(char),
    Esc,
    Enter,
    Tab,
    Backspace,
    Up,
    Down,
    Right,
    Left,
    Space,
}
