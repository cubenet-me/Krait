// Модульная архитектура кодгенератора
pub mod lexer;
pub mod parser;
pub mod gen;
pub mod libs;

pub use gen::CodeGenerator;
pub use parser::Parser;
pub use lexer::Lexer;
