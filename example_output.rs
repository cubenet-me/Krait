// Автоматически сгенерировано из Backenium
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, middleware};
use serde_json::{json, to_string};
use serde::{Deserialize, Serialize};

fn add(
    a: i32,
    b: i32
) -> i32 {
    return (a + b);
}

fn factorial(
    n: i32
) -> i32 {
    if (n <= 1) {
        return 1;
    } else {
        return (n * factorial((n - 1)));
    }
}

fn is_even(
    x: i32
) -> bool {
    return ((x % 2) == 0);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Backenium server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/api/greet", web::get().to(get_api_greet))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
