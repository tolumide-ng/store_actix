use actix::{Actor, Handler, Message, SyncContext};
// use actix_derive::{Message, MessageResponse};
use crate::db::prelude::DbActor;
use crate::diesel::prelude::*;
use crate::models::users::{NewUser, User, UserAuth, UserMessage, UserEmail};
use uuid::Uuid;
use chrono::prelude::*;

use crate::schema::user_role::dsl::{user_role};

#[derive(Message)]
#[rtype(result = "QueryResult<UserMessage>")]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hash: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<UserAuth>")]
pub struct LoginUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct VerifyEmail {
    pub email: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct ResetPassword<'a> {
    pub password: &'a str,
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<UserMessage>;
    
    fn handle(&mut self, msg: CreateUser, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::user_info::dsl::{user_info};
        
        // let conn = &self.0.get()?;`
        let new_user: NewUser = NewUser::new(msg);
        let conn = if let Ok(connection) = self.0.get() {
            connection
        } else {
            panic!("Error getting connection, please try later")
        };

        let create_user = diesel::insert_into(user_info).values(new_user).get_result::<User>(&conn);


        match create_user {
            Ok(user) => {
                let message = format!("Please check your email {} for information on how to proceed", user.email);

                Ok(UserMessage {message})
            }
            Err(e) => Err(e)
        }

        
    }
}


impl Handler<VerifyEmail> for DbActor {


    type Result = Result<User, diesel::result::Error>;

    fn handle(&mut self, msg: VerifyEmail, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::user_info::dsl::*;

        
        let conn = if let Ok(connection) = self.0.get() {
            connection
        } else {
            panic!("Error getting connection, please try later")
        };


        let email_exist = user_info.filter(email.eq(msg.email)).first::<User>(&conn);

        match email_exist {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }
}