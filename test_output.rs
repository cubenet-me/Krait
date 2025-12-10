// Автоматически сгенерировано из Backenium
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, middleware};
use serde_json::{json, to_string};
use serde::{Deserialize, Serialize};

fn add(
    a: i32,
    b: i32
) -> i32 {
    return a;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Backenium server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
