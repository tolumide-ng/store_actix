use actix::{Actor, Handler, Message, SyncContext};
// use actix_derive::{Message, MessageResponse};
use crate::db;
use crate::diesel::prelude::*;
use crate::models::user::{NewUser, User, UserAuth, UserToken};
use crate::schema::{user_info, user_role};
use uuid::Uuid;
use actix::prelude::*;
// pub struct DbActor(pub: db::PgPool);

#[derive(Message)]
#[rtype(result = "QueryResult<UserAuth>")]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<UserAuth>")]
pub struct LoginUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct ForgotPassword<'a> {
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
