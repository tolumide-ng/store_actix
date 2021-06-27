use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer};
use std::env;


#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    "Hello World"
}

pub async fn start() -> std::io::Result<()> {
    let client_url = env::var("FRONTEND_URL").ok();

    HttpServer::new(|| {
        // let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header().max_age(3600);
        let cors = match client_url {
            // Some(ref client) => Cors::new().allowed_origin()
        }

        App::new().wrap(cors).service(index)

    }).bind(("127.0.0.1", 8000))?.run().await
}