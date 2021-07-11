use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer};
use actix::prelude::{Addr, SyncArbiter};
use std::env;
use crate::db::prelude::{DbExecutor, new_pool};


#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
// extern crate jsonwebtoken as jwt;


pub mod middlewares;
pub mod db;
pub mod errors;
pub mod prelude;


#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    "Hello World"
}

pub struct AppState {
    pub db: Addr<DbExecutor>,
}



#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let client_url = env::var("FRONTEND_URL").ok();
    let database_url = env::var("DATABASE_URL").expect("Environment Varibale DATABASE_URL is required");
    let database_pool = new_pool(database_url).expect("Failed to create pool");
    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let bind_address = env::var("BIND_ADDRESS").expect("Environment Variable BIND_ADDRESS is required");
   


    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
