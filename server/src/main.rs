// "Primitive" data types
mod bin;
mod language;

// Logging
mod logger;
use logger::setup_logger;

// Config
mod config;
use config::DatabaseType;


// Key generation
mod key;

// Data Manager
mod data;
use data::{DataManager, MemoryDataManager, PSQLDataManager, SQLiteDataManager};

// API Server
mod api;
use api::APIServer;

use log::info;
use tokio::sync::Mutex;
use std::{sync::Arc, time::Instant};

#[tokio::main]
async fn main() {
    setup_logger().expect("Failed to setup logger");

    let start = Instant::now();
    let config = match config::read_config() {
        Ok(cfg) => cfg,
        Err(err) => {
            println!("There was an error reading the config: {}", err);
            return;
        }
    };
    info!("Loaded configuration file successfully");

    let data_manager: Box<dyn DataManager + Send + Sync> = match config.database.database_type {
        DatabaseType::SQLite => Box::from(SQLiteDataManager::connect(&config.database.database_uri).await.expect("Connection to database failed")),
        DatabaseType::Postgres => Box::from(PSQLDataManager::connect(&config.database.database_uri).await.expect("Connection to database failed")),
        DatabaseType::Memory => Box::from(MemoryDataManager::new())
    };
    info!("Connected to the database ({:?}) successfully", config.database.database_type);

    // data_manager.create_bin("new_key".to_owned(), "new bin".to_owned(), "random\nlines".as_bytes().to_vec(), Language::Rust, Utc::now().naive_utc()).await;

    info!("Starting API server, time taken: {:.3}ms", start.elapsed().as_nanos() as f64 / 1_000_000.0);
    let server = APIServer::new(Arc::from(Mutex::from(data_manager)));
    info!("Binding to http://{}", &config.api.bind);
    server.start(config.api.bind.leak()).await;
}
