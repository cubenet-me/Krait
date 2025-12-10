// Автоматически сгенерировано из Backenium
use actix_web::{web, App, HttpServer, middleware};

pub fn add(
    a: i32,
    b: i32
) -> i32 {
    return a + b;
}

pub fn factorial(
    n: i32
) -> i32 {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

pub fn is_even(
    x: i32
) -> bool {
    return x % 2 == 0;
}

pub fn max(
    a: i32,
    b: i32
) -> i32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

pub fn greet(
    name: String
) -> String {
    let greeting: String = "Hello, " + name;
    return greeting;
}

pub fn calculate(
    x: i32,
    y: i32
) -> i32 {
    let sum: i32 = x + y;
    let product: i32 = x * y;
    let result: i32 = sum + product;
    return result;
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
