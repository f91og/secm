use crate::app::App;
use crate::panel::PanelName;
use crate::utils;
// use std::io::{BufRead, BufReader};
// use crate::commands;
use clipboard::{ClipboardContext, ClipboardProvider};

// pub fn pressed_enter(app: &mut App) {
//     // TODO beautify (currently ugly)
//     let item = &app.get_panel().content.clone()[app.cursor as usize];
//     app.panels.get_mut(&PanelName::Status).unwrap().content = vec![item.to_owned()];

//     match app.current_panel {
//         PanelName::Commands => {
//             match item.as_str() {
//                 "clippy" => commands::do_command(app, "clippy"),
//                 "fmt" => commands::do_command(app, "fmt"),
//                 _ => {}
//             }
//             // println!("{}", item);
//         }
//         _ => {}
//     };
// }

// 键盘按键对应的处理函数，比如回车键后复制内容到剪贴板
pub fn pressed_enter(app: &mut App) {
    let secrets_panel = app.panels.get(&PanelName::Secrets).unwrap();
    let selected_index = secrets_panel.index;
    let secret_name = &secrets_panel.content[selected_index];
    let secret = utils::get_secret(&secret_name, "/Users/xue.a.yu/.psm_secret").unwrap();
    // 复制到剪贴板
    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard.set_contents(secret).unwrap();
}