// use models::{}
use actix_web::{get, post, web::{self, Data, Json}, Responder, HttpResponse};
use crate::{helpers::response_generator::ErrorResponse, models::users::UserMessage, validations::users::{UserData, UserLogin}};
use crate::db::prelude::AppState;
use crate::helpers::hash::{LocalHasher};
use crate::actors::users::{CreateUser};
use crate::actors::users::{VerifyEmail};




#[post("/register")]
async fn register(user: Json<UserData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    let UserData {first_name, last_name, email, password} = user;

    match db.send(VerifyEmail {email: email.clone()}).await {
        Ok(user) => {
            match user {
                Ok(_) => {
                    let msg = ErrorResponse::new(409, "Email already exists");
                    return HttpResponse::Conflict().json(msg)
                }
                _ => {}
            }
        }
        Err(_err) => {
            let msg = ErrorResponse::new(500, "Internal Server Error, Please try again later");

            return HttpResponse::InternalServerError().json(msg)
        }
    }


    let local_hash = LocalHasher {password: password.as_str()};

    let hash = local_hash.generate_hash();
    
    match db.send(CreateUser {first_name, last_name, email,  hash}).await {
        Ok(Ok(message)) => HttpResponse::Ok().json(message),
        _ => HttpResponse::InternalServerError().json("Internal Server Error")
    }
}



#[post("/login")]
async fn login(user: Json<UserLogin>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();


    let UserLogin { email, password } = user;

    // let hash_helper = LocalHasher {password};
    String::from("welcome to login page")
}