use crate::db::index;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub async fn register(data: web::Data<index::PooledConn>, req: HttpRequest) -> impl Responder {
    println!("THE SAID REQUEST >>>>>>>>>>><<<<<<<<<<< {:#?}", req);
    format!("Hello World")
    // let url = req.url_for("foo", &["1", "2", "3"]); // <- generate url for "foo" resource
    // HttpResponse::Ok().into()
    // format!("{:#?}", url)
    // let serialized = serde_json::to_string(&point).unwrap();
}

pub fn login() {}

pub fn basic_info() {}

pub fn forgot_password() {}

pub fn reset_password() {}
