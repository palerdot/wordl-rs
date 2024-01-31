// ref: https://ratatui.rs/tutorials/counter-app/multiple-files/event/
use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

use crate::wordle::model::Message;

/// terminal events
#[derive(Clone, Debug)]
pub enum Event {
    /// Terminal tick
    Tick,
    /// Keyboard events
    Key(KeyEvent),
    /// Handle state updates directly
    StateUpdate(Message),
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
    sender: mpsc::UnboundedSender<Event>,
    /// event receiver channel
    receiver: mpsc::UnboundedReceiver<Event>,
    #[allow(dead_code)]
    /// event handler thread
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// constructs new instance of 'Event Handler'
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        let _sender = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);

            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = tick_delay => {
                        _sender.send(Event::Tick).unwrap();
                    }
                    Some(Ok(evt)) = crossterm_event => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                // handle only press events
                                if key.kind == crossterm::event::KeyEventKind::Press {
                                    _sender.send(Event::Key(key)).unwrap();
                                }
                            },

                            CrosstermEvent::Mouse(mouse) => {
                                _sender.send(Event::Mouse(mouse)).unwrap();
                            },

                            CrosstermEvent::Resize(w,h) => {
                                _sender.send(Event::Resize(w,h)).unwrap();
                            },

                            CrosstermEvent::FocusGained => {},
                            CrosstermEvent::FocusLost => {},
                            CrosstermEvent::Paste(_) => {},
                        }
                    }
                };
            }
        });

        Self {
            sender,
            receiver,
            handler,
        }
    }

    // Receives next event from the handler thread
    pub async fn next(&mut self) -> color_eyre::Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(color_eyre::eyre::eyre!("Unable to get event"))
    }

    // send delayed state update message
    pub async fn send_delayed_message(&self, delay: u64, message: Message) {
        let sender = self.sender.clone();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(delay)).await;
            sender.send(Event::StateUpdate(message)).unwrap();
        });
    }
}
