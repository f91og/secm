use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::panel::{Panel, PanelName};
use crate::utils;

#[derive(PartialEq)]  // 这个宏自动生成 PartialEq 实现
pub enum Mode {
    Normal,
    Filter,
    Make,
    Rename,
}

// 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段
// 不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现
pub struct App {
    pub secrets: BTreeMap<String, String>,
    pub panels: HashMap<PanelName, Panel>,
    pub cursor: u8,
    pub mode: Mode,
    pub show_popup: bool,
}

// 为结构体添加方法
impl App {
    pub fn new(secret_file: &str) -> App {
        let secrets = utils::get_secrets(secret_file);
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
    pub fn get_selected_secret(&mut self) -> String {
        let current_index = self.panels.get(&PanelName::Secrets).unwrap().index;
        let current_secret = &self.panels.get(&PanelName::Secrets).unwrap().content[current_index];
        return current_secret.to_owned();
    }
}
