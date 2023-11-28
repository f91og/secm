use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::panel::{Panel, PanelName};
use crate::utils;

#[derive(PartialEq)]  // 这个宏自动生成 PartialEq 实现
pub enum Mode {
    Normal,
    Filter,
    Make,
    Add,
    Rename,
    Delete,
}

// 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段
// 不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现
// App负责管理状态数据，并提供方法来修改状态
pub struct App {
    pub secrets: BTreeMap<String, String>,
    pub panels: HashMap<PanelName, Panel>,
    pub cursor: u8,
    pub mode: Mode,
    pub show_popup: bool,
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
                PanelName::RenameSecret,
                Panel {
                    index: 0,
                    panel_name: PanelName::RenameSecret,
                    content: vec!["".to_string(), "".to_string()],
                }
            ),
            (
                PanelName::MakeSecret,
                Panel {
                    index: 0,
                    panel_name: PanelName::MakeSecret,
                    content: vec!["".to_string()],
                }
            ),
            (
                PanelName::AddSecret,
                Panel {
                    index: 0,
                    panel_name: PanelName::AddSecret,
                    content: vec!["".to_string(), "".to_string(), "".to_string()],
                }
            ),
            (
                PanelName::DeleteSecret,
                Panel {
                    index: 0,
                    panel_name: PanelName::DeleteSecret,
                    content: vec!["".to_string()],
                }
            )
        ]);
        App {
            secrets: secrets,
            panels,
            cursor: 0,
            mode: Mode::Normal,
            show_popup: false,
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
        utils::sync_secrets_to_file(&self.secrets);
        Ok(())
    }

    pub fn rename_secret(&mut self) {
        let (current_secret, _)  = self.get_selected_secret();
        let rename_secret_panel = self.panels.get_mut(&PanelName::RenameSecret).unwrap();
        let new_secret_name = rename_secret_panel.content[0].trim();
        let secret_value = self.secrets.get(&current_secret).unwrap();
        if self.secrets.contains_key(new_secret_name) {
            rename_secret_panel.content[1] = "Secret already exists".to_string();
        } else if new_secret_name.is_empty() {
            rename_secret_panel.content[1] = "Name cannot be empty".to_string();
        } else {
            self.secrets.insert(new_secret_name.to_string(), secret_value.to_string());
            self.secrets.remove(&current_secret); // this must after line 104, after immutable borrow by secret_value is dropped
            utils::sync_secrets_to_file(&self.secrets);
        }
    }

    pub fn back_to_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.panels.get_mut(&PanelName::RenameSecret).unwrap().clear_content();
        self.panels.get_mut(&PanelName::Filter).unwrap().clear_content();
        self.panels.get_mut(&PanelName::Secrets).unwrap().content = self.secrets.keys().cloned().collect();
        self.panels.get_mut(&PanelName::AddSecret).unwrap().clear_content();
        self.panels.get_mut(&PanelName::DeleteSecret).unwrap().clear_content();
    }

    pub fn add_secret (&mut self) {
        let add_secret_panel = self.panels.get_mut(&PanelName::AddSecret).unwrap();
        let new_secret_name = add_secret_panel.content[0].trim();
        let new_secret_value = add_secret_panel.content[1].trim();
        if new_secret_name.is_empty() || new_secret_value.is_empty() {
            add_secret_panel.content[2] = "Name and value cannot be empty".to_string();
        }else if self.secrets.contains_key(new_secret_name) {
            add_secret_panel.content[2] = "Secret already exists".to_string();
        } else {
            self.secrets.insert(new_secret_name.to_string(), new_secret_value.to_string());
            utils::sync_secrets_to_file(&self.secrets);
        }
    }
}
