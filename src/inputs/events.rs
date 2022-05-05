use crossterm::event;
use std::{
    sync::mpsc::{channel, Receiver, RecvError, Sender},
    thread,
    time::Duration,
};

use crate::inputs::InputEvent;

pub struct Events {
    rx: Receiver<InputEvent>,
    _tx: Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = channel();

        let event_tx = tx.clone();
        thread::spawn(move || loop {
            if event::poll(tick_rate).unwrap() {
                if let event::Event::Key(key_event) = event::read().unwrap() {
                    let key = key_event.into();
                    event_tx.send(InputEvent::Input(key)).unwrap();
                }
            } else {
                event_tx.send(InputEvent::Tick).unwrap();
            }
        });

        Events { rx, _tx: tx }
    }

    // Attempts to read an event, is blocking
    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}
