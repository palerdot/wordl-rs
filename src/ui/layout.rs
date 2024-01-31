use ratatui::prelude::*;
use std::rc::Rc;

pub fn master_layout(frame: &mut Frame) -> Rc<[ratatui::layout::Rect]> {
    let size = frame.size();

    let constraints = if size.height >= 31 {
        vec![Constraint::Percentage(66), Constraint::Percentage(34)]
    } else {
        vec![Constraint::Percentage(100)]
    };

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame.size())
}
