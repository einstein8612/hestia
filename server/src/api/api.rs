use crate::data::DataManager;

use super::routes::*;
use super::middleware::*;

use std::sync::Arc;

use axum::middleware;
use axum::routing::post;
use axum::{routing::get, Router};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub data_manager: Arc<Mutex<Box<dyn DataManager + Send + Sync>>>,
}

pub struct APIServer {
    router: Router,
}

impl APIServer {
    pub fn new(data_manager: Arc<Mutex<Box<dyn DataManager + Send + Sync>>>) -> Self {
        APIServer{
            router: Self::build_router()
                .with_state(
                    AppState{
                        data_manager,
                    }),
        }
    }

    pub async fn start(self, addr: &'static str) {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.router).await.unwrap();
    }

    fn build_router() -> Router<AppState> {
        Router::new()
            .route("/statistics", get(get_statistics))
            .route("/bin/:bin_key", get(get_bin_by_key))
            .route("/bin", post(create_bin))
            .layer(middleware::from_fn(global_middleware))
            .layer(middleware::from_fn(cors_middleware))
    }
}