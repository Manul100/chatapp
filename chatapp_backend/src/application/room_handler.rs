use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{api::router::AppState, domain::models::chat_model::*};



pub async fn create_room(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateRoomSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
}