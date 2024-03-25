use axum::{extract::Request, http::{Method, StatusCode}, middleware::Next, response::{IntoResponse, Response}};

pub async fn global_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    response.headers_mut().insert("Server", format!("Hestia v{}", env!("CARGO_PKG_VERSION")).parse().unwrap());
    response.headers_mut().insert("X-Contribute", env!("CARGO_PKG_REPOSITORY").parse().unwrap());

    response
}

pub async fn cors_middleware(request: Request, next: Next) -> Response {
    let mut response = match *request.method() {
        Method::OPTIONS => StatusCode::OK.into_response(),
        _ => next.run(request).await
    };

    response.headers_mut().insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap());
    response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization".parse().unwrap());

    response
}