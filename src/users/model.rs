use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}