# 🏗️ Гайд по модульной архитектуре Backenium

## Обзор

Backenium v0.1.3+ имеет чистую модульную архитектуру, которая позволяет:

- 📦 Использовать как библиотеку
- 🔧 Легко расширять функциональность
- 🧪 Тестировать каждый модуль отдельно
- 🚀 Быстро добавлять новые функции

## Директория `src/modules`

Все компоненты находятся в одной папке для удобства управления:

```
src/modules/
├── lexer/       - Лексический анализатор (40+ токенов)
├── parser/      - Парсер (построение AST)
├── codegen/     - Генератор Rust кода
├── cli/         - Интерфейс командной строки
└── api/         - Web API поддержка
```

### Модуль `lexer`

**Что делает:** Преобразует текст в токены

**Публичный API:**
```rust
pub struct Lexer { ... }
pub enum TokenType { ... }
pub struct Token { ... }

impl Lexer {
    pub fn new(input: &str) -> Self
    pub fn tokenize(&mut self) -> Vec<Token>
}
```

**Пример:**
```rust
use backenium_translator::Lexer;

let mut lexer = Lexer::new("func test() end");
let tokens = lexer.tokenize();
println!("{:?}", tokens);
```

### Модуль `parser`

**Что делает:** Строит AST из токенов

**Публичный API:**
```rust
pub struct Parser { ... }
pub enum DataType { Int, Float, Double, Txt, Bool, Auto }
pub enum Expr { Literal, Identifier, BinaryOp, FunctionCall }
pub enum Statement { VarDecl, Return, If, Expression }
pub struct FunctionDef { ... }
pub enum TopLevel { Import, Function, Route, Statement }

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self
    pub fn parse(&mut self) -> Result<Vec<TopLevel>, String>
}
```

**Пример:**
```rust
use backenium_translator::{Lexer, Parser};

let mut lexer = Lexer::new("func add(int x) x end");
let tokens = lexer.tokenize();

let mut parser = Parser::new(tokens);
let ast = parser.parse()?;
```

### Модуль `codegen`

**Что делает:** Генерирует Rust код из AST

**Публичный API:**
```rust
pub struct CodeGenerator { ... }

impl CodeGenerator {
    pub fn new() -> Self
    pub fn generate(&mut self, items: &[TopLevel]) -> String
}
```

**Пример:**
```rust
use backenium_translator::{Lexer, Parser};
use backenium_translator::modules::codegen::CodeGenerator;

let mut lexer = Lexer::new(code);
let tokens = lexer.tokenize();
let mut parser = Parser::new(tokens);
let ast = parser.parse()?;

let mut gen = CodeGenerator::new();
let rust_code = gen.generate(&ast);
println!("{}", rust_code);
```

### Модуль `cli`

**Что делает:** Функции для работы с файлами

**Публичный API:**
```rust
pub enum CliResult { Success(String), Error(String) }

pub fn translate_file(input_path: &str, output_path: &str) -> CliResult
pub fn translate_project(input_dir: &str, output_dir: &str) -> CliResult
pub fn show_help()
pub fn show_version()
```

**Пример:**
```rust
use backenium_translator::modules::cli;

match cli::translate_file("input.bkc", "output.rs") {
    cli::CliResult::Success(msg) => println!("✓ {}", msg),
    cli::CliResult::Error(err) => eprintln!("✗ {}", err),
}
```

### Модуль `api`

**Что делает:** Поддержка Web API и конфигурация

**Публичный API:**
```rust
pub struct ApiInfo { ... }
pub struct ApiConfig { ... }

impl ApiInfo {
    pub fn from_routes(routes: &[&RouteDef]) -> Self
    pub fn describe(&self) -> String
    pub fn needs_actix(&self) -> bool
}

pub fn validate_routes(routes: &[&RouteDef]) -> Result<(), String>
pub fn print_api_stats(routes: &[&RouteDef])
```

**Пример:**
```rust
use backenium_translator::modules::api;

let info = api::ApiInfo::from_routes(&routes);
println!("Type: {}", info.describe());

if info.has_routes {
    api::print_api_stats(&routes);
}
```

## Главный API (lib.rs)

Библиотека предоставляет удобный публичный API:

```rust
use backenium_translator::translate;

fn main() -> Result<(), String> {
    let source = "func hello() print(\"world\") end";
    let rust_code = translate(source)?;
    println!("{}", rust_code);
    Ok(())
}
```

## Главное приложение (main.rs)

Минимальное приложение, использующее только CLI модуль:

```rust
use backenium_translator::modules::cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    match args.len() {
        2 => {
            match args[1].as_str() {
                "--help" => cli::show_help(),
                "--version" => cli::show_version(),
                _ => eprintln!("Unknown command"),
            }
        }
        3 => {
            cli::translate_file(&args[1], &args[2]).print();
        }
        _ => cli::show_help(),
    }
}
```

## Как добавить новый модуль

Допустим, нужно добавить модуль для оптимизации AST.

### 1. Создать структуру

```bash
mkdir src/modules/optimizer
```

### 2. Написать код

```rust
// src/modules/optimizer/mod.rs

use super::parser::TopLevel;

pub struct Optimizer;

impl Optimizer {
    pub fn new() -> Self {
        Optimizer
    }
    
    pub fn optimize(&self, ast: &[TopLevel]) -> Vec<TopLevel> {
        // Логика оптимизации
        ast.to_vec()
    }
}
```

### 3. Экспортировать модуль

```rust
// src/modules/mod.rs - добавить строку
pub mod optimizer;
```

### 4. Использовать в другом модуле

```rust
// src/modules/codegen/mod.rs - добавить импорт
use super::optimizer::Optimizer;

// Использовать
let optimizer = Optimizer::new();
let optimized = optimizer.optimize(&ast);
```

### 5. Экспортировать из библиотеки (опционально)

```rust
// src/lib.rs - добавить если нужен публичный API
pub use modules::optimizer::Optimizer;
```

## Примеры расширения

### Пример 1: Добавить модуль форматирования

```rust
// src/modules/formatter/mod.rs

pub struct Formatter;

impl Formatter {
    pub fn format_rust_code(code: &str) -> String {
        // Форматирование через rustfmt API
        code.to_string()
    }
}
```

### Пример 2: Добавить модуль анализа

```rust
// src/modules/analyzer/mod.rs

use super::parser::TopLevel;

pub struct Analyzer;

impl Analyzer {
    pub fn analyze(ast: &[TopLevel]) -> AnalysisResult {
        // Анализ кода
        AnalysisResult { /* ... */ }
    }
}

pub struct AnalysisResult {
    pub functions: usize,
    pub routes: usize,
}
```

## Структура зависимостей

```
main.rs (точка входа)
  ↓
lib.rs (публичный API)
  ├── modules::cli (CLI функции)
  ├── modules::api (Web поддержка)
  ├── modules::codegen (генерация)
  │   ↓
  ├── modules::parser (парсинг)
  │   ↓
  └── modules::lexer (токены)
```

## Правила модульного дизайна

✅ **Каждый модуль отвечает за одно**
- lexer → токены
- parser → AST
- codegen → Rust код
- cli → файлы
- api → конфигурация

✅ **Модули могут зависеть от предыдущих**
- codegen зависит от parser
- parser зависит от lexer
- cli может использовать любой

✅ **Минимальная зависимость**
- main.rs зависит только от cli
- lib.rs экспортирует публичный API

✅ **Каждый модуль имеет mod.rs**
- Публичный API в mod.rs
- Внутренняя логика может быть в других файлах

## Компиляция

```bash
# Собрать все (lib + bin)
cargo build

# Собрать только библиотеку
cargo build --lib

# Собрать только бинарник
cargo build --bin bkm_translator

# Release версия
cargo build --release

# Тесты
cargo test

# Документация
cargo doc --open
```

## Использование как библиотеки

```toml
# В Cargo.toml другого проекта
[dependencies]
backenium_translator = { path = "../backenium_translator" }
```

```rust
// В коде проекта
use backenium_translator::translate;

fn main() -> Result<(), String> {
    let rust = translate("func test() end")?;
    println!("{}", rust);
    Ok(())
}
```

## Версионирование

Версия указана в `src/lib.rs`:

```rust
pub const VERSION: &str = "0.1.3";
```

Она автоматически используется в CLI и других модулях.

## Советы по разработке

1. **Тестируйте модули отдельно**
   ```bash
   cargo test --lib modules::lexer
   ```

2. **Используйте документированные примеры**
   ```rust
   /// Пример:
   /// ```
   /// use backenium_translator::Lexer;
   /// let mut lexer = Lexer::new("func test() end");
   /// ```
   pub fn new(input: &str) -> Self { ... }
   ```

3. **Следуйте паттернам**
   - Публичный struct/enum → pub
   - Приватные детали → нет pub
   - Основной интерфейс → pub fn

4. **Не ломайте публичный API**
   - Старайтесь добавлять, а не удалять
   - Отмечайте deprecations
   - Пишите документацию для публичного API

---

**Backenium v0.1.3** - модульная, расширяемая архитектура! 🚀
