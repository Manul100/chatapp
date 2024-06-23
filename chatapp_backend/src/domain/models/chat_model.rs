use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::Type;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Room {
    pub room_id: Uuid,
    pub room_name: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub message_id: Uuid,
    pub content: String,
    pub room_id: Uuid,
    pub user_id: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    Creator,
    Admin,
    Member
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRoom {
    pub user_room_id: Uuid,
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub role: UserRole,
    #[serde(rename = "joinedAt")]
    pub joined_at: Option<DateTime<Utc>>
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomSchema {
    pub room_name: String,
    pub user_id: Uuid,
    pub description: String
}