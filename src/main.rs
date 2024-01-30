// ref: https://ratatui.rs/concepts/application-patterns/the-elm-architecture/

use ratatui::{backend::CrosstermBackend, Terminal};

use events::{Event, EventHandler};
use tui::Tui;
use update::{handle_key_event, update};
use wordle::model::{Model, RunningState};

pub mod events;
pub mod tui;
pub mod ui;
pub mod update;
pub mod wordle;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let result = run().await;
    result?;

    Ok(())
}

async fn run() -> color_eyre::Result<()> {
    // TEA - The ELM architecture
    // Model | Update | View
    let mut model = Model::default();

    if let Some(wordle) = wordle::data::get_wordle() {
        model.wordle = wordle;
    }

    // init terminal
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // set up terminal and listen for events
    while model.running_state != RunningState::Done {
        // render user interface
        tui.draw(&mut model)?;
        // Handle events (we will sending tick events periodically)
        match tui.events.next().await? {
            Event::Tick => {}
            Event::Key(key_event) => {
                if let Some(message) = handle_key_event(key_event) {
                    update(&mut model, message, &tui.events).await;
                }
            }
            Event::StateUpdate(message) => {
                update(&mut model, message, &tui.events).await;
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // exit the terminal
    tui.exit()?;
    Ok(())
}
