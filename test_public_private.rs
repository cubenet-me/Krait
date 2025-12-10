// Автоматически сгенерировано из Backenium
pub fn add(
    a: i32,
    b: i32
) -> i32 {
    return a + b;
}

fn helper() -> i32 {
    println!("{}", "helper");
}

pub fn main() {
    let result: i32 = add(5, 3);
    println!("{}", "Result");
}

