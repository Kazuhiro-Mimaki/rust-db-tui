use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| render_layout(f))?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor()?;

    Ok(())
}

fn render_layout<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let block_1 = Block::default().title("Block 1").borders(Borders::ALL);
    let block_2_1 = Block::default().title("Block 2-1").borders(Borders::ALL);
    let block_2_2 = Block::default().title("Block 2-2").borders(Borders::ALL);

    let chunks_1 = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(size);

    let chunks_2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
        .split(chunks_1[1]);

    f.render_widget(block_1, chunks_1[0]);
    f.render_widget(block_2_1, chunks_2[0]);
    f.render_widget(block_2_2, chunks_2[1]);
}
