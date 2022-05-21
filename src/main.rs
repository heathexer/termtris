pub mod app;
pub mod game;
pub mod inputs;

use app::{game_ui, AppReturn};
use app::{start_ui, App};
use std::io::{self, Stdout};
use std::time::Duration;
use tui::backend::Backend;

use color_eyre::eyre::Result;

use inputs::{events::Events, InputEvent};
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum AppState {
    MainMenu,
    Running,
}

impl AppState {
    fn new() -> AppState {
        AppState::MainMenu
    }

    fn get_function<B: Backend>(
        s: Self,
    ) -> fn(&mut Events, &mut Terminal<B>, &mut App) -> Result<AppReturn> {
        match s {
            AppState::MainMenu => start_menu,
            AppState::Running => run_game,
        }
    }
}

fn run_game<B: Backend>(
    events: &mut Events,
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<AppReturn> {
    // Render
    terminal.draw(|f| game_ui::draw(f, app))?;

    // Handle inputs
    let result = match events.next()? {
        InputEvent::Input(key) => app.do_action(key),
        InputEvent::Tick => app.update_on_tick(),
    };

    Ok(result)
}

fn start_menu<B: Backend>(
    events: &mut Events,
    terminal: &mut Terminal<B>,
    _app: &mut App,
) -> Result<AppReturn> {
    // Render
    terminal.draw(|f| start_ui::draw(f))?;

    // Handle inputs
    let result = match events.next().unwrap() {
        InputEvent::Input(_key) => AppReturn::Transition(AppState::Running),
        InputEvent::Tick => AppReturn::Continue,
    };

    Ok(result)
}

fn start_ui(mut app: App) -> Result<()> {
    // setup terminal with Crossterm backend
    let stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(100);

    let mut events = Events::new(tick_rate);
    events.start();

    let mut f = start_menu
        as fn(&mut Events, &mut Terminal<CrosstermBackend<Stdout>>, &mut App) -> Result<AppReturn>;

    loop {
        match f(&mut events, &mut terminal, &mut app)? {
            AppReturn::Exit => break,
            AppReturn::Continue => continue,
            AppReturn::UpdateSpeed => events.update_tick_rate(app.get_tick_delay()),
            AppReturn::Transition(s) => f = AppState::get_function(s),
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let app = App::new();

    start_ui(app)
}
