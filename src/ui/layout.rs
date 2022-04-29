use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn make_layout(size: Rect) -> (Vec<Rect>, Vec<Rect>, Vec<Rect>) {
    let chunks_1 = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let chunks_2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Length(85)].as_ref())
        .split(chunks_1[1]);

    let chunks_3 = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(chunks_2[1]);

    return (chunks_1, chunks_2, chunks_3);
}
