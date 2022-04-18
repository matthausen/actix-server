use serde::{Deserialize, Serialize};

// The Dynamo DB User model
#[derive(Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}