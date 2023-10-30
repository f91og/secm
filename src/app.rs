use std::collections::HashMap;

use crate::panel::{Panel, PanelName};
use crate::utils;

// 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段
// 不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现
pub struct App {
    /// Current value of the input box
    // pub input: String,
    pub secrets: Vec<String>,
    pub panels: HashMap<PanelName, Panel>,
    pub cursor: u8,
    // pub show_popup: bool,
    // pub secrets: Vec<String>, // 存放一些数据或者 UI 状态
    // pub len_after_filtered: usize, // 过滤后的数据长度
    // pub selected_secret_index: usize, // 选中的索引
}

// 为结构体添加方法
impl App {
    pub fn new(secret_file: &str) -> App {
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
                    content: utils::get_secret_names(secret_file),
                }
            ),
        ]);
        App {
            secrets: utils::get_secret_names(secret_file),
            panels,
            cursor: 0,
            // show_popup: false,
            // secrets: Vec::new(),
            // len_after_filtered: 0,
            // selected_secret_index: 0,
        }
    }

    // pub fn get_panel(&mut self) -> &mut Panel {
    //     self.panels.get_mut(&self.current_panel).unwrap()
    // }

    pub fn get_specific_panel(&mut self, name: PanelName) -> &mut Panel {
        self.panels.get_mut(&name).unwrap()
    }

    pub fn filter_secrets(&mut self) {
        let _keyword = &self.panels.get(&PanelName::Filter).unwrap().content[0];
        if _keyword.trim() != "" {
            self.panels.get_mut(&PanelName::Secrets).unwrap().content = self.secrets.iter().filter(|s| s.contains(_keyword)).cloned().collect();
        } else {
            self.panels.get_mut(&PanelName::Secrets).unwrap().content = self.secrets.clone();
        }
    }
}

