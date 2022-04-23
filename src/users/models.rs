use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
