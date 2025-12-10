// Автоматически сгенерировано из Backenium
use actix_web::{get, post, put, delete, App, HttpServer, HttpResponse, Responder};

pub fn get_users() -> String {
    return "users list".to_string();
}

pub fn get_user_by_id(
    id: i32
) -> String {
    return "user".to_string();
}

#[get("/api/users")]
async fn get_api_users() -> impl Responder {
    HttpResponse::Ok().body("response")
}

#[post("/api/users")]
async fn post_api_users() -> impl Responder {
    HttpResponse::Ok().body("response")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Backenium server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(get_api_users)
            .service(post_api_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
