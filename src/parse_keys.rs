use crate::app::App;
use crate::app::Mode;
use crate::keymaps;
use crate::panel::PanelName;
use crossterm::event::{KeyCode, KeyEvent};

pub fn parse_keys(app: &mut App, key: KeyEvent) -> Option<()> {
    // https://www.reddit.com/r/rust/comments/17qi2oq/why_here_occurs_error_cannot_borrow_appmode_as/
    let filter_panel = app.panels.get_mut(&PanelName::Filter).unwrap(); // ok
    // let filter_panel = app.get_specific_panel(PanelName::Filter); // not ok

    match key.code {
        KeyCode::Char(ch) => {
            if ch == '/' && app.mode == Mode::Normal {
                app.mode = Mode::Filter;
            } else if app.mode == Mode::Filter {
                filter_panel.content[0].push(ch);
                app.refresh_secrets_panel();
            } else if app.mode == Mode::Normal {
                if ch == 'q' {
                    return Some(())
                } else if ch == 'j' {
                    keymaps::move_cursor_vertical(app, 1);
                } else if ch == 'k' {
                    keymaps::move_cursor_vertical(app, -1);
                }
            }
        }
        KeyCode::Backspace => {
            filter_panel.content[0].pop();
            app.refresh_secrets_panel();
        }
        KeyCode::Esc => return Some(()),
        KeyCode::Enter => {
            if app.mode == Mode::Normal {
                keymaps::pressed_enter(app);    // 复杂的处理放到keymaps里去
                return Some(());
            } else {
                app.mode = Mode::Normal
            }
        }
        KeyCode::Up => {
            keymaps::move_cursor_vertical(app, -1);
        }
        KeyCode::Down => {
            keymaps::move_cursor_vertical(app, 1);
        }
        _ => {}
    }
    None
}