use crate::app::App;
use crate::panel::PanelName;
use crate::app::Mode;
use clipboard::{ClipboardContext, ClipboardProvider};

// 键盘按键对应的处理函数，比如回车键后复制内容到剪贴板
pub fn pressed_enter(app: &mut App) {
    match app.mode {
        Mode::Rename => {
            _ = app.rename_secret(); // todo: return operation result
        }
        Mode::Add => {
            _ = app.add_secret();
        }
        Mode::Delete => {
            if app.panels.get(&PanelName::DeleteSecret).unwrap().content[0].trim() == "y" {
                _ = app.delete_secret();
            }
        }
        _ => {
            let (_, secret) = app.get_selected_secret();
                // 复制到剪贴板
            let mut clipboard = ClipboardContext::new().unwrap();
            clipboard.set_contents(secret).unwrap();
        }
    }
}

pub fn move_cursor_vertical(app: &mut App, step: i8) {
    let secrets_panel = app.panels.get_mut(&PanelName::Secrets).unwrap();
    if step == 1 && secrets_panel.index < secrets_panel.content.len() - 1 {
        secrets_panel.index += 1;
    } else if step == -1 && secrets_panel.index > 0 {
        secrets_panel.index -= 1;
    }
}