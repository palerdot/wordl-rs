use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
// use std::sync::{Arc, Mutex};
// use std::thread;
use std::time::Duration;
use wordle::model::{Message, Model, RunningState};

use crate::wordle;
// use crate::wordle::model::{LetterState, LetterStatus};

fn update_model(model: &mut Model, msg: Message) {
    match msg {
        Message::Listen(letter) => {
            // we will listen only if it is in waiting state
            if model.running_state != RunningState::Waiting {
                return;
            }

            // we have to handle the letter only if active guess is incomplete
            if model.active_guess.len() < 5 {
                let updated_guess = format!("{}{}", model.active_guess, letter.to_lowercase())
                    .to_lowercase()
                    .into();
                model.active_guess = updated_guess;
            }
        }
        Message::Calculate => {
            // start calculation only if the guess has 5 letters
            if model.active_guess.len() != 5 {
                return;
            }

            // first change state to calculating
            model.running_state = RunningState::Calculating;
            let guess =
                wordle::utils::check(model.wordle.to_string(), model.active_guess.to_string());
            // reset active guess
            model.active_guess = "".into();
            let latest_position = model.guesses.len();
            // // insert empty vector
            model.guesses.insert(latest_position, Vec::new());

            for guess_letter in &guess {
                if let Some(current) = model.guesses.get_mut(latest_position) {
                    // sleep and insert for reveal animation
                    std::thread::sleep(Duration::from_millis(314));
                    current.push(guess_letter.clone());
                }
            }

            let is_correct_guess = wordle::utils::is_correct_guess(guess.clone());
            let is_attempts_over = model.guesses.len() == 6;
            let is_over = is_correct_guess || is_attempts_over;

            if is_over {
                model.running_state = RunningState::Over;
            } else {
                model.running_state = RunningState::Waiting;
            }
        }
        Message::Erase => {
            // we will listen only if it is in waiting state
            if model.running_state != RunningState::Waiting {
                return;
            }

            if model.active_guess.len() > 0 {
                model.active_guess.pop();
            }
        }
        Message::Reset => {}
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
    }
}

pub fn update(model: &mut Model, key_event: KeyEvent) {
    let msg = match key_event.code {
        // https://ratatui.rs/templates/async/config-rs/
        KeyCode::Char('n') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Message::Reset)
        }
        // KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Message::Quit),
        KeyCode::Esc => Some(Message::Quit),
        KeyCode::Char(letter) => {
            if letter.is_ascii_alphabetic() {
                Some(Message::Listen(letter))
            } else {
                None
            }
        }
        KeyCode::Backspace | KeyCode::Delete => Some(Message::Erase),
        KeyCode::Enter => Some(Message::Calculate),
        _code => None,
    };

    if let Some(message) = msg {
        update_model(model, message);
    }
}
