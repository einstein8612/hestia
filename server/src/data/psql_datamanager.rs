use crate::{bin::Bin, language::Language};

use super::{DataError, DataManager, Statistics};

use axum::async_trait;
use chrono::NaiveDateTime;
use std::error::Error;
use std::str::FromStr;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};
use sqlx::ConnectOptions;

pub struct PSQLDataManager {
    pool: Pool<Postgres>
}

impl PSQLDataManager {
    pub async fn connect(url: &str) -> Result<Self, Box<dyn Error>> {
        let connect_options = PgConnectOptions::from_str(url)?
            .disable_statement_logging();
        let pool = PgPoolOptions::new().max_connections(100).connect_with(connect_options).await?;
        let manager = PSQLDataManager{pool};
        manager.setup_database().await?;
        Ok(manager)
    }

    async fn setup_database(&self) -> Result<(), Box<dyn Error>> {
        sqlx::query("CREATE TABLE IF NOT EXISTS Bins(id BIGSERIAL PRIMARY KEY, key VARCHAR(255) NOT NULL, name VARCHAR(255) NOT NULL, description VARCHAR(255) NOT NULL, filename VARCHAR(255) NOT NULL, content BYTEA NOT NULL, language VARCHAR(255) NOT NULL, views BIGINT NOT NULL, created_at TIMESTAMP NOT NULL);")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl DataManager for PSQLDataManager {
    async fn fetch_statistics(&self) -> Result<Statistics, DataError> {
        let result: Result<(i64, i64, f64, i64), sqlx::Error> = sqlx::query_as("SELECT COUNT(*) AS bins, SUM(CHAR_LENGTH(CONVERT_FROM(content, 'UTF8'))-CHAR_LENGTH(REPLACE(CONVERT_FROM(content, 'UTF8'), '\n', ''))+1) AS lines, CAST(SUM(PG_COLUMN_SIZE(content)) AS float)/CAST(1048576 AS float) AS size_mb, CAST(SUM(views) AS BIGINT) AS views FROM bins;")
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
        let result: (i64,) = sqlx::query_as("INSERT INTO Bins(key, name, description, filename, content, language, views, created_at) VALUES ($1, $2, $3, $4, $5, $6, 0, $7) RETURNING id")
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