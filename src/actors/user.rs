use actix::{Actor, Handler, Message, SyncContext};
// use actix_derive::{Message, MessageResponse};
use crate::db::index::DbActor;
use crate::diesel::prelude::*;
use crate::helpers::hash::generate_hash;
use crate::models::user::{NewUser, User, UserAuth, UserMessage, UserToken};
use crate::schema::{user_info, user_role};
use actix::prelude::*;
use uuid::Uuid;
// pub struct DbActor(pub: db::PgPool);

#[derive(Message)]
#[rtype(result = "QueryResult<UserMessage>")]
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
pub struct ResetPassword<'a> {
    pub password: &'a str,
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<UserMessage>;

    fn handle(&mut self, msg: CreateUser, ctx: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().as_ref().expect("Unable to get connection");

        let CreateUser {
            first_name,
            last_name,
            email,
            password,
        } = msg;

        // use bcyrpt to hash password here

        let new_user = NewUser {
            first_name,
            last_name,
            email,
            hash: generate_hash(password.as_str()),
        };

        // make call to db to cache user her

        // change the response type to one that implements a trait called response generator

        Ok(UserMessage {
            message: String::from("Please check your email for information on how to proceed"),
        })
    }
}
