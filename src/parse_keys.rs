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
                        'r' => {
                            let (current_secret, _) = app.get_selected_secret();
                            app.panels.get_mut(&PanelName::RenameSecret).unwrap().content[0] = current_secret;
                            app.mode = Mode::Rename
                        },
                        'm' => app.mode = Mode::Make,
                        'a' => app.mode = Mode::Add,
                        '/' => app.mode = Mode::Filter,
                        'd' => app.mode = Mode::Delete,
                        _ => {}
                    }
                }
                KeyCode::Down => keymaps::move_cursor_vertical(app, 1),
                KeyCode::Up => keymaps::move_cursor_vertical(app, -1),
                KeyCode::Enter => {
                    keymaps::pressed_enter(app);    // 复杂的处理放到keymaps里去
                    return Some(());
                }
                _ => {}
            }
        }
        Mode::Filter => {
            match key.code {
                KeyCode::Char(ch) => {
                    app.panels.get_mut(&PanelName::Filter).unwrap().content[0].push(ch);
                    app.filter_secrets_panel();
                }
                KeyCode::Backspace => {
                    app.panels.get_mut(&PanelName::Filter).unwrap().content[0].pop();
                    app.filter_secrets_panel();
                }
                KeyCode::Esc => app.back_to_normal_mode(),
                KeyCode::Enter => {
                    keymaps::pressed_enter(app);    // 复杂的处理放到keymaps里去
                    return Some(());
                }
                KeyCode::Down => keymaps::move_cursor_vertical(app, 1),
                KeyCode::Up => keymaps::move_cursor_vertical(app, -1),
                _ => {}
            }
        }
        Mode::Make  => {
            match key.code {
                KeyCode::Esc => app.mode = Mode::Normal,
                _ => {}
            }
        }
        Mode::Rename  => {
            match key.code {
                KeyCode::Char(ch) => {
                    app.panels.get_mut(&PanelName::RenameSecret).unwrap().content[0].push(ch);
                }
                KeyCode::Backspace => {
                    app.panels.get_mut(&PanelName::RenameSecret).unwrap().content[0].pop();
                }
                KeyCode::Esc => app.back_to_normal_mode(),
                KeyCode::Enter => {
                    keymaps::pressed_enter(app);
                    app.back_to_normal_mode();
                }
                _ => {}
            }
        }
        Mode::Add => {
            match key.code {
                KeyCode::Char(ch) => {
                    let current_content_index = app.panels.get(&PanelName::AddSecret).unwrap().index;
                    app.panels.get_mut(&PanelName::AddSecret).unwrap().content[current_content_index].push(ch);
                }
                KeyCode::Backspace => {
                    let current_content_index = app.panels.get(&PanelName::AddSecret).unwrap().index;
                    app.panels.get_mut(&PanelName::AddSecret).unwrap().content[current_content_index].pop();
                }
                KeyCode::Enter => {
                    keymaps::pressed_enter(app);
                    app.back_to_normal_mode();
                }
                KeyCode::Esc => app.back_to_normal_mode(),
                KeyCode::Tab => {
                    let current_content_index = app.panels.get(&PanelName::AddSecret).unwrap().index;
                    // toggle between fields
                    if current_content_index == 0 {
                        app.panels.get_mut(&PanelName::AddSecret).unwrap().index = 1;
                    } else {
                        app.panels.get_mut(&PanelName::AddSecret).unwrap().index = 0;
                    }
                }
                _ => {}
            }
        }
        Mode::Delete => {
            match key.code {
                KeyCode::Char(ch) => {
                    app.panels.get_mut(&PanelName::DeleteSecret).unwrap().content[0].push(ch);
                }
                KeyCode::Backspace => {
                    app.panels.get_mut(&PanelName::DeleteSecret).unwrap().content[0].pop();
                }
                KeyCode::Esc => app.back_to_normal_mode(),
                KeyCode::Enter => {
                    keymaps::pressed_enter(app);    // 复杂的处理放到keymaps里去
                    app.back_to_normal_mode();
                }
                _ => {}
            }
        }
    }
    None
}