use crate::app::App;
use crate::keymaps;
use crate::panel::PanelName;
use crossterm::event::{KeyCode, KeyEvent};

pub fn parse_keys(app: &mut App, key: KeyEvent) -> Option<()> {
    let filter_panel = app.get_specific_panel(PanelName::Filter);

    match key.code {
        KeyCode::Char(ch) => {
            // cannot borrow `filter_panel.content` as mutable, as it is behind a `&` reference
            // `filter_panel` is a `&` reference, so the data it refers to cannot be borrowed as mutable
            filter_panel.content[0].push(ch);
            app.filter_secrets();
        }
        KeyCode::Backspace => {
            filter_panel.content[0].pop();
            app.filter_secrets();
        }
        KeyCode::Esc => return Some(()),
        KeyCode::Enter => keymaps::pressed_enter(app), // 复杂的处理放到keymaps里去
        KeyCode::Up => {
            // Rust编译器的错误信息 "cannot borrow *app as mutable more than once at a time" 表示你在同一作用域内尝试多次获取 *app 的可变引用。
            // 在Rust中，只能有一个可变引用对同一数据进行修改。这是Rust的借用规则之一，旨在确保数据的安全性和避免竞态条件
            let secrets_panel = app.get_specific_panel(PanelName::Secrets);
            if secrets_panel.index > 0 {
                secrets_panel.index -= 1;
            }
        }
        KeyCode::Down => {
            let secrets_panel = app.get_specific_panel(PanelName::Secrets);
            if secrets_panel.index < secrets_panel.content.len() - 1 {
                secrets_panel.index += 1;
            }
        }
        _ => {}
    }
    None
}