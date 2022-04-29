pub mod events;
pub mod keys;

pub enum InputEvent {
    Input(keys::Key),
    Tick,
}
