use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use sqlx::{MySqlPool, Row};
use std::env;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set config
    dotenv().ok();

    loop {
        terminal.draw(|f| render_layout(f))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Down => {}
                KeyCode::Up => {}
                _ => {}
            }
        }
    }

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}

#[tokio::main]
async fn render_layout<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let block_1 = Block::default().title("Block 1").borders(Borders::ALL);
    let block_2_1 = Block::default().title("Tables").borders(Borders::ALL);
    let block_2_2 = Block::default().title("Block 2-2").borders(Borders::ALL);

    // Set database
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // Show table name list
    let query = format!(
        "{} {}",
        "SHOW TABLE STATUS FROM",
        &env::var("DB_NAME").unwrap()
    );
    let tables = sqlx::query(&query.as_str()).fetch_all(&pool).await.unwrap();

    let items: Vec<_> = tables
        .iter()
        .map(|table| {
            ListItem::new(Spans::from(vec![Span::styled(
                table.get::<String, _>("Name"),
                Style::default(),
            )]))
        })
        .collect();

    let tables = List::new(items).block(block_2_1).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

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
    f.render_widget(tables, chunks_2[0]);
    f.render_widget(block_2_2, chunks_2[1]);
}
