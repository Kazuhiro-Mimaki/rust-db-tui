use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn make_normal_layout(size: Rect) -> (Vec<Rect>, Vec<Rect>) {
    let chunks_1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Length(85)].as_ref())
        .split(size);

    let chunks_2 = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(chunks_1[1]);

    return (chunks_1, chunks_2);
}

pub fn make_edit_layout(size: Rect) -> (Vec<Rect>, Vec<Rect>) {
    let chunks_1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Length(85)].as_ref())
        .split(size);

    let chunks_2 = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(chunks_1[1]);

    return (chunks_1, chunks_2);
}
