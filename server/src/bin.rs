use crate::language::Language;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Bin {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub description: String,
    pub filename: String,
    pub content: Vec<u8>,
    pub language: Language,
    pub views: i64,
    pub created_at: NaiveDateTime,
}