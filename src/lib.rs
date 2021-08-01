extern crate actix;

#[macro_use]
extern crate diesel;

// #[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel_migrations;

// extern crate jsonwebtoken as jwt;
extern crate dotenv;


use crate::db::prelude::{AppState, get_pool};
use actix::SyncArbiter;
// use actix::prelude::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware, App,
    web, HttpServer,
};
use db::prelude::{DbActor, run_migrations};
use dotenv::dotenv;
use std::env;


pub mod actors;
pub mod db;
pub mod errors;
pub mod helpers;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod schema;
pub mod controllers;
pub mod validations;


#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let client_url = env::var("FRONTEND_URL").ok();

    dotenv().ok();


    // CONFIRM IF ALL REQUIRED ENVIRONMENT VARIABLES ARE PROVIDED
    let expected_variables = ["DATABASE_URL", "STORE_NAME", "SMTP_USERNAME", "SMTP_PASSWORD", "JWT_SECRET"];

    expected_variables.iter().for_each(|variable| {
        let error = format!("Environment Variable {} is required", variable);
        env::var(variable).expect(error.as_str());
    });



    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");



    run_migrations(&db_url);
    // let pool = get_pool(db_url).expect("msg");

    let db_addr = match get_pool(db_url) {
        Ok(pool) => SyncArbiter::start(num_cpus::get(), move || DbActor(pool.clone())),
        Err(_) => panic!("Error creating db connection"),
    };

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

        // App::new().service(
        //     web::scope("/api/v1").service(web::resource("/signup").to(routes::users::register)),
        // )

        App::new()
            .service(controllers::users::register)
            .service(controllers::users::login)
            .data(AppState {
                db: db_addr.clone()
            })
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
