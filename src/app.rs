use std::time::Duration;

use clipboard::{ClipboardContext, ClipboardProvider};
use ratatui::{
    crossterm::event::{KeyEventKind, KeyEvent},
    widgets::{
        ListItem, ListState,
    },
};

use std::collections::HashMap;
use std::time::Instant;

use crate::panel::{Panel, PanelName};
use crate::handle_keys::*;
use crate::Storage;
use crate::model::Secret;

pub const GUIDE_NORMAL: &str = "d: delete, a: add secret, m: make secret, enter: copy to clipboard, /: filter secrets, r: update, q: quit";
pub const GUIDE_ADD: &str = "enter: confirm, tab: switch input, esc: cancel";
pub const GUIDE_UPDATE: &str = "enter: update secret, esc: cancel";
pub const GUIDE_DELETE: &str = "enter: confirm, esc: cancel";
pub const GUIDE_MAKE: &str = "enter: make secret, esc: cancel, tab: switch input";

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Filter,
    Make,
    Add,
    Update,
    Delete,
}

pub struct App<S: Storage> {
    pub should_exit: bool,
    pub secrets:  Vec<(String, String)>,
    pub secret_list: SecretList,
    pub panels: HashMap<PanelName, Panel>,
    // pub cursor: u8,
    pub mode: Mode,
    pub guide: &'static str,
    pub error: AppErr,
    pub storage: S,
}

pub struct AppErr {
    pub msg: String,
    pub error_timer: Option<Instant>,
}

pub struct SecretList {
    pub secrets: Vec<Secret>,
    pub state: ListState,
}

impl FromIterator<(String, String)> for SecretList {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        let secrets = iter
            .into_iter()
            .map(|(name, value)| Secret::new(name, value))
            .collect();
        let state = ListState::default();
        Self { secrets, state } // 这里的secrets为什么要和结构体中的匿名字段名一致？
    }
}

impl Secret {
    fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
        }
    }
}

impl<S: Storage> App<S> {
    pub fn new(storage: S) -> Self { // Self是App的类型的别名
        let panels = HashMap::from([
            (
                PanelName::Filter,
                Panel {
                    index: 0,
                    panel_name: PanelName::Filter,
                    content: vec!["".to_string()],
                }
            ),
            (
                PanelName::UpdateSecret,
                Panel {
                    index: 0,
                    panel_name: PanelName::UpdateSecret,
                    content: vec!["".to_string(), "".to_string()],
                }
            ),
            (
                PanelName::MakeSecret,
                Panel {
                    index: 0,
                    panel_name: PanelName::MakeSecret,
                    content: vec!["".to_string(), "10".to_string(), "n".to_string()],
                }
            ),
            (
                PanelName::AddSecret,
                Panel {
                    index: 0,
                    panel_name: PanelName::AddSecret,
                    content: vec!["".to_string(), "".to_string()],
                }
            ),
            (
                PanelName::DeleteSecret,
                Panel {
                    index: 0,
                    panel_name: PanelName::DeleteSecret,
                    content: vec!["n".to_string()],
                }
            )
        ]);
        let all_secrets = storage
            .get_all()
            .unwrap_or_else(|err| {
                eprintln!("Failed to load secrets from storage: {}", err);
                vec![]
            });
        let secret_list = SecretList::from_iter(all_secrets.clone());

        Self {
            should_exit: false,
            secrets: all_secrets,
            secret_list,
            panels,
            mode: Mode::Normal,
            guide: GUIDE_NORMAL,
            error: AppErr {
                msg: "".to_string(),
                error_timer: None,
            },
            storage,
        }
    }

    pub fn filter_secrets_list(&mut self, filter: &str) {
        let filtered_secrets: Vec<(String, String)> = self.secrets.clone()
            .into_iter()
            .filter(|(name, _)| name.contains(filter))
            .collect();

        self.secret_list = SecretList::from_iter(filtered_secrets);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match self.mode {
            Mode::Filter => handle_key_in_filter_mode(self, key),
            Mode::Add => handle_key_in_add_mode(self, key),
            Mode::Normal =>handle_key_in_normal_mode(self, key),
            Mode::Make => handle_key_in_make_mode(self, key),
            Mode::Update => handle_key_in_update_mode(self, key),
            Mode::Delete => handle_key_in_delete_mode(self, key),
        }
    }

    pub fn get_panel(&mut self, panel_name: PanelName) -> &mut Panel {
        self.panels.get_mut(&panel_name).unwrap()
    }

    pub fn select_next(&mut self) {
        self.secret_list.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.secret_list.state.select_previous();
    }

    // fn select_first(&mut self) {
    //     self.secret_list.state.select_first();
    // }

    // fn select_last(&mut self) {
    //     self.secret_list.state.select_last();
    // }

    pub fn get_filter_string(&mut self) -> String {
        let panel = self.panels.get(&PanelName::Filter).unwrap();
        let filter = panel.content[0].clone();
        return filter;
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
        match self.mode {
            Mode::Add => self.guide = GUIDE_ADD,
            Mode::Make => self.guide = GUIDE_MAKE,
            Mode::Delete => self.guide = GUIDE_DELETE,
            Mode::Update => {
                if let Some(secret) = self.get_selected_item() {
                    let update_secret_panel = self.get_panel(PanelName::UpdateSecret);
                    update_secret_panel.content[0] = secret.name;
                    update_secret_panel.content[1] = secret.value;
                    self.guide = GUIDE_UPDATE;
                }
            }
            Mode::Normal => {
                self.guide = GUIDE_NORMAL;
                self.error.msg.clear();
                self.panels.get_mut(&PanelName::UpdateSecret).unwrap().clear_content();
                self.panels.get_mut(&PanelName::Filter).unwrap().clear_content();
                self.panels.get_mut(&PanelName::AddSecret).unwrap().clear_content();
                self.panels.get_mut(&PanelName::DeleteSecret).unwrap().clear_content();
            }
            _ => {},
        }
    }

    pub fn get_selected_item(&mut self) ->  Option<Secret>  {
        if let Some(i) = self.secret_list.state.selected() {
            let item = self.secret_list.secrets[i].clone();
            Some(item)
        } else {
            None
        }
    }

    pub fn copy_selected_to_clipboard(&mut self) {
        if let Some(secret) = self.get_selected_item() {
            let mut clipboard = ClipboardContext::new().unwrap();
            clipboard.set_contents(secret.value).unwrap();
        } else {
            self.error = AppErr{msg: "No secret selected".to_string(), error_timer: Some(Instant::now())};
        }
    }

    pub fn clear_error_if_expired(&mut self) {
        if let Some(timer) = self.error.error_timer {
            if timer.elapsed() >= Duration::from_secs(3) {
                self.error.msg.clear();
                self.error.error_timer = None;
            }
        }
    }

    pub fn add_secret(&mut self, name: String, value: String) -> Result<(), String> {
        if name.is_empty() || value.is_empty() {
            return Err("Name, value and cannot be empty".to_string());
        }

        if self.secrets.iter().any(|s| s.0 == name) {
            return Err("Secret already exists".to_string());
        }
        self.storage.write(&name, &value)?;

        self.secrets.push((name, value));
        self.secret_list= SecretList::from_iter(self.secrets.clone());

        Ok(())
    }

    pub fn update_selected_secret(&mut self) -> Result<(), String> {
        if let Some(i)  = self.secret_list.state.selected() {
            let original_name = &self.secrets[i].0;
            let update_secret_panel = self.panels.get_mut(&PanelName::UpdateSecret).unwrap();
            let name = update_secret_panel.content[0].trim();
            let value = update_secret_panel.content[1].trim();

            if name.is_empty() || value.is_empty() {
                return Err("Name and value cannot be empty".to_string());
            }

            // 在 Rust 中，.expect("Failed to update secret") 是一种用于处理 Result 或 Option 类型的方式。它会检查 Result 是否是 Ok 或 Some，如果是，它会继续执行；如果不是（即为 Err 或 None），则会终止程序，并打印给定的错误消息（例如 "Failed to update secret"），然后 panic（引发恐慌）。
            //所以，.expect() 不会将错误信息传递到上层，它会使程序在遇到错误时崩溃。如果你希望错误信息能够传递到上层，而不是让程序崩溃，你可以使用 ? 运算符来传播错误
            //  self.storage.delete(name).expect("update failed")
            self.storage.delete(original_name)?;
            self.storage.write(name, value)?;
            self.secrets[i] = (name.to_string(), value.to_string());
            self.secret_list = SecretList::from_iter(self.secrets.clone());
            return Ok(())
        }
        Err("No secret selected".to_string())
    }

    pub fn delete_selected_secret(&mut self) -> Result<(), String> {
        if let Some(i) = self.secret_list.state.selected() {
            self.storage.delete(&self.secrets[i].0)?;
            self.secrets.remove(i);
            self.secret_list.secrets.remove(i);
            return Ok(())
        }
        Err("No secret selected".to_string())
    }
}

impl From<&Secret> for ListItem<'_> {
    fn from(secret: &Secret) -> Self {
        let line = secret.name.clone();
        ListItem::new(line)
    }
}

// fn secret_items_to_strings(secrets: &Vec<SecretItem>) -> Vec<String> {
//     let mut result = Vec::new();
//     for secret in secrets {
//         result.push(format!("{}: {}", secret.name, secret.value));
//     }
//     result
// }