use crate::db::index::{new_pool, DbExecutor};
use actix::prelude::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_web::{
    get,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware, web,
    web::Data,
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
// extern crate jsonwebtoken as jwt;
extern crate dotenv;

pub mod db;
pub mod errors;
pub mod index;
pub mod middlewares;
pub mod routes;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

// use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let client_url = env::var("FRONTEND_URL").ok();

    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Environment Varibale DATABASE_URL is required");
    let database_pool = new_pool(database_url).expect("Failed to create pool");
    // let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));
    // let database_address =
    //     SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    HttpServer::new(move || {
        // let state = AppState {
        //     db: database_address.clone(),
        // };

        let cors = match client_url {
            Some(ref origin) => Cors::default()
                .allowed_origin(origin)
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
            None => Cors::default()
                .allowed_origin("*")
                .send_wildcard()
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
        };

        // App::new::data(database_pool.clone()).route("/", web::post().to(routes::users::register))
        App::new().data(database_pool.clone()).service(
            web::scope("/api/v1")
                // .route("/{name}", web::get().to(routes::users::register))
                .service(web::resource("/signup").to(routes::users::register)),
        )
        // .service(web::resource("/api"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
