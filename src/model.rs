use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Secret {
    pub name: String,
    pub value: String,
}