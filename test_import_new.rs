// Автоматически сгенерировано из Backenium
use actix_web::{get, post, put, delete, App, HttpServer, HttpResponse, Responder};

pub fn get_status() -> String {
    return "ok".to_string();
}

#[get("/api/status")]
async fn get_api_status() -> impl Responder {
    HttpResponse::Ok().body("response")
}

#[post("/api/calculate")]
async fn post_api_calculate() -> impl Responder {
    HttpResponse::Ok().body("response")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Backenium server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(get_api_status)
            .service(post_api_calculate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
