use crate::app::App;
// use crate::panel::PanelName;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    style::{Color, Style},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default() // 首先获取默认构造
        .direction(Direction::Vertical) // 垂直分割
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref()) // 按照 3 行 和 最小 0 行的规则分割区域
        .split(size); // 分割整块 Terminal 区域

    // Render the filter input
    let filter_input = Paragraph::new(app.input.as_ref())
       .style(Style::default().fg(Color::Yellow))
       .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(filter_input, chunks[0]);
    f.set_cursor(
        // Put cursor past the end of the input text
        chunks[0].x + app.input.width() as u16 + 1,
        // Move one line down, from the border to the input line
        chunks[0].y + 1,
    );

    // Render the list of items
    let items_block = Block::default().borders(Borders::ALL);

    // if item index equal to selected index, highlight the item
    let items: Vec<ListItem> = app.secrets.iter()
        .enumerate()
        .filter(|(_, item)| {
            app.input.is_empty() || item.contains(app.input.as_str())
        })
        .map(|(i, item)| {
            let style = if i == app.selected_secret_index {
                Style::default().fg(Color::Yellow).bg(Color::LightCyan)
            } else {
                Style::default()
            };
            // filter item those that match the filter input
            ListItem::new(item.clone()).style(style)
        })
        .collect();
    app.len_after_filtered = items.len();
    // let items: Vec<ListItem> = app.secrets.iter().enumerate().map(|(i, item)| {
    //     let style = if i == app.selected_secret_index {
    //         Style::default().fg(Color::Yellow).bg(Color::LightCyan)
    //     } else {
    //         Style::default()
    //     };
    //     // filter item those that match the filter input
    //     ListItem::new(item.clone()).style(style)
    // }).collect();
    let items_list = List::new(items)
            .block(items_block)
            .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow));
    f.render_widget(items_list, chunks[1]);
}