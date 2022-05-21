pub mod app;
pub mod game;
pub mod inputs;

use app::App;
use app::{ui, AppReturn};
use std::io;
use std::time::Duration;

use color_eyre::eyre::Result;

use inputs::{events::Events, InputEvent};
use tui::{backend::CrosstermBackend, Terminal};

pub fn start_ui(mut app: App) -> Result<()> {
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

    loop {
        // Render
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // Handle inputs
        let result = match events.next().unwrap() {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };

        match result {
            AppReturn::Continue => {}
            AppReturn::UpdateSpeed => events.update_tick_rate(app.get_tick_delay()),
            AppReturn::Exit => break,
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
