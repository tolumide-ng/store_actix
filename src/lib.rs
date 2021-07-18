use crate::db::index::get_pool;
use actix::prelude::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware, web,
    web::Data,
    App, HttpRequest, HttpServer, Responder,
};
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel_migrations;

// extern crate jsonwebtoken as jwt;
extern crate dotenv;

pub mod actors;
pub mod db;
pub mod errors;
pub mod helpers;
pub mod index;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod schema;

// use actix_web::{web, App, HttpRequest, HttpServer, Responder};

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let client_url = env::var("FRONTEND_URL").ok();

    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Environment Varibale DATABASE_URL is required");
    // let database_pool = new_pool(database_url).expect("Failed to create pool");

    let db_pool = get_pool(String::from(database_url)).unwrap();

    // let database_address = SyncArbiter::start(num_cpus::get(), move || DbActor(database_pool.clone()));

    HttpServer::new(move || {
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

        App::new().data(db_pool.clone()).service(
            web::scope("/api/v1").service(web::resource("/signup").to(routes::users::register)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
