use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
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
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref()) // 画了3个chunk，后面填充内容
        .split(size);

    // Render the filter input
    let filter_panel = app.panels.get(&PanelName::Filter).unwrap();
    let filter_chunk = Paragraph::new(filter_panel.content[0].clone())
       .style(Style::default().fg(Color::Yellow))
       .block(Block::default().borders(Borders::ALL).title("Filter"));

    // Render the list of secrets
    let secrets_panel = app.panels.get(&PanelName::Secrets).unwrap();
    // ui渲染逻辑里不应该有任何数据处理逻辑的
    let secrets: Vec<ListItem> = secrets_panel.content.iter()
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

    let secrets_chunk = List::new(secrets)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow));

    let command_guides = "shift + d: delete secret, shift + a: add secret, v: show secret content, enter: copy secret to clipboard, /: filter secrets";
    let command_guides_chunk = Paragraph::new(command_guides).alignment(Alignment::Center).style(Style::default().fg(Color::Blue));

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
    f.render_widget(command_guides_chunk, chunks[2]);
}