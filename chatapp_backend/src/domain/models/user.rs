use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub userId: Option<i32>,
    pub username: String,
    pub password: String,
    pub email: String,
    pub activated: Option<bool>
}