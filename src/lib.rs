use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer};
use actix::prelude::{Addr, SyncArbiter};
use std::env;

// #[macro_use]
// extern crate diesel;



pub mod middlewares;
pub mod db;

#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    "Hello World"
}

pub struct AppState {
    // pub db: Addr<DbExecutor>,
}



#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
//     let client_url = env::var("FRONTEND_URL").ok();
//     let database_url = env::var("DATABASE_URL").expect("PLEASE ADD DATABASE_URL");
//     let database_address = SyncArbiter::start(num_cpus::get(), move || DbExec)
   


    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
