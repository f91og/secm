use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::panel::{Panel, PanelName};
use crate::utils;
use crate::utils::get_secret_file_path;

#[derive(PartialEq)]  // 这个宏自动生成 PartialEq 实现
pub enum Mode {
    Normal,
    Filter,
    Make,
    Add,
    Update,
    Delete,
}

pub const GUIDE_NORMAL: &str = "d: delete, a: add secret, m: make secret, enter: copy to clipboard, /: filter secrets, r: rename, q: quit";
pub const GUIDE_ADD: &str = "enter: confirm, tab: switch input, esc: cancel";
pub const GUIDE_RENAME: &str = "enter: rename secret, esc: cancel";
pub const GUIDE_DELETE: &str = "enter: confirm, esc: cancel";
pub const GUIDE_MAKE: &str = "enter: make secret, esc: cancel, tab: switch input";

// 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段
// 不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现
// App负责管理状态数据，并提供方法来修改状态
pub struct App {
    pub secrets: BTreeMap<String, String>,
    pub panels: HashMap<PanelName, Panel>,
    pub cursor: u8,
    pub mode: Mode,
    pub show_popup: bool,
    pub guide: String,
    pub error: String,
    pub error_timer: Option<Instant>,
}

// 为结构体添加方法
impl App {
    pub fn new() -> App {
        let secrets = utils::get_secrets(&utils::get_secret_file_path());
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
                PanelName::Secrets,
                Panel {
                    index: 0,   // 这个是数据行的索引，任何一个tui窗口都可以抽象成一个多行文本
                    panel_name: PanelName::Secrets,
                    content: secrets.keys().cloned().collect(),
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
        App {
            secrets: secrets,
            panels,
            cursor: 0,
            mode: Mode::Normal,
            show_popup: false,
            guide: GUIDE_NORMAL.to_string(),
            error: String::new(),
            error_timer: None,
        }
    }

    pub fn filter_secrets_panel(&mut self) {
        let _keyword = &self.panels.get(&PanelName::Filter).unwrap().content[0];
        if _keyword.trim() != "" {
            self.panels.get_mut(&PanelName::Secrets).unwrap().content = self.secrets
                .iter()
                .filter(|(key, _)| key.contains(_keyword))
                .map(|(key, _)| key.clone())
                .collect();
        } else {
            self.panels.get_mut(&PanelName::Secrets).unwrap().content = self.secrets.keys().cloned().collect();
        }
        self.panels.get_mut(&PanelName::Secrets).unwrap().index = 0;
    }

    // get current secret in Secrets Panel
    pub fn get_selected_secret(&mut self) -> (String, String) {
        let current_index = self.panels.get(&PanelName::Secrets).unwrap().index;
        let name = &self.panels.get(&PanelName::Secrets).unwrap().content[current_index];
        let value = self.secrets.get(name).unwrap();
        return (name.to_owned(), value.to_owned());
    }

    pub fn delete_secret(&mut self) -> Result<(), String> {
        let (current_secret, _) = self.get_selected_secret();
        if self.secrets.remove(&current_secret).is_none() {
            return Err("Secret not found".to_string());
        }
        self.panels.get_mut(&PanelName::Secrets).unwrap().content = self.secrets.keys().cloned().collect();
        utils::sync_secrets_to_file(&self.secrets, &get_secret_file_path());
        Ok(())
    }

    pub fn update_secret(&mut self) -> Result<(), String> {
        let (current_secret, _)  = self.get_selected_secret();
        let update_secret_panel = self.panels.get_mut(&PanelName::UpdateSecret).unwrap();
        let secret_name = update_secret_panel.content[0].trim();
        let secret_value = update_secret_panel.content[1].trim();
        // let secret_value = self.secrets.get(&current_secret).unwrap();
        if self.secrets.contains_key(secret_name) {
            return Err("Secret already exists".to_string());
        } else if secret_name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        self.secrets.insert(secret_name.to_string(), secret_value.to_string());
        self.secrets.remove(&current_secret); // this must after line 104, after immutable borrow by secret_value is dropped
        utils::sync_secrets_to_file(&self.secrets, &get_secret_file_path());
        Ok(())
    }

    pub fn add_secret (&mut self) -> Result<(), String> {
        let add_secret_panel = self.panels.get_mut(&PanelName::AddSecret).unwrap();
        let new_secret_name = add_secret_panel.content[0].trim();
        let new_secret_value = add_secret_panel.content[1].trim();
        if new_secret_name.is_empty() || new_secret_value.is_empty() {
            return Err("Name and value cannot be empty".to_string());
        }
        if self.secrets.contains_key(new_secret_name) {
            return Err("Secret already exists".to_string());
        }
        self.secrets.insert(new_secret_name.to_string(), new_secret_value.to_string());
        utils::sync_secrets_to_file(&self.secrets, &get_secret_file_path());
        Ok(())
    }

    pub fn make_secret(&mut self) -> Result<(), String> {
        let make_secret_panel = self.panels.get_mut(&PanelName::MakeSecret).unwrap();
        let name = make_secret_panel.content[0].trim();
        let length = make_secret_panel.content[1].trim();
        let advance = make_secret_panel.content[2].trim();
        if name.is_empty() || length.is_empty() || advance.is_empty() {
            return Err("Name, length and advance cannot be empty".to_string());
        }
        if self.secrets.contains_key(name) {
            return Err("Secret already exists".to_string());
        }

        let n = match length.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                return Err("Length must be number".to_string());
            }
        };

        let new_secret_value = utils::generate_random_string(n, advance == "yes" || advance == "y");
        self.secrets.insert(name.to_string(), new_secret_value);
        utils::sync_secrets_to_file(&self.secrets, &get_secret_file_path());
        Ok(())
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
        match self.mode {
            Mode::Add => self.guide = GUIDE_ADD.to_string(),
            Mode::Make => self.guide = GUIDE_MAKE.to_string(),
            Mode::Delete => self.guide = GUIDE_DELETE.to_string(),
            Mode::Update => {
                let (name, value) = self.get_selected_secret();
                self.panels.get_mut(&PanelName::UpdateSecret).unwrap().content[0] = name;
                self.panels.get_mut(&PanelName::UpdateSecret).unwrap().content[1] = value;
                self.guide = GUIDE_RENAME.to_string()
            }
            Mode::Normal => {
                self.guide = GUIDE_NORMAL.to_string();
                self.error = "".to_string();
                self.panels.get_mut(&PanelName::UpdateSecret).unwrap().clear_content();
                self.panels.get_mut(&PanelName::Filter).unwrap().clear_content();
                self.panels.get_mut(&PanelName::Secrets).unwrap().content = self.secrets.keys().cloned().collect();
                self.panels.get_mut(&PanelName::AddSecret).unwrap().clear_content();
                self.panels.get_mut(&PanelName::DeleteSecret).unwrap().clear_content();
            }
            _ => {},
        }
    }

    pub fn set_error(&mut self, error: &str) {
        self.error = error.to_string();
        self.error_timer = Some(Instant::now());
    }

    pub fn clear_error_if_expired(&mut self) {
        if let Some(timer) = self.error_timer {
            if timer.elapsed() >= Duration::from_secs(3) {
                self.error.clear();
                self.error_timer = None;
            }
        }
    }
}
