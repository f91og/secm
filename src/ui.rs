use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    style::{Color, Style},
    Frame,
};
use unicode_width::UnicodeWidthStr;
use crate::app::App;
use crate::app::Mode;
use crate::panel::PanelName;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let mut chunks = Layout::default() // 首先获取默认构造
        .direction(Direction::Vertical) // 垂直分割
        .margin(2)
        .constraints([Constraint::Max(0), Constraint::Min(0), Constraint::Length(3)].as_ref()) // 画了3个chunk，后面填充内容
        .split(size);

    // Render the filter input
    let filter_panel = app.panels.get(&PanelName::Filter).unwrap();
    let filter_chunk = Paragraph::new(filter_panel.content[0].clone())
       .style(Style::default()
       .fg(Color::Yellow))
       .block(Block::default()
       .borders(Borders::ALL).title("Filter"));

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
        .block(Block::default()
        .borders(Borders::ALL))
        .highlight_style(Style::default()
        .fg(Color::Yellow));

    if app.mode == Mode::Filter {
        let mut filter_area = chunks[0];
        filter_area.height = 3;

        chunks[1].y += filter_area.height;
        chunks[1].height -= filter_area.height;

        f.render_widget(filter_chunk, filter_area);
        f.set_cursor(
            // Put cursor past the end of the input text
            filter_area.x + filter_panel.content[0].width() as u16 + 1,
            // Move one line down, from the border to the input line
            filter_area.y + 1,
        );
    }
    f.render_widget(secrets_chunk, chunks[1]);

    if app.mode == Mode::Rename {
        let (current_secret, _) = app.get_selected_secret();
        let rename_secret_chunk = Paragraph::new(app.panels.get(&PanelName::RenameSecret).unwrap().content[0].clone())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(format!("rename {}", current_secret)));
        let area = centered_rect(60, 7, size); // here dose size come from?
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(rename_secret_chunk, area);
    }
    if app.mode == Mode::Add {
        let name_area = centered_rect(30, 7, size);
        let mut value_area = centered_rect(30, 7, size);
        value_area.y += 2; // position below name area

        let app_add_secret_panel = app.panels.get(&PanelName::AddSecret).unwrap();

        render_label_input(f, name_area, "name: ".to_string(), app_add_secret_panel.content[0].clone(), app_add_secret_panel.index == 0);
        render_label_input(f, value_area, "value: ".to_string(), app_add_secret_panel.content[1].clone(), app_add_secret_panel.index == 1);
    }
    if app.mode == Mode::Delete {
        let (current_secret, _) = app.get_selected_secret();
        let confirm_area = centered_rect(30, 7, size);

        let app_delete_secret_panel = app.panels.get(&PanelName::DeleteSecret).unwrap();
        let confirm = format!("delete {}? y/n:", current_secret);
        render_label_input(f, confirm_area, confirm, app_delete_secret_panel.content[0].clone(), true);
    }
    if app.mode == Mode::Make {
        let name_area = centered_rect(30, 7, size);
        let mut length_area = centered_rect(30, 7, size);
        let mut advance_area = centered_rect(30, 7, size);
        length_area.y += 2; 
        advance_area.y += 4; 

        let app_make_secret_panel = app.panels.get(&PanelName::MakeSecret).unwrap();
   
        render_label_input(f, name_area, "name: ".to_string(), app_make_secret_panel.content[0].clone(), app_make_secret_panel.index == 0);
        render_label_input(f, length_area, "length: ".to_string(), app_make_secret_panel.content[1].clone(), app_make_secret_panel.index == 1);
        render_label_input(f, advance_area, "advance: ".to_string(), app_make_secret_panel.content[2].clone(), app_make_secret_panel.index == 2);
    }
    let guide_chunk = Paragraph::new(app.guide.to_string()).alignment(Alignment::Center).style(Style::default().fg(Color::Blue));
    let error_chunk = Paragraph::new(app.error.to_string()).alignment(Alignment::Center).style(Style::default().fg(Color::Red));
    if app.error.is_empty() {
        f.render_widget(guide_chunk, chunks[2]);
    } else {
        f.render_widget(error_chunk, chunks[2]);
        app.clear_error_if_expired();
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn render_label_input<B: Backend>(f: &mut Frame<B>, area: Rect, label: String, input_content: String, set_cursor: bool) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(label.width() as u16 + 1), Constraint::Percentage(80)].as_ref())
        .split(area);
    let label_paragraph = Paragraph::new(label)
        .style(Style::default().fg(Color::Yellow));
    let input_paragraph= Paragraph::new(input_content.clone())
        .style(Style::default()
        .fg(Color::Yellow))
        .block(Block::default());
    f.render_widget(Clear, area);
    f.render_widget(label_paragraph, layout[0]);
    f.render_widget(input_paragraph, layout[1]);
    if set_cursor {
        f.set_cursor(
            layout[1].x + input_content.width() as u16,
            layout[1].y
        )
    }
}