// use models::{}
use actix_web::{get, post, web::{self, Data, Json}, Responder, HttpResponse};
use crate::{helpers::response_generator::ErrorResponse, models::users::{UserAuth, UserMessage}, validations::users::{UserData, UserLogin}};
use crate::db::prelude::AppState;
use crate::helpers::hash::{LocalHasher};
use crate::actors::users::{CreateUser};
use crate::actors::users::{VerifyEmail};
use crate::helpers::token;
use crate::models::users::{User};




#[post("/register")]
async fn register(user: Json<UserData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    let UserData {first_name, last_name, email, password} = user;

    match db.send(VerifyEmail {email: email.clone()}).await {
        Ok(user) => {
            match user {
                Ok(_) => {
                    let msg = ErrorResponse::new(409, "Email already exists".to_owned());
                    return HttpResponse::Conflict().json(msg)
                }
                _ => {}
            }
        }
        Err(_err) => {
            return HttpResponse::InternalServerError().json(ErrorResponse::server_error())
        }
    }


    let hash = LocalHasher::generate_hash(&password);
    
    match db.send(CreateUser {first_name, last_name, email,  hash}).await {
        Ok(Ok(message)) => HttpResponse::Ok().json(message),
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::server_error())
    }
}



#[post("/login")]
async fn login(user: Json<UserLogin>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    let UserLogin {email, password} = user;

    //  PLEASE REFACTOR THIS MATCH HELL
    match db.send(VerifyEmail {email: email}).await {
        Ok(user) => {
            match user {
                Ok(the_user) => {

                    let User {first_name, last_name, email, hash, .. } = the_user;


                    if LocalHasher::verify_hash(hash, password) {
                        let user_token = token::UserToken::generate_token(email.to_owned(), 40, "store_recs".to_string());

                        match user_token {
                            Ok(token) => {
                                return HttpResponse::Ok().json(UserAuth::new(first_name, last_name, email, token))
                            }
                            Err(_e) => {}
                        }

                    } else {
                        return HttpResponse::Unauthorized().json(ErrorResponse::auth_error(Some(": Email or Password is incorrect")))
                    }
                }
                _ => {
                    return HttpResponse::Unauthorized().json(ErrorResponse::auth_error(Some(": Email or Password is incorrect")))
                }
            }
        }
        _ => {
            return HttpResponse::InternalServerError().json(ErrorResponse::server_error())
        }
    }

    

    // let UserLogin { email, password } = user;

    // let hash_helper = LocalHasher {password};
    // String::from("welcome to login page")


    let msg = UserMessage {message: "This is a placeholder text".to_string()};
    HttpResponse::Ok().json(msg)
}