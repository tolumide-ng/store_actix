use chrono;
use uuid;
use serde::{Deserialize, Serialize};
use crate::schema::*;

pub struct AppState {}

#[derive(Queryable, Deserialize, Debug, Serialize, Identifiable, Deserialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub user_type String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}


#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[table_name="user"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email String,
    pub password: String,
}


#[derive(Serialize)]
pub struct UserAuth<'a> {
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
    token: &'a str,
}


#[derive(Serialize, Queryable, Identifiable, Deserialize, Debug)]
pub struct UserToken<'a> {
    token: &'a str,
}

 
#[derive(Serialize, Queryable, Identifiable, Deserialize, Debug)]
pub struct ResetPassword<'a> {
    message: &'a str
}

