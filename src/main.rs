use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use db::sql_client::MySqlClient;
use dotenv::dotenv;
use std::env;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};
use ui::widgets::{
    database::DatabaseWdg,
    sql_input::SqlInputWdg,
    sql_output::SqlOutputWdg,
    tab::{TabWdg, TableMode},
    table_column::TableColumnWdg,
    table_list::TableListWdg,
    table_record::TableRecordWdg,
};
use unicode_width::UnicodeWidthStr;

mod db;
mod ui;
mod utils;

enum WidgetMode {
    Normal,
    ChangeDB,
    EditSQL,
}

pub struct App {
    widget_mode: WidgetMode,
}

impl App {
    fn new() -> Self {
        Self {
            widget_mode: WidgetMode::Normal,
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

    // Set config
    dotenv().ok();

    let db_url = format!("{}", &env::var("DATABASE_URL").unwrap());
    let mut mysql_client = MySqlClient::new(&db_url).await;
    let databases = mysql_client.get_db_list().await;
    let default_database_name = &databases[0];

    let new_db_url = format!(
        "{}/{}",
        &env::var("DATABASE_URL").unwrap(),
        default_database_name.to_string()
    );
    mysql_client.change_db(&new_db_url).await;

    let tables = mysql_client
        .get_table_list(default_database_name.to_string())
        .await;
    let default_table_name = &tables[0];

    let (headers, records) = mysql_client
        .get_table_records(default_table_name.to_string())
        .await;

    let (column_headers, column_items) = mysql_client
        .get_table_columns(default_table_name.to_string())
        .await;

    let mut app = App::new();

    let mut database_widget = DatabaseWdg::new(databases.clone());
    let mut table_list_widget = TableListWdg::new(tables.clone());
    let mut table_record_widget =
        TableRecordWdg::new(default_table_name.to_string(), headers, records);
    let mut table_column_widget =
        TableColumnWdg::new(default_table_name.to_string(), column_headers, column_items);
    let mut sql_input_widget = SqlInputWdg::new();
    let mut sql_output_widget = SqlOutputWdg::new();
    let mut tab_widget = TabWdg::new();

    loop {
        terminal.draw(|f| {
            render_layout(
                f,
                &mut app,
                &mut database_widget,
                &mut table_record_widget,
                &mut table_column_widget,
                &mut table_list_widget,
                &mut sql_input_widget,
                &mut sql_output_widget,
                &mut tab_widget,
            )
        })?;

        if let Event::Key(key) = event::read()? {
            match app.widget_mode {
                WidgetMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('a') => {
                        table_list_widget.move_up();
                    }
                    KeyCode::Char('b') => {
                        table_list_widget.move_down();
                    }
                    KeyCode::Char('e') => {
                        app.widget_mode = WidgetMode::EditSQL;
                    }
                    KeyCode::Char('c') => {
                        app.widget_mode = WidgetMode::ChangeDB;
                    }
                    KeyCode::Char('0') => {
                        tab_widget.mode = TableMode::Records;
                    }
                    KeyCode::Char('1') => {
                        tab_widget.mode = TableMode::Columns;
                    }
                    KeyCode::Up => {
                        match tab_widget.mode {
                            TableMode::Records => {
                                table_record_widget.move_up();
                            }
                            TableMode::Columns => {
                                table_column_widget.move_up();
                            }
                        };
                    }
                    KeyCode::Down => {
                        match tab_widget.mode {
                            TableMode::Records => {
                                table_record_widget.move_down();
                            }
                            TableMode::Columns => {
                                table_column_widget.move_down();
                            }
                        };
                    }
                    KeyCode::Right => {
                        match tab_widget.mode {
                            TableMode::Records => {
                                table_record_widget.move_right();
                            }
                            TableMode::Columns => {
                                table_column_widget.move_right();
                            }
                        };
                    }
                    KeyCode::Left => {
                        match tab_widget.mode {
                            TableMode::Records => {
                                table_record_widget.move_left();
                            }
                            TableMode::Columns => {
                                table_column_widget.move_left();
                            }
                        };
                    }
                    KeyCode::Enter => {
                        table_list_widget.change_table();
                        if !table_record_widget
                            .is_current_table(table_list_widget.current_table.to_string())
                        {
                            // reset table records
                            let (record_headers, record_fields) = mysql_client
                                .get_table_records(table_list_widget.current_table.to_string())
                                .await;
                            table_record_widget.reset_default_records(
                                table_list_widget.current_table.to_string(),
                                record_headers,
                                record_fields,
                            );
                            // reset table columns
                            let (column_headers, column_fields) = mysql_client
                                .get_table_columns(table_list_widget.current_table.to_string())
                                .await;
                            table_column_widget.reset_default_records(
                                table_list_widget.current_table.to_string(),
                                column_headers,
                                column_fields,
                            );
                        }
                    }
                    _ => {}
                },
                WidgetMode::ChangeDB => match key.code {
                    KeyCode::Enter => {
                        database_widget.change_database();
                        // change db
                        let new_db_url = format!(
                            "{}/{}",
                            &env::var("DATABASE_URL").unwrap(),
                            database_widget.current_database.to_string()
                        );
                        mysql_client.change_db(&new_db_url).await;

                        let new_tables = mysql_client
                            .get_table_list(database_widget.current_database.to_string())
                            .await;
                        let new_table_name = &new_tables[0];
                        table_list_widget.change_table();
                        table_list_widget.change_tables(new_tables.clone());

                        // reset table records
                        let (record_headers, record_fields) = mysql_client
                            .get_table_records(new_table_name.to_string())
                            .await;
                        table_record_widget.reset_default_records(
                            table_list_widget.current_table.to_string(),
                            record_headers,
                            record_fields,
                        );

                        // reset table columns
                        let (column_headers, column_fields) = mysql_client
                            .get_table_columns(new_table_name.to_string())
                            .await;
                        table_column_widget.reset_default_records(
                            table_list_widget.current_table.to_string(),
                            column_headers,
                            column_fields,
                        );
                        app.widget_mode = WidgetMode::Normal;
                    }
                    KeyCode::Up => {
                        database_widget.move_up();
                    }
                    KeyCode::Down => {
                        database_widget.move_down();
                    }
                    KeyCode::Char('c') => {
                        app.widget_mode = WidgetMode::Normal;
                    }
                    _ => {}
                },
                WidgetMode::EditSQL => match key.code {
                    KeyCode::Enter => {
                        let res = mysql_client
                            .execute_input_query(sql_input_widget.input.clone())
                            .await;
                        match res {
                            Ok(res) => sql_output_widget.set_success_msg(res),
                            Err(e) => sql_output_widget.set_error_msg(e.to_string()),
                        }
                    }
                    KeyCode::Char(c) => {
                        sql_input_widget.input.push(c);
                    }
                    KeyCode::Backspace => {
                        sql_input_widget.input.pop();
                    }
                    KeyCode::Esc => {
                        app.widget_mode = WidgetMode::Normal;
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
    database_widget: &mut DatabaseWdg,
    table_record_widget: &mut TableRecordWdg,
    table_column_widget: &mut TableColumnWdg,
    table_list_widget: &mut TableListWdg,
    sql_input_widget: &mut SqlInputWdg,
    sql_output_widget: &mut SqlOutputWdg,
    tab_widget: &mut TabWdg,
) {
    let size = f.size();

    table_record_widget.update_visible_range();

    match app.widget_mode {
        WidgetMode::Normal => {
            let (chunks_1, chunks_2) = ui::layout::make_normal_layout(size);

            // database widget
            f.render_widget(database_widget.current_database_widget(), chunks_1[0]);

            // table list widget
            f.render_stateful_widget(
                table_list_widget.widget(),
                chunks_1[1],
                &mut table_list_widget.table_select_state,
            );
            // sql input widget
            f.render_widget(sql_input_widget.widget(), chunks_2[0]);

            // tab widget
            f.render_widget(tab_widget.widget(), chunks_2[1]);

            match tab_widget.mode {
                TableMode::Records => {
                    f.render_stateful_widget(
                        // table record widget
                        table_record_widget.widget(),
                        chunks_2[2],
                        &mut table_record_widget.select_row_list_state,
                    );
                }
                TableMode::Columns => {
                    f.render_stateful_widget(
                        // table column widget
                        table_column_widget.widget(),
                        chunks_2[2],
                        &mut table_column_widget.select_row_list_state,
                    );
                }
            };
        }
        WidgetMode::ChangeDB => {
            let (chunks_1, chunks_2) = ui::layout::make_change_db_layout(size);

            // database widget
            f.render_stateful_widget(
                database_widget.expand_db_list_widget(),
                chunks_1,
                &mut database_widget.database_select_state,
            );

            // sql input widget
            f.render_widget(sql_input_widget.widget(), chunks_2[0]);

            // tab widget
            f.render_widget(tab_widget.widget(), chunks_2[1]);

            match tab_widget.mode {
                TableMode::Records => {
                    // table record widget
                    f.render_stateful_widget(
                        table_record_widget.widget(),
                        chunks_2[2],
                        &mut table_record_widget.select_row_list_state,
                    );
                }
                TableMode::Columns => {
                    // table column widget
                    f.render_stateful_widget(
                        table_column_widget.widget(),
                        chunks_2[2],
                        &mut table_column_widget.select_row_list_state,
                    );
                }
            };
        }
        WidgetMode::EditSQL => {
            let (chunks_1, chunks_2) = ui::layout::make_edit_sql_layout(size);

            // database widget
            f.render_widget(database_widget.current_database_widget(), chunks_1[0]);

            // table list widget
            f.render_stateful_widget(
                table_list_widget.widget(),
                chunks_1[1],
                &mut table_list_widget.table_select_state,
            );

            // sql input widget
            f.render_widget(sql_input_widget.widget(), chunks_2[0]);
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks_2[0].x + sql_input_widget.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks_2[0].y + 1,
            );

            // sql output widget
            f.render_widget(sql_output_widget.widget(), chunks_2[1]);
        }
    }
}
