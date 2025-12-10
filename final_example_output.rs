// Автоматически сгенерировано из Backenium
use actix_web::{web, App, HttpServer, middleware};

pub fn greet() -> i32 {
    println!("{}", "Welcome to Backenium!");
}

pub fn add(
    a: i32,
    b: i32
) -> i32 {
    return a + b;
}

pub fn max(
    x: i32,
    y: i32
) -> i32 {
    if x > y {
        return x;
    } else {
        return y;
    }
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

pub fn greet_user(
    name: String
) -> String {
    let greeting: i32 = "Hello, " + name;
    return greeting;
}

pub fn main() {
    println!("{}", "=== Backenium Demo ===");
    greet();
    let sum: i32 = add(10, 20);
    println!("{}", "Sum: ");
    let max_val: i32 = max(15, 25);
    println!("{}", "Max value: ");
    let fact: i32 = factorial(5);
    println!("{}", "5! = ");
    let msg: String = greet_user("Alice");
    println!("{}", "Message: ");
}

