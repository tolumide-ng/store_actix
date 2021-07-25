use crate::schema::user_info;
use chrono;
use serde::{Deserialize, Serialize};
use uuid;
use crate::actors::users::CreateUser;



#[derive(Queryable, Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hash: String,
    pub user_type: Option<uuid::Uuid>,
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

#[derive(Serialize, Queryable)]
pub struct UserAuth {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub token: String,
}

#[derive(Serialize, Queryable, Deserialize, Debug)]
pub struct UserToken<'a> {
    pub token: &'a str,
}

#[derive(Serialize, Queryable, Deserialize, Debug)]
pub struct UserMessage {
    pub message: String,
}

 
#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}


impl NewUser {
    pub fn new(user: CreateUser) -> Self {
        NewUser {
            first_name: user.first_name,
            hash: user.hash,
            last_name: user.last_name,
            email: user.email
        }
    }
}
