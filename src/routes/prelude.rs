use actix_web::{web};

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::scope("/api/v1").service(web::resource("user").route(web::post().to_async())))
}