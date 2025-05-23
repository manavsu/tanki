use strum::Display;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Screen {
    Home,
    Practice(uuid::Uuid),
}
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
    CtrlSpace,
    Up,
    Down,
    Right,
    Left,
    Space,
    Save,
    Load,
    Screen(Screen),
}
