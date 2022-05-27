pub mod actions;
pub mod game_ui;
pub mod menu_actions;
pub mod start_ui;

use std::time::Duration;

use self::{
    actions::{Action, Actions},
    menu_actions::MenuAction,
};

use crate::{game::Game, inputs::keys::Key, AppState};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum AppReturn {
    Exit,
    Continue,
    UpdateSpeed,
    Transition(AppState),
}

// Struct to store the overall app state and process input events
// State is pretty much unused for now, but planned for use when pause/menus are added
pub struct App<'a> {
    actions: Actions,
    menu_actions: MenuActions,
    _state: AppState,
    game: Game<'a>,
}

// I can't think of a reason to have a Default impl for any structs in this project but I'm open to any reasons
#[allow(clippy::new_without_default)]
impl<'a> App<'a> {
    pub fn new() -> Self {
        let actions = Actions::from(Action::iterator().cloned().collect::<Vec<_>>());
        let state = AppState::new();
        let game = Game::new();
        App {
            actions,
            _state: state,
            game,
        }
    }

    fn actions(&self) -> &Actions {
        &self.actions
    }

    pub(crate) fn get_tick_delay(&self) -> Duration {
        self.game.score.level.get_tick_delay()
    }

    // Handle an input
    pub(crate) fn do_action(&mut self, key: Key) -> AppReturn {
        match self._state {
            AppState::MainMenu => self.do_menu_action(self.menu_actions.find(key)),
            AppState::Running => self.do_game_action(self.actions.find(key)),
        }
    }

    pub(crate) fn do_menu_action(&mut self, action: Option<&MenuAction>) -> AppReturn {
        if let Some(a) = action {
            match a {
                MenuAction::Exit => todo!(),
                MenuAction::Previous => todo!(),
                MenuAction::Next => todo!(),
                MenuAction::Confirm => todo!(),
            }
        }

        AppReturn::Continue
    }

    pub(crate) fn do_game_action(&mut self, action: Option<&Action>) -> AppReturn {
        if let Some(a) = action {
            match a {
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
    pub(crate) fn update_on_tick(&mut self) -> AppReturn {
        if self.game.move_down() {
            AppReturn::UpdateSpeed
        } else {
            AppReturn::Transition(AppState::MainMenu)
        }
    }
}
