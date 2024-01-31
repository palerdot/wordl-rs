use std::{io, panic};

use color_eyre::Result;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::events::EventHandler;
use crate::ui;
use crate::wordle::model::Model;

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

// Representation of terminal user interface
// Sets up terminal and handles events
pub struct Tui {
    // interface to the terminal
    terminal: CrosstermTerminal,
    // event handler
    pub events: EventHandler,
}

impl Tui {
    // constructs new instance of 'Tui'
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    // Initializes the terminal interface
    // enables raw mode and sets terminal properties
    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // define custom panic hook to reset terminal properties
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    // Resets the terminal interface
    // also used by panic hook to revert terminal properties during unexpected error
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }

    // exits the terminal interface
    // disables raw mode and reverts back terminal properties
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;

        Ok(())
    }

    // draw the terminal interface
    pub fn draw(&mut self, model: &mut Model) -> Result<()> {
        self.terminal.draw(|frame| ui::view(model, frame))?;
        Ok(())
    }
}
