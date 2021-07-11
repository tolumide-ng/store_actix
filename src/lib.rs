use actix_web::{get, http::header::{AUTHORIZATION, CONTENT_TYPE}, web::Data, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware};
use actix::prelude::{Addr, SyncArbiter};
use dotenv::dotenv;
use std::env;
use actix_cors::Cors;
use crate::db::index::{DbExecutor, new_pool};




#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
// extern crate jsonwebtoken as jwt;
extern crate dotenv;


pub mod middlewares;
pub mod db;
pub mod errors;
pub mod index;
pub mod routes;


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

    dotenv().ok();


    let database_url = env::var("DATABASE_URL").expect("Environment Varibale DATABASE_URL is required");
    println!("DATABSE URL IS!!!!!!!!! {}", database_url);
    let database_pool = new_pool(database_url).expect("Failed to create pool");
    // let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));
    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    

    // let bind_address = env::var("BIND_ADDRESS").expect("Environment Variable BIND_ADDRESS is required");
   


    HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone()
        };
        let cors = match client_url {
            Some(ref origin) => Cors::default().allowed_origin(origin).allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE]).max_age(3600),
            None => Cors::default().allowed_origin("*").send_wildcard().allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE]).max_age(3600)
        };

        App::new().app_data(Data::new(state)).wrap(middleware::Logger::default()).wrap(cors).configure(routes);



        App::new().service(index)
    })
        .bind(&bind_address)?
        .run()
        .await
}
