use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::table::TableStruct;

use super::records::render_table_records_wdg;

pub struct TabStruct<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabStruct<'a> {
    pub fn new() -> TabStruct<'a> {
        TabStruct {
            titles: vec!["Records [0]", "Columns [1]"],
            index: 0,
        }
    }
}

pub fn render_tabs_wdg<B: Backend>(f: &mut Frame<B>, area: Rect, tab_struct: &mut TabStruct) {
    let titles = tab_struct
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default())))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL))
        .select(tab_struct.index)
        .style(Style::default())
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        );
    f.render_widget(tabs, area);
}

pub fn render_table_by_tab_wdg<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    tabs: &mut TabStruct,
    table_records: &mut TableStruct,
    table_columns: &mut TableStruct,
) {
    match tabs.index {
        0 => {
            render_table_records_wdg(f, area, table_records);
        }
        1 => {
            render_table_records_wdg(f, area, table_columns);
        }

        _ => unreachable!(),
    };
}
