// use crate::start_ui
use std::{cell::RefCell, io, rc::Rc};
use termtris::{app::App, start_ui};

fn main() -> Result<(), io::Error> {
    let app = Rc::new(RefCell::new(App::new()));
    start_ui(app)?;
    Ok(())
}
