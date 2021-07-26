// use models::{}
use actix_web::{get, post, web::{self, Data, Json}, Responder, HttpResponse};
use crate::validations::users::{UserData};
use crate::db::prelude::AppState;
use crate::helpers::hash::{generate_hash};
use crate::actors::users::{CreateUser};


#[post("/register")]
async fn register(user: Json<UserData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    let UserData {first_name, last_name, email, password} = user;
    
    let hash = generate_hash(password.as_str());
    
    match db.send(CreateUser {first_name, last_name, email,  hash}).await {
        Ok(Ok(message)) => HttpResponse::Ok().json(message),
        _ => HttpResponse::InternalServerError().json("Internal Server Error")
    }
}