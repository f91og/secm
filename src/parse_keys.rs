use crate::app::App;
use crate::app::Mode;
use crate::keymaps;
use crate::panel::PanelName;
use crossterm::event::{KeyCode, KeyEvent};

pub fn parse_keys(app: &mut App, key: KeyEvent) -> Option<()> {
    // https://www.reddit.com/r/rust/comments/17qi2oq/why_here_occurs_error_cannot_borrow_appmode_as/
    // let filter_panel = app.panels.get_mut(&PanelName::Filter).unwrap(); // ok
    // let filter_panel = app.get_specific_panel(PanelName::Filter); // not ok

    match app.mode {
        Mode::Normal => {
            match key.code {
                KeyCode::Char(ch) => {
                    match ch {
                        'q' => return Some(()),
                        'j' => keymaps::move_cursor_vertical(app, 1),
                        'k' => keymaps::move_cursor_vertical(app, -1),
                        'r' => app.switch_mode(Mode::Update),
                        'm' => app.switch_mode(Mode::Make),
                        'a' => app.switch_mode(Mode::Add),
                        '/' => app.switch_mode(Mode::Filter),
                        'd' => app.switch_mode(Mode::Delete),
                        _ => {}
                    }
                }
                KeyCode::Down => keymaps::move_cursor_vertical(app, 1),
                KeyCode::Up => keymaps::move_cursor_vertical(app, -1),
                KeyCode::Esc => return Some(()),
                KeyCode::Enter => {keymaps::pressed_enter(app); return Some(())},    // 复杂的处理放到keymaps里去
                _ => {}
            }
        }
        Mode::Filter => {
            let filter_panel = app.panels.get_mut(&PanelName::Filter).unwrap();
            match key.code {
                KeyCode::Char(ch) => {
                    filter_panel.content[0].push(ch);
                    app.filter_secrets_panel();
                }
                KeyCode::Backspace => {
                    filter_panel.content[0].pop();
                    app.filter_secrets_panel();
                }
                KeyCode::Esc => app.switch_mode(Mode::Normal),
                KeyCode::Enter => app.mode = Mode::Normal,
                KeyCode::Down => keymaps::move_cursor_vertical(app, 1),
                KeyCode::Up => keymaps::move_cursor_vertical(app, -1),
                _ => {}
            }
        }
        Mode::Make => {
            let make_secret_panel = app.panels.get_mut(&PanelName::MakeSecret).unwrap();
            match key.code {
                KeyCode::Esc => app.switch_mode(Mode::Normal),
                KeyCode::Char(ch) => make_secret_panel.content[make_secret_panel.index].push(ch),
                KeyCode::Backspace => _ = make_secret_panel.content[make_secret_panel.index].pop(),
                KeyCode::Tab => make_secret_panel.index = (make_secret_panel.index + 1) % 3,
                KeyCode::Enter => keymaps::pressed_enter(app),
                _ => {}
            }
        }
        Mode::Update => {
            let panel = app.panels.get_mut(&PanelName::UpdateSecret).unwrap();
            match key.code {
                KeyCode::Char(ch) => panel.content[panel.index].push(ch),
                KeyCode::Backspace => _ = panel.content[panel.index].pop(),
                KeyCode::Esc => app.switch_mode(Mode::Normal),
                KeyCode::Enter => keymaps::pressed_enter(app),
                KeyCode::Tab => panel.index ^= 1,
                _ => {}
            }
        }
        Mode::Add => {
            let panel = app.panels.get_mut(&PanelName::AddSecret).unwrap();
            match key.code {
                KeyCode::Char(ch) => panel.content[panel.index].push(ch),
                KeyCode::Backspace => _ = panel.content[panel.index].pop(),
                KeyCode::Enter => keymaps::pressed_enter(app),
                KeyCode::Esc => app.switch_mode(Mode::Normal),
                KeyCode::Tab => panel.index ^= 1,
                _ => {}
            }
        }
        Mode::Delete => {
            let delete_secret_panel = app.panels.get_mut(&PanelName::DeleteSecret).unwrap();
            match key.code {
                KeyCode::Char(ch) => delete_secret_panel.content[0].push(ch),
                KeyCode::Backspace => _ = delete_secret_panel.content[0].pop(),
                KeyCode::Esc => app.switch_mode(Mode::Normal),
                KeyCode::Enter => keymaps::pressed_enter(app),    // 复杂的处理放到keymaps里去
                _ => {}
            }
        }
    }
    None
}