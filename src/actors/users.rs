use actix::{Actor, Handler, Message, SyncContext};
// use actix_derive::{Message, MessageResponse};
use crate::db::prelude::DbActor;
use crate::diesel::prelude::*;
use crate::helpers::hash::generate_hash;
use crate::models::users::{NewUser, User, UserAuth, UserMessage, UserToken};
use crate::schema::user_info::dsl::{user_info};
use crate::schema::user_role::dsl::{user_role};
// use crate::schema::{user_info, user_role};
// use actix::prelude::*;
// use crate::schema::user_info::dsl::{user_info};

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
        
        // let conn = &self.0.get()?;`
        let new_user: NewUser = NewUser::new(msg);
        let conn = self.0.get().as_ref().expect("Unable to get connection");

        // let basic_user = user_role.filter()
        // let stored_user: User = users.filter(email.eq(msg.email)).fi

        // diesel::insert_into(user_info).values(new_user).execute(conn)?;
        diesel::insert_into(user_info).values(new_user).get_result::<User>(conn)?;
        // match diesel::insert_into(user_info).values(new_user).get_result::<User>(conn) {}



        // change the response type to one that implements a trait called response generator

        Ok(UserMessage {
            message: String::from("Please check your email for information on how to proceed"),
        })
    }
}
