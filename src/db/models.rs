use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub struct NewUser {
    pub email: String,
    pub name: String,
}
