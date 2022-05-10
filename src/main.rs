use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use sqlx::mysql::MySqlQueryResult;
use std::env;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::ListState,
    Frame, Terminal,
};
use ui::widgets::tab;

mod db;
mod table;
mod ui;
mod utils;

enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    input: String,
    input_mode: InputMode,
    output: MySqlQueryResult,
    error: String,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            output: MySqlQueryResult::default(),
            error: String::new(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut table_list_state = ListState::default();
    table_list_state.select(Some(0));

    // Set config
    dotenv().ok();

    let mysql_client = db::sql_client::MySqlClient::new(&env::var("DATABASE_URL").unwrap()).await;

    let databases = mysql_client.get_db_list().await;

    let tables = mysql_client.get_table_list(&databases[0]).await;
    let default_table_name = &tables[0];

    let (headers, records) = mysql_client.get_record_list(default_table_name).await;
    let mut table_struct =
        table::TableStruct::new(default_table_name.to_string(), headers, records);

    let (column_headers, column_items) = mysql_client.get_table_columns(default_table_name).await;
    let mut column_table =
        table::TableStruct::new(default_table_name.to_string(), column_headers, column_items);

    let mut tab_struct = ui::widgets::tab::TabStruct::new();

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            render_layout(
                f,
                &mut app,
                &tables,
                &mut tab_struct,
                &mut table_struct,
                &mut column_table,
                &mut table_list_state,
            )
        })?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('a') => {
                        if let Some(selected) = table_list_state.selected() {
                            if selected != 0 {
                                table_list_state.select(Some(selected - 1));
                            };
                        }
                    }
                    KeyCode::Char('b') => {
                        if let Some(selected) = table_list_state.selected() {
                            if selected < tables.len().saturating_sub(1) {
                                table_list_state.select(Some(selected + 1));
                            }
                        }
                    }
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('0') => {
                        tab_struct.index = 0;
                        if let Some(selected) = table_list_state.selected() {
                            let selected_table_name = &tables[selected];
                            if selected_table_name.to_string() != table_struct.name {
                                let (headers, records) =
                                    mysql_client.get_record_list(selected_table_name).await;
                                table_struct
                                    .reset_default_records(
                                        selected_table_name.to_string(),
                                        headers,
                                        records,
                                    )
                                    .await;
                            }
                        }
                    }
                    KeyCode::Char('1') => {
                        tab_struct.index = 1;
                        if let Some(selected) = table_list_state.selected() {
                            let selected_table_name = &tables[selected];
                            if selected_table_name.to_string() != column_table.name {
                                let (headers, records) =
                                    mysql_client.get_table_columns(selected_table_name).await;
                                column_table
                                    .reset_default_records(
                                        selected_table_name.to_string(),
                                        headers,
                                        records,
                                    )
                                    .await;
                            }
                        }
                    }
                    KeyCode::Up => {
                        match tab_struct.index {
                            0 => {
                                table_struct.move_up();
                            }
                            1 => {
                                column_table.move_up();
                            }
                            _ => unreachable!(),
                        };
                    }
                    KeyCode::Down => {
                        match tab_struct.index {
                            0 => {
                                table_struct.move_down();
                            }
                            1 => {
                                column_table.move_down();
                            }
                            _ => unreachable!(),
                        };
                    }
                    KeyCode::Right => {
                        match tab_struct.index {
                            0 => {
                                table_struct.move_right();
                            }
                            1 => {
                                column_table.move_right();
                            }
                            _ => unreachable!(),
                        };
                    }
                    KeyCode::Left => {
                        match tab_struct.index {
                            0 => {
                                table_struct.move_left();
                            }
                            1 => {
                                column_table.move_left();
                            }
                            _ => unreachable!(),
                        };
                    }
                    KeyCode::Enter => {
                        tab_struct.index = 0;
                        if let Some(selected) = table_list_state.selected() {
                            let selected_table_name = &tables[selected];
                            if selected_table_name.to_string() != table_struct.name {
                                let (headers, records) =
                                    mysql_client.get_record_list(selected_table_name).await;
                                table_struct
                                    .reset_default_records(
                                        selected_table_name.to_string(),
                                        headers,
                                        records,
                                    )
                                    .await;
                            };
                        }
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let res = mysql_client.execute_input_query(&mut app).await;
                        match res {
                            Ok(result) => app.output = result,
                            Err(e) => app.error = e.to_string(),
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
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

fn render_layout<B: Backend>(
    f: &mut Frame<'_, B>,
    app: &mut App,
    tables: &Vec<String>,
    tab_struct: &mut tab::TabStruct,
    table_struct: &mut table::TableStruct,
    column_table: &mut table::TableStruct,
    table_list_state: &mut ListState,
) {
    let size = f.size();

    table_struct.update_visible_range();

    match app.input_mode {
        InputMode::Normal => {
            let (chunks_1, chunks_2) = ui::layout::make_normal_layout(size);

            ui::widgets::tables::render_table_list_wdg(f, chunks_1[0], tables, table_list_state);
            ui::widgets::input_query::render_sql_input_wdg(f, chunks_2[0], app);
            ui::widgets::tab::render_tabs_wdg(f, chunks_2[1], tab_struct);
            ui::widgets::tab::render_table_by_tab_wdg(
                f,
                chunks_2[2],
                tab_struct,
                table_struct,
                column_table,
            );
        }
        InputMode::Editing => {
            let (chunks_1, chunks_2) = ui::layout::make_edit_layout(size);

            ui::widgets::tables::render_table_list_wdg(f, chunks_1[0], tables, table_list_state);
            ui::widgets::input_query::render_sql_input_wdg(f, chunks_2[0], app);
            ui::widgets::input_query::render_sql_output_wdg(f, chunks_2[1], app);
        }
    }
}
