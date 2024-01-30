use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::Block,
};

use crate::wordle::model::Model;

mod grid;
mod keyboard;
mod layout;

// [ELM VIEW] view is a function of model
pub fn view(model: &mut Model, f: &mut Frame) {
    let block = Block::default()
        // .title("Block")
        .title(format!(
            "Porumai {} {} {}",
            model.wordle,
            model.guesses.len(),
            model.active_guess
        ))
        .title_alignment(Alignment::Center)
        // .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::White))
        // .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Rgb(0, 0, 0)));

    let master_layout = layout::master_layout(f);

    // top layout
    f.render_widget(block, master_layout[0]);
    // main grid
    grid::draw(f, master_layout[0], model);

    if master_layout.len() == 2 {
        // keyboard layout
        keyboard::draw(f, master_layout[1]);
    }
}
