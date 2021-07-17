use chrono;
use uuid;
use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Queryable, Deserialize, Debug, Serialize, Identifiable)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hash: String,
    pub user_type String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}


#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email String,
    pub hash: String,
}


#[derive(Serialize)]
pub struct UserAuth<'a> {
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
    token: &'a str,
}
