use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct IndexResponse {
    first_name: String,
    last_name: String,
    pronouns: String,
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    let resp = IndexResponse {
        first_name: String::from("lukas"),
        last_name: String::from("zanner"),
        pronouns: String::from("he/him"),
    };

    web::Json(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
