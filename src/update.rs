use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use wordle::model::{Message, Model, RunningState};

use crate::events::EventHandler;
use crate::wordle;
use crate::wordle::model::GameResult;
// use crate::wordle::model::{LetterState, LetterStatus};

pub async fn update(model: &mut Model, msg: Message, event_handler: &EventHandler) {
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
        Message::CalculateStart => {
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

            event_handler
                .send_delayed_message(314, Message::AnimateGuess(0, guess))
                .await;
        }

        Message::AnimateGuess(guess_position, guess) => {
            let latest_position = model.guesses.len() - 1;

            if let Some(current) = model.guesses.get_mut(latest_position) {
                if let Some(guess_letter) = guess.get(guess_position) {
                    current.push(guess_letter.clone());
                    // once last letter is animated don't send delayed event
                    let is_last_letter = guess_position + 1 == guess.len();

                    if is_last_letter {
                        event_handler
                            .send_delayed_message(0, Message::CalculateEnd(guess))
                            .await;
                    } else {
                        event_handler
                            .send_delayed_message(
                                515,
                                Message::AnimateGuess(guess_position + 1, guess),
                            )
                            .await;
                    }
                }
            }
        }
        Message::CalculateEnd(guess) => {
            // update keyboard hint
            wordle::utils::update_keyboard_hints(&mut model.keyboard_hints, guess.clone());

            let is_correct_guess = wordle::utils::is_correct_guess(guess.clone());
            let is_attempts_over = model.guesses.len() == 6;
            let is_over = is_correct_guess || is_attempts_over;

            if is_over {
                model.running_state = RunningState::Over(if is_correct_guess {
                    GameResult::CorrectGuess
                } else {
                    GameResult::WrongGuess
                });
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

pub fn handle_key_event(key_event: KeyEvent) -> Option<Message> {
    match key_event.code {
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
        KeyCode::Enter => Some(Message::CalculateStart),
        _code => None,
    }
}
