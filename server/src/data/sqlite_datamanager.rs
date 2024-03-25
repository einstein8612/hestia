use crate::{bin::Bin, language::Language};

use super::{DataError, DataManager, Statistics};

use axum::async_trait;
use chrono::NaiveDateTime;
use std::error::Error;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

pub struct SQLiteDataManager {
    pool: Pool<Sqlite>
}

impl SQLiteDataManager {
    pub async fn connect(url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = SqlitePoolOptions::new().max_connections(10).connect(url).await?;
        let manager = SQLiteDataManager{pool};
        manager.setup_database().await?;
        Ok(manager)
    }

    async fn setup_database(&self) -> Result<(), Box<dyn Error>> {
        sqlx::query("CREATE TABLE IF NOT EXISTS Bins(id INTEGER PRIMARY KEY AUTOINCREMENT, key VARCHAR(255) NOT NULL, name VARCHAR(255) NOT NULL, description VARCHAR(255) NOT NULL, filename VARCHAR(255) NOT NULL, content BLOB NOT NULL, language VARCHAR(255) NOT NULL, views BIGINT NOT NULL, created_at DATETIME NOT NULL);")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl DataManager for SQLiteDataManager {
    async fn fetch_statistics(&self) -> Result<Statistics, DataError> {
        let result: Result<(i64, i64, f64, i64), sqlx::Error> = sqlx::query_as("SELECT COUNT(*), SUM(LENGTH(content)-LENGTH(REPLACE(content, x'0A', ''))+1), CAST(SUM(LENGTH(content)) AS float)/CAST(1048576 AS float), SUM(views) FROM bins ORDER BY id DESC LIMIT 1;")
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(row) => Ok(Statistics{
                total_bins: row.0,
                total_lines: row.1,
                total_size_mb: row.2,
                total_views: row.3,
            }),
            Err(_) => Err(DataError::StatisticsError)
        }
    }

    async fn find_bin_by_id(&self, id: i64) -> Result<Bin, DataError> {
        let result: Result<(i64, String, String, String, String, Vec<u8>, String, i64, NaiveDateTime), sqlx::Error> = sqlx::query_as("UPDATE bins SET views=views+1 WHERE id=$1 RETURNING id,key,name,description,filename,content,language,views,created_at;")
            .bind(id)
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(row) => Ok(Bin { id: row.0, key: row.1, name: row.2, description: row.3, filename: row.4, content: row.5, language: Language::from_string(&row.6).unwrap(), views: row.7, created_at: row.8 }),
            Err(_) => Err(DataError::BinNotFound)
        }
    }

    async fn find_bin_by_key(&self, key: &str) -> Result<Bin, DataError> {
        let result: Result<(i64, String, String, String, String, Vec<u8>, String, i64, NaiveDateTime), sqlx::Error> = sqlx::query_as("UPDATE bins SET views=views+1 WHERE key=$1 RETURNING id,key,name,description,filename,content,language,views,created_at;")
            .bind(key)
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(row) => Ok(Bin { id: row.0, key: row.1, name: row.2, description: row.3, filename: row.4, content: row.5, language: Language::from_string(&row.6).unwrap(), views: row.7, created_at: row.8 }),
            Err(_) => Err(DataError::BinNotFound)
        }
    }

    async fn create_bin(&mut self, key: String, name: String, description: String, filename: String, content: Vec<u8>, language: Language, created_at: NaiveDateTime) -> Result<Bin, DataError> {
        let result: (i64,) = sqlx::query_as("INSERT INTO Bins(key, name, description, filename, content, language, views, created_at) VALUES ($1, $2, $3, $4, $5, 0, $6) RETURNING id")
            .bind(&key)
            .bind(&name)
            .bind(&description)
            .bind(&filename)
            .bind(&content)
            .bind(&language.to_string())
            .bind(&created_at)
            .fetch_one(&self.pool)
            .await.unwrap();
        Ok(Bin { id: result.0, key, name, description, filename, content, language, views: 0, created_at })
    }

    async fn delete_bin_by_id(&mut self, _: i64) -> Result<(), DataError> {
        Err(DataError::BinNotFound)
    }
}