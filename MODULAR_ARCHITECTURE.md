# 📦 Модульная архитектура Backenium v0.1.3

## Структура проекта

```
backenium_translator/
├── Cargo.toml           # Конфигурация (lib + bin)
├── src/
│   ├── lib.rs          # Публичная библиотека API
│   ├── main.rs         # CLI интерфейс (использует модули)
│   └── modules/        # Все модули в одной папке
│       ├── mod.rs      # Главный файл модулей
│       ├── lexer/      # Лексический анализ
│       │   └── mod.rs
│       ├── parser/     # Синтаксический анализ
│       │   └── mod.rs
│       ├── codegen/    # Генерация Rust кода
│       │   └── mod.rs
│       ├── cli/        # Интерфейс командной строки
│       │   └── mod.rs
│       └── api/        # Поддержка Web API
│           └── mod.rs
└── target/             # Скомпилированные бинарии
```

## Модули

### `modules::lexer` - Лексический анализ

Преобразует исходный текст Backenium в токены.

```rust
pub struct Lexer { ... }
pub enum TokenType { ... }
pub struct Token { ... }

impl Lexer {
    pub fn new(input: &str) -> Self
    pub fn tokenize(&mut self) -> Vec<Token>
}
```

**Использование:**
```rust
use backenium_translator::Lexer;

let mut lexer = Lexer::new(source);
let tokens = lexer.tokenize();
```

### `modules::parser` - Синтаксический анализ

Строит AST (Abstract Syntax Tree) из токенов.

```rust
pub struct Parser { ... }
pub enum DataType { Int, Float, Double, Txt, Bool, Auto }
pub enum Expr { Literal, Identifier, BinaryOp, FunctionCall }
pub enum Statement { VarDecl, Return, If, Expression }
pub struct FunctionDef { ... }
pub enum TopLevel { Import, Function, Route, Statement }
```

**Использование:**
```rust
use backenium_translator::Parser;

let mut parser = Parser::new(tokens);
let ast = parser.parse()?;
```

### `modules::codegen` - Генерация Rust кода

Генерирует Rust код из AST.

```rust
pub struct CodeGenerator { ... }

impl CodeGenerator {
    pub fn new() -> Self
    pub fn generate(&mut self, items: &[TopLevel]) -> String
}
```

**Использование:**
```rust
use backenium_translator::CodeGenerator;

let mut codegen = CodeGenerator::new();
let rust_code = codegen.generate(&ast);
```

### `modules::cli` - Интерфейс командной строки

Функции для работы с файлами и проектами.

```rust
pub enum CliResult { Success(String), Error(String) }

pub fn translate_file(input_path: &str, output_path: &str) -> CliResult
pub fn translate_project(input_dir: &str, output_dir: &str) -> CliResult
pub fn show_help()
pub fn show_version()
```

**Использование:**
```rust
use backenium_translator::modules::cli;

cli::translate_file("input.bkc", "output.rs").print();
```

### `modules::api` - Web API поддержка

Функции для работы с Web маршрутами и конфигурацией.

```rust
pub struct ApiInfo { ... }
pub struct ApiConfig { ... }

pub fn validate_routes(routes: &[&RouteDef]) -> Result<(), String>
pub fn print_api_stats(routes: &[&RouteDef])
```

**Использование:**
```rust
use backenium_translator::modules::api;

let info = api::ApiInfo::from_routes(&routes);
println!("Type: {}", info.describe());
```

## Публичный API (lib.rs)

Главная библиотека экспортирует основные типы и функцию трансляции:

```rust
// Основная функция трансляции
pub fn translate(source: &str) -> Result<String, String>

// Переэкспортированные типы
pub use modules::lexer::{Lexer, Token, TokenType};
pub use modules::parser::{Parser, DataType, Expr, Statement, FunctionDef, RouteDef, TopLevel};
pub use modules::codegen::CodeGenerator;

pub const VERSION: &str = "0.1.3";
```

## Как добавить новый модуль

### 1. Создать папку

```bash
mkdir src/modules/mymodule
```

### 2. Создать mod.rs

```rust
// src/modules/mymodule/mod.rs

pub struct MyType { ... }
pub fn my_function() { ... }
```

### 3. Добавить в modules/mod.rs

```rust
// src/modules/mod.rs
pub mod mymodule;
```

### 4. Использовать

```rust
use backenium_translator::modules::mymodule;
```

## Примеры использования

### Как библиотека

```rust
use backenium_translator::translate;

fn main() -> Result<(), String> {
    let source = "func hello() print(\"world\") end";
    let rust_code = translate(source)?;
    println!("{}", rust_code);
    Ok(())
}
```

### Через modules

```rust
use backenium_translator::modules::{lexer, parser, codegen};

fn main() -> Result<(), String> {
    let source = "func hello() print(\"world\") end";
    
    // Лексический анализ
    let mut lex = lexer::Lexer::new(source);
    let tokens = lex.tokenize();
    
    // Синтаксический анализ
    let mut p = parser::Parser::new(tokens);
    let ast = p.parse()?;
    
    // Генерация
    let mut gen = codegen::CodeGenerator::new();
    let rust_code = gen.generate(&ast);
    
    Ok(())
}
```

## Компиляция

### Библиотека

```bash
cargo build --lib
```

### Бинарник

```bash
cargo build --bin bkm_translator
```

### Оба

```bash
cargo build
```

### Release версия

```bash
cargo build --release
```

## Тестирование

```bash
cargo test
```

## Структура зависимостей

```
main.rs (CLI)
    ↓
lib.rs (API)
    ├── modules::cli
    ├── modules::api
    ├── modules::codegen
    │   ↓
    ├── modules::parser
    │   ↓
    └── modules::lexer
```

## Преимущества модульной архитектуры

✅ **Чистота кода** - каждый модуль отвечает за одно
✅ **Переиспользование** - можно использовать как библиотеку
✅ **Расширяемость** - легко добавлять новые модули
✅ **Тестируемость** - каждый модуль можно тестировать отдельно
✅ **Логическое разделение** - функциональность четко разделена

## Миграция со старых версий

Транслятор полностью совместим:

```bash
# Старая версия
./bkm_translator input.bkc output.rs

# Новая версия - работает также!
./target/debug/bkm_translator input.bkc output.rs
```

---

**Backenium v0.1.3** - модульная архитектура готова! 🎉
