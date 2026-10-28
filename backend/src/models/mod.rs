use serde::Serialize;

pub mod team;
pub mod user;

#[derive(Debug, Serialize)]
/// One page of a collection. `total` is the size of the filtered set, not the length of `items`.
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: u64,
}
