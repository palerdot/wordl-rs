use ratatui::widgets::block::Title;
use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, rect: Rect) {
    let master_block = Block::new()
        .title(Title::from("  WORDL  ").alignment(Alignment::Center))
        .borders(Borders::ALL)
        .style(Style::new().white().on_black().bg(Color::Rgb(0, 0, 0)));

    frame.render_widget(master_block, rect);

    let keyboard_size = (10 * 5) / 2; // 10 letters max and width 5

    let letters = vec!["qwertyuiop".chars(), "asdfghjkl".chars(), "zxcvbnm".chars()];

    let width = 5;
    let height = 3;

    for (row_index, row) in letters.iter().enumerate() {
        for (index, letter) in row.clone().enumerate() {
            let offset: u16 = u16::from((rect.width - rect.left()) / 2) - keyboard_size;
            let x = rect.left() + u16::from(index as u16) * width + offset + (row_index as u16 * 3);
            let area = Rect {
                x,
                y: rect.top() + (row_index as u16 * height + 1),
                width,
                height,
            };
            let block = Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(0, 0, 0, 0))
                .border_style(Style::new().fg(Color::Rgb(255, 0, 0)))
                .style(Style::new().white().on_black().bg(Color::Rgb(0, 0, 0)));

            frame.render_widget(
                Paragraph::new(format!("{}", letter.to_string()))
                    .block(block)
                    .alignment(Alignment::Center),
                area,
            );
        }
    }
}
