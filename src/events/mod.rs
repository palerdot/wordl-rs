// ref: https://ratatui.rs/tutorials/counter-app/multiple-files/event/
use color_eyre::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

/// terminal events
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick
    Tick,
    /// Keyboard events
    Key(KeyEvent),
    /// Mouse events
    Mouse(MouseEvent),
    /// resize events
    Resize(u16, u16),
}

/// Terminal event handler
#[derive(Debug)]
pub struct EventHandler {
    /// event sender channel
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    /// event receiver channel
    receiver: mpsc::Receiver<Event>,
    /// event handler thread
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// constructs new instance of 'Event Handler'
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            // spawns a new thread in the background
            // https://stackoverflow.com/a/75840352/1410291
            // Only difference between native OS threads and tokio is thread pool management
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(()) // ignore other events type like release events
                                }
                            }
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        // send tick event periodically every tick rate
                        sender
                            .send(Event::Tick)
                            .expect("porumai ... unable to send tick event");
                        // update last tick
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            handler,
        }
    }

    // Receive next event from the handler thread
    // This will block the current thread if no data is available (and possible to send more data)
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
