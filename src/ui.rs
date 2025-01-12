use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{palette::tailwind::{BLUE, SLATE}, Color, Modifier, Style, Stylize}, symbols, terminal::Frame, text::Line, widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, Paragraph}
};
use unicode_width::UnicodeWidthStr;
use crate::{app::App, panel::Panel, Storage};
use crate::app::Mode;
use crate::panel::PanelName;

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn ui<S: Storage>(f: &mut Frame, app: &mut App<S>) {
    let size = f.size();

    let vertical = Layout::vertical([
        Constraint::Max(0),
        Constraint::Min(0),
        Constraint::Length(3)
    ]).margin(2);
    let [mut filter_area, mut secrets_area, guide_area] = vertical.areas(size);

    if app.mode == Mode::Filter {
        let filter_string = app.get_filter_string();
        app.filter_secrets_list(&filter_string);
        let filter_chunk = Paragraph::new(filter_string.clone())
            .style(Style::default()
            .fg(Color::Yellow))
            .block(Block::default()
            .borders(Borders::ALL).title("Filter"));

        filter_area.height = 3;
        secrets_area.y += filter_area.height;
        secrets_area.height -= filter_area.height;

        f.render_widget(filter_chunk, filter_area);
        f.set_cursor(
            // Put cursor past the end of the input text
            filter_area.x + filter_string.width() as u16 + 1,
            // Move one line down, from the border to the input line
            filter_area.y + 1,
        );
    }

    // Render the list of secrets
    let block = Block::new()
        .title(Line::raw("SECRETS").centered())
        .borders(Borders::TOP)
        .border_set(symbols::border::EMPTY)
        .border_style(TODO_HEADER_STYLE)
        .bg(NORMAL_ROW_BG);

    let items: Vec<ListItem> = app
        .secret_list
        .secrets
        .iter()
        .enumerate()
        .map(|(i, secret_item)| {
            let color = alternate_colors(i);
            ListItem::from(secret_item).bg(color)
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let list = List::new(items)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
    // same method name `render`.
    // StatefulWidget::render(list, area, buf, &mut self.secret_list.state);
    // f.render_widget(secrets_chunk, secrets_area);
    f.render_stateful_widget(list, secrets_area, &mut app.secret_list.state);

    if app.mode == Mode::Add || app.mode == Mode::Update{
        let name_area = centered_rect(60, 7, size);
        let mut value_area = centered_rect(60, 7, size);
        value_area.y += 2; // position below name area

        let panels: &Panel;
        if app.mode == Mode::Add {
            panels = app.panels.get(&PanelName::AddSecret).unwrap();
        } else {
            panels = app.panels.get(&PanelName::UpdateSecret).unwrap();
        }

        render_label_input(f, name_area, "name: ".to_string(), panels.content[0].clone(), panels.index == 0);

        let secret_len = panels.content[1].width();
        // create a string using '*'s to represent the secret
        let secret_string = (0..secret_len).map(|_| "*").collect::<String>();
        render_label_input(f, value_area, "secret: ".to_string(), secret_string, panels.index == 1);
    }
    if app.mode == Mode::Delete {
        let confirm_area = centered_rect(30, 7, size);
        if let Some(selected_secret) = app.get_selected_item() {
            let confirm = format!("delete {}? y/n:", selected_secret.name);
            render_label_input(f, confirm_area, confirm, app.panels.get(&PanelName::DeleteSecret).unwrap().content[0].clone(), true);
        }
    }
    if app.mode == Mode::Make {
        let name_area = centered_rect(30, 7, size);
        let mut length_area = centered_rect(30, 7, size);
        let mut advance_area = centered_rect(30, 7, size);
        length_area.y += 2;
        advance_area.y += 4;

        let make_panel = app.panels.get(&PanelName::MakeSecret).unwrap();

        render_label_input(f, name_area, "name: ".to_string(), make_panel.content[0].clone(), make_panel.index == 0);
        render_label_input(f, length_area, "length: ".to_string(), make_panel.content[1].clone(), make_panel.index == 1);
        render_label_input(f, advance_area, "advance: ".to_string(), make_panel.content[2].clone(), make_panel.index == 2);
    }
    let guide_chunk = Paragraph::new(app.guide.to_string()).alignment(Alignment::Center).style(Style::default().fg(Color::Blue));
    let error_chunk = Paragraph::new(app.error.msg.to_string()).alignment(Alignment::Center).style(Style::default().fg(Color::Red));
    if app.error.msg.is_empty() {
        f.render_widget(guide_chunk, guide_area);
    } else {
        f.render_widget(error_chunk, guide_area);
        app.clear_error_if_expired();
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
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

fn render_label_input(f: &mut Frame, area: Rect, label: String, input_content: String, set_cursor: bool) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(label.width() as u16 + 1), Constraint::Percentage(80)].as_ref())
        .split(area);
    let label_paragraph = Paragraph::new(label)
        .style(Style::default()
        .fg(Color::Yellow));
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