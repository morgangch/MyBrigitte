use serde::Serialize;

pub mod user;

#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: u64,
}
