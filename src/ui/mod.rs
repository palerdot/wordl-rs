use ratatui::{
    prelude::{Alignment, Color, Frame, Rect, Span},
    style::{Style, Stylize},
    widgets::{block::Position, Block, Paragraph, Wrap},
};

use crate::wordle::model::{GameResult, LetterState, Model, RunningState};

mod grid;
mod keyboard;
mod layout;

// [ELM VIEW] view is a function of model
pub fn view(model: &mut Model, f: &mut Frame) {
    let block = Block::default()
        // .title(format!("{}", model.wordle,))
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::White))
        .style(Style::default().white().bg(Color::Rgb(0, 0, 0)))
        .title(get_status(model))
        .title_position(Position::Top);

    let master_layout = layout::master_layout(f);

    let common_text =
        "Type and enter the guess. Backspace to clear. Ctrl-N for new wordle. Esc/Ctrl-C to quit."
            .to_string();
    let help_text = if master_layout.len() == 1 {
        format!(
            "{}. Check https://github.com/palerdot/wordl-rs for more info.",
            common_text
        )
    } else {
        common_text
    };
    let help_text_block = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .style(Style::new().fg(Color::Rgb(189, 189, 189)));

    let has_min_height = master_layout.len() == 2;

    // top layout
    f.render_widget(block, master_layout[0]);
    // main grid
    grid::draw(f, master_layout[0], model);
    // status text
    f.render_widget(
        help_text_block,
        Rect {
            x: 0,
            y: master_layout[0].height - if has_min_height { 1 } else { 3 },
            height: 2,
            width: f.size().width - 5, // ..master_layout[0]
        },
    );

    if has_min_height {
        // keyboard layout
        keyboard::draw(f, master_layout[1], &mut model.keyboard_hints);
    }
}

fn get_status(model: &mut Model) -> Span {
    let step = model.guesses.len();

    match &model.running_state {
        RunningState::Waiting => Span::styled(
            format!("{}/6: Enter your guess", step),
            Style::default()
                .fg(Color::Rgb(189, 189, 189))
                .bg(Color::Rgb(0, 0, 0)),
        ),
        RunningState::Calculating => Span::styled(
            format!("{}/6: Checking", step),
            Style::default()
                .fg(Color::Rgb(189, 189, 189))
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
                        Color::Rgb(255, 95, 135)
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
