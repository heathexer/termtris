use crossterm::event;
use std::{
    sync::{
        mpsc::{channel, Receiver, RecvError, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use crate::inputs::InputEvent;

pub struct Events {
    tick_rate: Arc<Mutex<Duration>>,
    rx: Receiver<InputEvent>,
    tx: Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = channel();

        Events {
            tick_rate: Arc::new(Mutex::new(tick_rate)),
            rx,
            tx,
        }
    }

    pub fn start(&self) {
        let event_tx = self.tx.clone();
        let tick_rate = Arc::clone(&self.tick_rate);

        thread::spawn(move || loop {
            let tr = tick_rate.lock().unwrap().clone();
            drop(tr);

            if event::poll(tr).unwrap() {
                if let event::Event::Key(key_event) = event::read().unwrap() {
                    let key = key_event.into();
                    event_tx.send(InputEvent::Input(key)).unwrap();
                }
            } else {
                event_tx.send(InputEvent::Tick).unwrap();
            }
        });
    }

    // Attempts to read an event, is blocking
    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }

    pub fn update_tick_rate(&mut self, new_rate: Duration) {
        let mut tr = self.tick_rate.lock().unwrap();
        *tr = new_rate;
    }
}
