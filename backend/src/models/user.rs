use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}
