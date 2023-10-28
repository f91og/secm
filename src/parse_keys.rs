use crate::app::App;
use crate::keymaps;
use crossterm::event::{KeyCode, KeyEvent};

pub fn parse_keys(app: &mut App, key: KeyEvent) -> Option<()> {
    match key.code {
        KeyCode::Char(ch) => {
            app.input.push(ch);
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Esc => return Some(()),
        // KeyCode::Tab => app.cycle_panels(true),
        // KeyCode::Left | KeyCode::Char('h') => app.cycle_panels(false),
        // KeyCode::Right | KeyCode::Char('l') => app.cycle_panels(true),
        KeyCode::Enter => keymaps::pressed_enter(app), // 复杂的处理放到keymaps里去
        KeyCode::Up => {
            if app.selected_secret_index > 0 {
                app.selected_secret_index -= 1;
            }
        }
        KeyCode::Down => {
            if app.selected_secret_index < app.len_after_filtered - 1 {
                app.selected_secret_index += 1;
            } else if app.selected_secret_index > app.len_after_filtered - 1 {
                app.selected_secret_index = app.len_after_filtered - 1;
            }
        }
        _ => {}
    }
    None
}