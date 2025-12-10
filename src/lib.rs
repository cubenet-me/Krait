// Krait Translator Library
// Основной модуль для работы транслятора

pub mod modules;

// Пересклады основных типов для удобства
pub use modules::{Lexer, Parser, CodeGenerator, TopLevel, DataType, Expr, Statement, FunctionDef, RouteDef, TokenType};

// Версия
pub const VERSION: &str = "0.2.0";

/// Транслирует код Krait в Rust
pub fn translate(source: &str) -> Result<String, String> {
    // Лексический анализ
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    // Синтаксический анализ
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    // Генерация кода
    let mut codegen = CodeGenerator::new();
    let rust_code = codegen.generate(&ast[..]);
    
    Ok(rust_code)
}
