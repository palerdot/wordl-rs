// use ratatui::widgets::block::Title;
use crate::wordle::model::{LetterState, LetterStatus, Model};
use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, rect: Rect, model: &mut Model) {
    // draw empty grid
    for row in 0..=5 {
        for column in 0..5 {
            render(
                frame,
                rect,
                Grid {
                    row,
                    column,
                    letter_status: LetterStatus {
                        letter: ' ',
                        status: LetterState::Unknown,
                    },
                },
            );
        }
    }

    for (row, guess) in model.guesses.iter().enumerate() {
        for (column, guess_status) in guess.into_iter().enumerate() {
            let grid = Grid {
                row,
                column,
                letter_status: guess_status.clone(),
            };
            render(frame, rect, grid);
        }
    }

    // render active guess
    if model.active_guess.len() > 0 {
        let row = model.guesses.len();
        for (column, letter) in model.active_guess.chars().enumerate() {
            let grid = Grid {
                row,
                column,
                letter_status: LetterStatus {
                    letter,
                    status: LetterState::Unknown,
                },
            };
            render(frame, rect, grid);
        }
    }
}

struct Grid {
    row: usize,
    column: usize,
    letter_status: LetterStatus,
}

fn render(frame: &mut Frame, rect: Rect, grid: Grid) {
    let width = 5;
    let height = 3;
    let row = grid.row;
    let column = grid.column;

    let letter = grid.letter_status.letter.to_uppercase().to_string();
    let offset: u16 = u16::from((rect.width - rect.left()) / 2) - 12;
    let x = rect.left() + u16::from(column as u16) * width + offset;
    let area = Rect {
        x,
        y: rect.top() + (row as u16 * height) + 1,
        width,
        height,
    };

    let bg = match grid.letter_status.status {
        LetterState::Correct => Color::Rgb(0, 135, 0),
        LetterState::Incorrect => Color::Rgb(215, 0, 0),
        LetterState::Unknown => Color::Rgb(48, 48, 48),
    };

    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::QuadrantOutside)
        .border_style(Style::new().fg(Color::Rgb(0, 0, 0)))
        // .padding(Padding::new(1, 1, 1, 1))
        // .style(Style::new().white().on_black().bg(Color::Rgb(0, 0, 0)));
        .style(Style::new().white().on_black().bg(bg).bold());

    frame.render_widget(
        Paragraph::new(format!("{}", letter.to_string()))
            .block(block)
            .style(Style::new().white().on_black().bg(bg))
            .alignment(Alignment::Center),
        area,
    );
}
