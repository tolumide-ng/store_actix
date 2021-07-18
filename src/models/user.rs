use crate::schema::user_info;
use chrono;
use serde::{Deserialize, Serialize};
use uuid;

pub struct AppState {}

// impl

#[derive(Queryable, Deserialize, Debug, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub user_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[table_name = "user_info"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hash: String,
}

#[derive(Serialize)]
pub struct UserAuth {
    first_name: String,
    last_name: String,
    email: String,
    token: String,
}

#[derive(Serialize, Queryable, Deserialize, Debug)]
pub struct UserToken<'a> {
    token: &'a str,
}

#[derive(Serialize, Queryable, Deserialize, Debug)]
pub struct UserMessage {
    message: String,
}
