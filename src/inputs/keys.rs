use std::fmt::{self, Display};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BaseKey {
    None,
    Char(char),
    Enter,
    Tab,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Key {
    Plain(BaseKey),
    Ctrl(BaseKey),
}

impl Display for BaseKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseKey::None => write!(f, ""),
            BaseKey::Char(' ') => write!(f, "Space"),
            BaseKey::Char(c) => write!(f, "{}", c),
            BaseKey::Enter => write!(f, "Enter"),
            BaseKey::Tab => write!(f, "Tab"),
            BaseKey::Left => write!(f, "←"),
            BaseKey::Right => write!(f, "→"),
            BaseKey::Up => write!(f, "↑"),
            BaseKey::Down => write!(f, "↓"),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Key::Plain(key) => write!(f, "{key}"),
            Key::Ctrl(key) => write!(f, "Ctrl+{key}"),
        }
    }
}

impl From<KeyEvent> for Key {
    fn from(key_event: KeyEvent) -> Self {
        let base_key = match key_event.code {
            KeyCode::Char(c) => BaseKey::Char(c),
            KeyCode::Enter => BaseKey::Enter,
            KeyCode::Tab => BaseKey::Tab,
            KeyCode::Left => BaseKey::Left,
            KeyCode::Right => BaseKey::Right,
            KeyCode::Up => BaseKey::Up,
            KeyCode::Down => BaseKey::Down,
            _ => BaseKey::None,
        };

        match key_event.modifiers {
            KeyModifiers::CONTROL => Key::Ctrl(base_key),
            _ => Key::Plain(base_key),
        }
    }
}
