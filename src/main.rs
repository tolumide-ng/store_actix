use store;



use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer};
use std::env;


fn main() {
    if env::var("RUST_LOG").ok().is_none(){
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }
    


    store::start();
}



