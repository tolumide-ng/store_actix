use crate::actix::{Actor, Hanlder, Message, SyncContext};
use crate::db;
use crate::diesel::prelude::*;
use crate::models::{NewUser, User, UserAuth, UserToken};
use crate::schema::{user_info, user_role};
use uuid::Uuid;

// pub struct DbActor(pub: db::PgPool);

#[derive(Message)]
#[rtype(result = "QueryResult<UserAuth>")]
pub struct CreateUser {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Message)]
#[rtype(result = "QueryResult<UserAuth>")]
pub struct LoginUser {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct ForgotPassword {
    pub email: &'a str,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct ResetPassword {
    pub password: &'a str,
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<UserAuth>;

    fn handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().as_ref().expect("Unable to get connection");

        let {first_name, last_name, email, password} = msg;

        // use bcyrpt to hash password here

        let new_user = NewUser {
            first_name, last_name, email, password: ""
        }
    }
}
