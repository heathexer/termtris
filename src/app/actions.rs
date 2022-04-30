use std::fmt::{self, Display};

use crate::inputs::keys::{BaseKey, Key};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    RotateLeft,
    RotateRight,
    ShiftLeft,
    ShiftRight,
    HardDrop,
    Hold,
}

impl Action {
    // Iterator over all available actions
    pub fn iterator() -> std::slice::Iter<'static, Action> {
        static ACTIONS: [Action; 7] = [
            Action::Quit,
            Action::RotateLeft,
            Action::RotateRight,
            Action::ShiftLeft,
            Action::ShiftRight,
            Action::HardDrop,
            Action::Hold,
        ];
        ACTIONS.iter()
    }

    // List of keys/combinations associated with an action
    pub fn keys(&self) -> Vec<Key> {
        match self {
            Action::Quit => vec![Key::Ctrl(BaseKey::Char('c'))],
            Action::ShiftLeft => vec![Key::Plain(BaseKey::Char('a')), Key::Plain(BaseKey::Left)],
            Action::ShiftRight => vec![Key::Plain(BaseKey::Char('d')), Key::Plain(BaseKey::Right)],
            Action::RotateLeft => vec![
                Key::Plain(BaseKey::Char('q')),
                Key::Plain(BaseKey::Char(',')),
            ],
            Action::RotateRight => vec![Key::Plain(BaseKey::Char('e')), Key::Plain(BaseKey::Up)],
            Action::HardDrop => vec![Key::Plain(BaseKey::Char(' '))],
            Action::Hold => vec![Key::Plain(BaseKey::Char('c'))],
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Quit => write!(f, "Quit"),
            Action::RotateLeft => write!(f, "Rotate Left"),
            Action::RotateRight => write!(f, "Rotate Right"),
            Action::ShiftLeft => write!(f, "Move Left"),
            Action::ShiftRight => write!(f, "Move Right"),
            Action::HardDrop => write!(f, "Hard Drop"),
            Action::Hold => write!(f, "Hold"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    // Find the associated action for a given input
    pub fn find(&self, key: Key) -> Option<&Action> {
        for action in Action::iterator() {
            if action.keys().contains(&key) {
                return Some(action);
            }
        }
        None
    }
}

impl From<Vec<Action>> for Actions {
    fn from(actions: Vec<Action>) -> Self {
        Self(actions)
    }
}
