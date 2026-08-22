use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub name: String,
}
