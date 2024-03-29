use crate::app::App;
use crate::panel::PanelName;
use crate::app::Mode;
use clipboard::{ClipboardContext, ClipboardProvider};

// 键盘按键对应的处理函数，比如回车键后复制内容到剪贴板
pub fn pressed_enter(app: &mut App) {
    match app.mode {
        Mode::Update => {
            if let Err(err) = app.update_secret() {
                app.set_error(&err);
            } else {
                app.switch_mode(Mode::Normal)
            }
        }
        Mode::Add => {
            if let Err(err) = app.add_secret() {
                app.set_error(&err);
            } else {
                app.switch_mode(Mode::Normal)
            }
        }
        Mode::Delete => {
            if app.panels.get(&PanelName::DeleteSecret).unwrap().content[0].trim() == "y" {
                if let Err(err) = app.delete_secret() {
                    app.set_error(&err);
                } else {
                    app.switch_mode(Mode::Normal)
                }
            } else {
                app.switch_mode(Mode::Normal)
            }
        }
        Mode::Make => {
            if let Err(err) = app.make_secret() {
                app.set_error(&err);
            } else {
                app.switch_mode(Mode::Normal)
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

// pub fn mask_secrets(app: &mut App) {
//     let (_, secret) = app.get_selected_secret();

//     // Mask the secret by replacing each character with '*'
//     let mut masked = String::with_capacity(secret.len());
//     masked.push_str(&"*".repeat(secret.len()));

//     // Update the secret panel with the masked secret
//     let secrets_panel = app.panels.get_mut(&PanelName::Secrets).unwrap();
//     secrets_panel.content[secrets_panel.index] = masked;
// }