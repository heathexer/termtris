pub mod actions;
pub mod state;
pub mod ui;

use std::time::Duration;

use self::{
    actions::{Action, Actions},
    state::AppState,
};

use crate::{game::Game, inputs::keys::Key};

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
    UpdateSpeed,
}

pub struct App<'a> {
    actions: Actions,
    _state: AppState,
    game: Game<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let actions = Actions::from(Action::iterator().cloned().collect::<Vec<_>>());
        let state = AppState::initialized();
        let game = Game::new();
        App {
            actions,
            _state: state,
            game,
        }
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub fn get_tick_delay(&self) -> Duration {
        self.game.score.level.get_tick_delay()
    }

    // Handle an input
    pub fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                Action::Quit => AppReturn::Exit,
                Action::ShiftLeft => {
                    self.game.move_left();
                    AppReturn::Continue
                }
                Action::ShiftRight => {
                    self.game.move_right();
                    AppReturn::Continue
                }
                Action::RotateLeft => {
                    self.game.rotate_left();
                    AppReturn::Continue
                }
                Action::RotateRight => {
                    self.game.rotate_right();
                    AppReturn::Continue
                }
                Action::HardDrop => {
                    self.game.hard_drop();
                    AppReturn::Continue
                }
                Action::SoftDrop => {
                    self.game.soft_drop();
                    AppReturn::Continue
                }
                Action::Hold => {
                    self.game.hold();
                    AppReturn::Continue
                }
            }
        } else {
            AppReturn::Continue
        }
    }

    // Handle a tick
    pub fn update_on_tick(&mut self) -> AppReturn {
        self.game.move_down();
        AppReturn::UpdateSpeed
    }
}
