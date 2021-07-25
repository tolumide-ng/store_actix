// use models::{}
use actix_web::{get, post, web::{self, Data, Json}, Responder};
use crate::models::users::{UserData};
use crate::db::prelude::AppState;


#[post("/register")]
async fn create_user(user: Json<UserData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();
    format!("HEllo world")
}