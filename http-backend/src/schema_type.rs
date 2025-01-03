use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Room {
    pub id: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SignUp {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct RoomResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SignUpResponse {
    pub message: String,
    pub success: bool,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub id: i32,
}
