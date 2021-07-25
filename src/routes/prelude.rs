use actix_web::{web, HttpResponse};
// use futures::Future;
use crate::routes::users;

fn routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::resource("/api/v1").route("/", web::get().to(users::register));
    cfg.service(web::resource("/test")
    .route(web::get().to(|| HttpResponse::Ok()))
    .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
);
    // app.service(web::scope("/api/v1").service(web::resource("user").route(web::post().to_async(users::register))));
}