use std::io::BufRead;

use crate::{bin::Bin, language::Language};

use super::{DataError, DataManager, Statistics};

use axum::async_trait;
use chrono::NaiveDateTime;

pub struct MemoryDataManager {
    bins: Vec<Bin>
}


impl MemoryDataManager {
    pub fn new() -> MemoryDataManager {
        MemoryDataManager {
            bins: Vec::new()
        }
    }
}

#[async_trait]
impl DataManager for MemoryDataManager {
    async fn fetch_statistics(&self) -> Result<Statistics, DataError> {
        Ok(Statistics{
            total_bins: self.bins.len() as i64,
            total_lines: self.bins.iter().map(|bin| bin.content.lines().count() as i64).sum(),
            total_size_mb: self.bins.iter().map(|bin| bin.content.len() as f64).sum::<f64>()/((1<<20) as f64),
            total_views: self.bins.iter().map(|bin| bin.views).sum()
        })
    }

    async fn find_bin_by_id(&self, id: i64) -> Result<Bin, DataError> {
        if let Some(bin) = self.bins.iter().find(|bin| bin.id == id) {
            return Ok(bin.clone());
        }
        Err(DataError::BinNotFound)
    }

    async fn find_bin_by_key(&self, key: &str) -> Result<Bin, DataError> {
        if let Some(bin) = self.bins.iter().find(|bin| bin.key == key) {
            return Ok(bin.clone());
        }
        Err(DataError::BinNotFound)
    }

    async fn create_bin(&mut self, key: String, name: String, description: String, filename: String, content: Vec<u8>, language: Language, created_at: NaiveDateTime) -> Result<Bin, DataError> {
        let new_bin = Bin{
            id: self.bins.len() as i64,
            key,
            name,
            description,
            filename,
            content,
            language,
            views: 0,
            created_at
        };

        self.bins.push(new_bin.clone());
        Ok(new_bin)
    }

    async fn delete_bin_by_id(&mut self, id: i64) -> Result<(), DataError> {
        if let Some(index) = self.bins.iter().position(|bin| bin.id != id) {
            self.bins.swap_remove(index);
            return Ok(());
        }
        Err(DataError::BinNotFound)
    }
}