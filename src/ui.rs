use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    style::{Color, Style},
    Frame,
};
use unicode_width::UnicodeWidthStr;
use crate::app::App;
use crate::app::Mode;
use crate::panel::PanelName;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default() // 首先获取默认构造
        .direction(Direction::Vertical) // 垂直分割
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref()) // 按照 3 行 和 最小 0 行的规则分割区域
        .split(size); // 分割整块 Terminal 区域

    // Render the filter input
    let filter_panel = app.panels.get(&PanelName::Filter).unwrap();

    let filter_chunk = Paragraph::new(filter_panel.content[0].clone())
       .style(Style::default().fg(Color::Yellow))
       .block(Block::default().borders(Borders::ALL).title("Filter"));

    // Render the list of items
    let items_block = Block::default().borders(Borders::ALL);

    let secrets_panel = app.panels.get(&PanelName::Secrets).unwrap();
    // ui渲染逻辑里不应该有任何数据处理逻辑的

    let items: Vec<ListItem> = secrets_panel.content.iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == secrets_panel.index {
                Style::default().fg(Color::Black).bg(Color::LightCyan)
            } else {
                Style::default()
            };
            // filter item those that match the filter input
            ListItem::new(item.clone()).style(style)
        })
        .collect();

    let secrets_chunk = List::new(items)
            .block(items_block)
            .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow));

    if app.mode == Mode::Filter {
        f.render_widget(filter_chunk, chunks[0]);
        f.set_cursor(
            // Put cursor past the end of the input text
            chunks[0].x + filter_panel.content[0].width() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[0].y + 1,
        );
    }
    f.render_widget(secrets_chunk, chunks[1]);
}