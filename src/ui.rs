use crate::app::App;
// use crate::panel::PanelName;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    style::{Color, Style},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    // Render the filter input
    let filter_input = Paragraph::new(app.input.as_ref())
       .style(Style::default().fg(Color::Yellow))
       .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(filter_input, chunks[0]);
    f.set_cursor(chunks[0].x + 1, chunks[0].y + 1);

    // let filter_block = Block::default().borders(Borders::ALL);
    // f.render_widget(filter_block, chunks[0]);

    // Render the list of items
    let items_block = Block::default().borders(Borders::ALL);

    let items: Vec<ListItem> = app.secrets.iter().map(|item| ListItem::new(item.clone())).collect();
    let items_list = List::new(items)
            .block(items_block)
            .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow));
    f.render_widget(items_list, chunks[1]);

    // let chunks = Layout::default() // 首先获取默认构造
    //     .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref()) // 按照 3 行 和 最小 3 行的规则分割区域
    //     .direction(Direction::Vertical) // 垂直分割
    //     .split(f.size()); // 分割整块 Terminal 区域
    // let paragraph = Paragraph::new(Span::styled(
    //     app.url.as_str(),
    //     Style::default().add_modifier(Modifier::BOLD),
    // ))
    // .block(Block::default().borders(Borders::ALL).title("HelloGitHub"))
    // .alignment(tui::layout::Alignment::Left);
    // f.render_widget(paragraph, chunks[0]);

    // let paragraph = Paragraph::new("分享 GitHub 上有趣、入门级的开源项目")
    //     .style(Style::default().bg(Color::White).fg(Color::Black))
    //     .block(Block::default().borders(Borders::ALL).title("宗旨"))
    //     .alignment(Alignment::Center);
    // f.render_widget(paragraph, chunks[1]);
}