use std::{cell::RefCell, io, rc::Rc, time::Duration};

use inputs::{events::Events, InputEvent};
use tui::{backend::CrosstermBackend, Terminal};

pub mod app;
pub mod game;
pub mod inputs;

use app::{ui, App, AppReturn};

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<(), io::Error> {
    // setup terminal with Crossterm backend
    let stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(500);
    let events = Events::new(tick_rate);

    loop {
        let mut app = app.borrow_mut();

        // Render
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // Handle inputs
        let result = match events.next().unwrap() {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };

        if result == AppReturn::Exit {
            break;
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
