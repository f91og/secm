pub mod app;
pub mod utils;
pub mod panel;
pub mod ui;
pub mod cmds;
pub mod handle_keys;
pub mod storage;

pub trait Storage {
    fn write(&self, key: &str, value: &str) -> Result<(), String>;
    fn read(&self, key: &str) -> Result<Option<String>, String>;
    fn update(&self, key: &str, value: &str) -> Result<(), String>;
    fn get_all(&self) -> Result<Vec<(String, String)>, String>;
    fn delete(&self, key: &str) -> Result<(), String>;
}
