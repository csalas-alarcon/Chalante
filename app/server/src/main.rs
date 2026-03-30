// ./app/server/src/main.rs

// Axum Imports
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
// JSON Imports
use serde_json::{json, Value};

// Error Definition
#[derive(Debug)]
enum ApiError {
    NotFound, 
    InvalidInput(String),
    InternalError,
}
// Error Implementation
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND, "Data not found".to_string(),
            ),
            ApiError::InvalidInput(msg) => (
                StatusCode::BAD_REQUEST, msg
            ),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string()
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

// GET ENDPOINTS
// /health
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok", 
        "message": "the server is running",
    }))
}
// /users
async fn list_users() -> Result<Json<Value>, ApiError> {
    Err(ApiError::InternalError)
}
// /users/{id}
async fn get_user(Path(id): Path<u32>) -> Result<Json<Value>, ApiError> {
    // If high ERROR
    if id > 100 {
        return Err(ApiError::NotFound);
    }
    // Else Dummy
    Ok(Json(json!({
        "id": id,
        "name": "User",
    })))
}

// Here we Define Endpoints
fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_check)) // NOTE: No ()
        .route("/users", get(list_users))
        .route("/users/{id}", get(get_user))
}

// Entry Point 
#[tokio::main]
async fn main() {
    // Instantiate the App
    let app = create_app();
    // Make it listen incoming requests
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind tcp listener");
    // Print if it Worked
    println!("Server running on http://localhost:3000");
    // Start Server
    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}