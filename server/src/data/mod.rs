mod memory_datamanager;
use chrono::NaiveDateTime;
pub use memory_datamanager::MemoryDataManager;

mod psql_datamanager;
pub use psql_datamanager::PSQLDataManager;

mod sqlite_datamanager;
pub use sqlite_datamanager::SQLiteDataManager;

use crate::{bin::Bin, language::Language};

use axum::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("bin could not be found")]
    BinNotFound,
    #[error("statistics could not be calculateed")]
    StatisticsError
}

#[derive(Deserialize, Serialize)]
pub struct Statistics {
    pub total_bins: i64,
    pub total_lines: i64,
    pub total_size_mb: f64,
    pub total_views: i64,
}

#[async_trait]
pub trait DataManager: Send + Sync {
    async fn fetch_statistics(&self) -> Result<Statistics, DataError>;
    async fn find_bin_by_id(&self, id: i64) -> Result<Bin, DataError>;
    async fn find_bin_by_key(&self, key: &str) -> Result<Bin, DataError>;
    async fn create_bin(&mut self, key: String, name: String, description: String, filename: String, content: Vec<u8>, language: Language, created_at: NaiveDateTime) -> Result<Bin, DataError>;
    async fn delete_bin_by_id(&mut self, id: i64) -> Result<(), DataError>;
}
