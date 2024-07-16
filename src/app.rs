use std::time::Duration;

use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::KeyEvent;
use ratatui::{
    crossterm::event::{KeyCode, KeyEventKind},
    widgets::{
        ListItem, ListState,
    },
};


use std::collections::HashMap;
use std::time::Instant;

use crate::panel::{Panel, PanelName};

use crate::utils;

pub const GUIDE_NORMAL: &str = "d: delete, a: add secret, m: make secret, enter: copy to clipboard, /: filter secrets, r: update, q: quit";
pub const GUIDE_ADD: &str = "enter: confirm, tab: switch input, esc: cancel";
pub const GUIDE_RENAME: &str = "enter: rename secret, esc: cancel";
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

pub struct App<'a> {
    pub should_exit: bool,
    pub secret_list: SecretList,
    pub panels: HashMap<PanelName, Panel>,
    // pub cursor: u8,
    pub mode: Mode,
    pub guide: &'a str,
    pub error: &'a str,
    error_timer: Option<Instant>,
}

pub struct SecretList {
    pub secrets: Vec<SecretItem>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct SecretItem {
    pub name: String,
    pub value: String,
}

//<'a> App<'a>
impl<'a> Default for App<'a> {
    fn default() -> Self { // Self是App的类型的别名
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
        Self {
            should_exit: false,
            secret_list: SecretList::from_iter(utils::get_secrets()),
            mode: Mode::Normal,
            panels,
            guide: GUIDE_NORMAL,
            error: "",
            error_timer: None,
        }
    }
}

impl FromIterator<(String, String)> for SecretList {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        let secrets = iter
            .into_iter()
            .map(|(name, value)| SecretItem::new(name, value))
            .collect();
        let state = ListState::default();
        Self { secrets, state } // 这里的secrets为什么要和结构体中的匿名字段名一致？
    }
}

impl SecretItem {
    fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
        }
    }
}

impl<'a> App<'a> {
    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Char('/') => self.switch_mode(Mode::Filter),
            KeyCode::Char('r') => self.switch_mode(Mode::Update),
            KeyCode::Char('m') => self.switch_mode(Mode::Make),
            KeyCode::Char('a') => self.switch_mode(Mode::Add),
            KeyCode::Char('d') => self.switch_mode(Mode::Delete),
            KeyCode::Enter => {
                self.handle_enter();
            }
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.secret_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.secret_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.secret_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.secret_list.state.select_last();
    }

   pub fn get_filter_string(&mut self) -> String {
        let panel = self.panels.get(&PanelName::Filter).unwrap();
        let filter = panel.content[0].clone();
        return filter;
    }

    fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
        match self.mode {
            Mode::Add => self.guide = GUIDE_ADD,
            Mode::Make => self.guide = GUIDE_MAKE,
            Mode::Delete => self.guide = GUIDE_DELETE,
            Mode::Filter => {
                if let Some(i) = self.secret_list.state.selected() {
                    let item = &self.secret_list.secrets[i];
                    let name = item.name.clone();
                    let value = item.value.clone();
                    self.panels.get_mut(&PanelName::UpdateSecret).unwrap().content[0] = name;
                    self.panels.get_mut(&PanelName::UpdateSecret).unwrap().content[1] = value;
                    self.guide = GUIDE_RENAME;
                }
            }
            Mode::Normal => {
                self.guide = GUIDE_NORMAL;
                self.error = "";
                self.panels.get_mut(&PanelName::UpdateSecret).unwrap().clear_content();
                self.panels.get_mut(&PanelName::Filter).unwrap().clear_content();
                self.panels.get_mut(&PanelName::AddSecret).unwrap().clear_content();
                self.panels.get_mut(&PanelName::DeleteSecret).unwrap().clear_content();
            }
            _ => {},
        }
    }

    fn handle_enter(&mut self) {
        match self.mode {
            Mode::Normal => {
                self.copy_selected_to_clipboard();
                self.should_exit = true;
            }
            Mode::Filter => {}
            Mode::Make => {}
            Mode::Add => {}
            Mode::Update => {}
            Mode::Delete => {}
        }
    }

    pub fn get_selected_secret(&mut self) -> &str {
        if let Some(i) = self.secret_list.state.selected() {
            let item = &self.secret_list.secrets[i];
            return &item.value;
        }
        ""
    }

    fn copy_selected_to_clipboard(&mut self) {
        let secret = self.get_selected_secret();
        if secret != "" {
            let mut clipboard = ClipboardContext::new().unwrap();
            clipboard.set_contents(secret.to_string()).unwrap();
        }
    }

    pub fn clear_error_if_expired(&mut self) {
        if let Some(timer) = self.error_timer {
            if timer.elapsed() >= Duration::from_secs(3) {
                self.error = "";
                self.error_timer = None;
            }
        }
    }
}

impl From<&SecretItem> for ListItem<'_> {
    fn from(secret: &SecretItem) -> Self {
        let line = secret.name.clone();
        ListItem::new(line)
    }
}