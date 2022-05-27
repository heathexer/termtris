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
        let tick_event_tx = self.tx.clone();
        let input_event_tx = self.tx.clone();
        let tick_rate = Arc::clone(&self.tick_rate);

        thread::spawn(move || loop {
            // Needs to be cloned out so the lock isn't held even though Clippy complains about it
            #[allow(clippy::clone_on_copy)]
            let tr = tick_rate.lock().unwrap().clone();

            tick_event_tx.send(InputEvent::Tick).unwrap();
            thread::sleep(tr);
        });

        thread::spawn(move || loop {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                let key = key_event.into();
                input_event_tx.send(InputEvent::Input(key)).unwrap();
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
