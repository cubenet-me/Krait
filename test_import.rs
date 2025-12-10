// Автоматически сгенерировано из Backenium
use actix_web::{web, App, HttpServer, middleware};

pub fn get_status() -> String {
    return "ok";
}

pub fn main() {
    println!("{}", "API Server running");
}

