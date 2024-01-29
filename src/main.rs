// ref: https://ratatui.rs/concepts/application-patterns/the-elm-architecture/

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use rand::seq::SliceRandom;
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;
use wordle::model::{Message, Model, RunningState};

pub mod tui;
pub mod wordle;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut model = Model::default();

    let guesses = wordle::data::valid_guesses();
    if let Some(guess) = guesses.choose(&mut rand::thread_rng()) {
        model.wordle = guess.to_string();
    }

    while model.running_state != RunningState::Done {
        // keep on Rendering the current view
        terminal.draw(|f| view(&mut model, f))?;

        // handle events and map to message
        let mut current_msg = handle_event(&model)?;

        // update if there is a message
        while current_msg.is_some() {
            match current_msg.unwrap() {
                Message::SleepFor(sleep_time, next_msg) => {
                    current_msg = update(&mut model, *next_msg);
                    std::thread::sleep(Duration::from_millis(sleep_time));
                }
                msg => {
                    current_msg = update(&mut model, msg);
                }
            }
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Listen(letter) => {
            // we will listen only if it is in waiting state
            if model.running_state != RunningState::Waiting {
                return None;
            }

            // we have to handle the letter only if active guess is incomplete
            if model.active_guess.len() < 5 {
                let updated_guess = format!("{}{}", model.active_guess, letter.to_lowercase())
                    .to_lowercase()
                    .into();
                model.active_guess = updated_guess;
            }
        }
        Message::Erase => {
            // we will listen only if it is in waiting state
            if model.running_state != RunningState::Waiting {
                return None;
            }

            if model.active_guess.len() > 0 {
                model.active_guess.pop();
            }
        }
        Message::CalculateStart => {
            // start calculation only if the guess has 5 letters
            if model.active_guess.len() != 5 {
                return None;
            }

            // first change state to calculating
            model.running_state = RunningState::Calculating;
            let guess =
                wordle::utils::check(model.wordle.to_string(), model.active_guess.to_string());
            // reset active guess
            model.active_guess = "".into();
            let latest_position = model.guesses.len();
            // insert empty vector
            model.guesses.insert(latest_position, Vec::new());

            return Some(Message::Animate(0, guess));
        }
        Message::Animate(current_index, guess) => {
            if current_index < guess.len() {
                let latest_guess_position = model.guesses.len() - 1;
                if let Some(current_guess) = model.guesses.get_mut(latest_guess_position) {
                    let guess_letter = guess.get(current_index).unwrap();
                    // sleep and insert for reveal animation
                    current_guess.push(guess_letter.clone());

                    return Some(Message::SleepFor(
                        314,
                        Box::new(Message::Animate(current_index + 1, guess)),
                    ));
                    // return Some(Message::Animate(current_index + 1, guess));
                    //
                    // *current_guess = guess.clone();

                    // return Some(Message::CalculateEnd);

                    // return Some(Message::SleepFor(1314, Box::new(Message::CalculateEnd)));
                } else {
                    return Some(Message::CalculateEnd);
                }
            }
        }
        //  sleep for is intended for render loop
        Message::SleepFor(..) => {}

        Message::CalculateEnd => {
            let guess = model.guesses.last().unwrap();

            let is_correct_guess = wordle::utils::is_correct_guess(guess.clone());
            let is_attempts_over = model.guesses.len() == 6;
            let is_over = is_correct_guess || is_attempts_over;

            if is_over {
                model.running_state = RunningState::Over;
            } else {
                model.running_state = RunningState::Waiting;
            }
        }
        Message::Reset => {}
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
    }

    None
}

fn view(model: &mut Model, f: &mut Frame) {
    let block = Block::default()
        // .title("Block")
        .title(format!("Porumai {}", model.wordle))
        .title_alignment(Alignment::Center)
        // .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::White))
        // .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Rgb(0, 0, 0)));

    let master_layout = tui::layout::master_layout(f);

    // top layout
    f.render_widget(block, master_layout[0]);
    // main grid
    tui::grid::draw(f, master_layout[0], model);

    if master_layout.len() == 2 {
        // keyboard layout
        tui::keyboard::draw(f, master_layout[1]);
    }
}

// Convert event to message
// model is not needed now; but we might needed later
fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }

    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        // https://ratatui.rs/templates/async/config-rs/
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Message::Reset),
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
