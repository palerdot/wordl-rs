use ratatui::{
    prelude::{Alignment, Color, Frame, Span},
    style::{Style, Stylize},
    widgets::Block,
};

use crate::wordle::model::{GameResult, LetterState, Model, RunningState};

mod grid;
mod keyboard;
mod layout;

// [ELM VIEW] view is a function of model
pub fn view(model: &mut Model, f: &mut Frame) {
    let block = Block::default()
        .title(format!("WORDL: {}", model.wordle,))
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::White))
        .style(Style::default().white().bg(Color::Rgb(0, 0, 0)))
        .title(get_status(model));

    let master_layout = layout::master_layout(f);

    // top layout
    f.render_widget(block, master_layout[0]);
    // main grid
    grid::draw(f, master_layout[0], model);

    if master_layout.len() == 2 {
        // keyboard layout
        keyboard::draw(f, master_layout[1], &mut model.keyboard_hints);
    }
}

fn get_status(model: &mut Model) -> Span {
    let step = model.guesses.len();

    match &model.running_state {
        RunningState::Waiting => Span::styled(
            format!("{}/6 - Enter your guess", step),
            Style::default()
                .fg(Color::Rgb(255, 255, 0))
                .bg(Color::Rgb(0, 0, 0)),
        ),
        RunningState::Calculating => Span::styled(
            format!("{}/6 - Checking", step),
            Style::default()
                .fg(Color::Rgb(255, 255, 0))
                .bg(Color::Rgb(0, 0, 0)),
        ),
        RunningState::Over(result) => {
            let is_correct = if *result == GameResult::CorrectGuess {
                true
            } else {
                false
            };

            let answer = model.wordle.to_uppercase().to_string();

            Span::styled(
                if is_correct {
                    "Correct 😇".into()
                } else {
                    format!("{} is the correct word", answer)
                },
                Style::default()
                    .fg(if is_correct {
                        Color::Rgb(0, 255, 0)
                    } else {
                        Color::Rgb(255, 0, 0)
                    })
                    .bg(Color::Rgb(0, 0, 0)),
            )
        }
        _ => Span::styled(
            "",
            Style::default()
                .fg(Color::Rgb(0, 255, 0))
                .bg(Color::Rgb(0, 0, 0)),
        ),
    }
}

// helper function to get grid color
pub fn get_grid_color(letter_state: LetterState) -> Color {
    match letter_state {
        LetterState::Correct => Color::Rgb(0, 135, 0),
        LetterState::Incorrect => Color::Rgb(215, 175, 0),
        LetterState::NotPresent => Color::Rgb(88, 88, 88),
        LetterState::Unknown => Color::Rgb(48, 48, 48),
    }
}
