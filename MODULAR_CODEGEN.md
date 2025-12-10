# Модульная архитектура Krait кодгенератора

## Обзор

Кодгенератор Krait разделён на несколько независимых модулей для удобства расширения и поддержки новых библиотек.

```
src/modules/codegen/
├── mod.rs          # Главный модуль (переэкспорт)
├── lexer.rs        # Лексический анализатор (токенизация)
├── parser.rs       # Синтаксический анализатор (AST)
├── gen.rs          # Генератор Rust кода
├── libs.rs         # Реестр библиотек (cargo crates)
└── backends/       # Будущие backends для других языков
```

## Модули

### 1. lexer.rs - Лексический анализатор

Преобразует исходный текст Krait в поток токенов.

**Основные типы:**
- `TokenType` - перечисление всех типов токенов
- `Token` - токен с информацией о позиции
- `Lexer` - сам лексер

**Пример использования:**
```rust
let mut lexer = Lexer::new("public func get_users() -> txt");
let tokens = lexer.tokenize();
```

### 2. parser.rs - Синтаксический анализатор

Преобразует токены в AST (Abstract Syntax Tree).

**Основные типы:**
- `DataType` - типы данных (int, txt, bool, и т.д.)
- `Expr` - выражения
- `Statement` - инструкции
- `FunctionDef` - определение функции
- `RouteDef` - определение HTTP-маршрута
- `TopLevel` - элементы верхнего уровня
- `Parser` - сам парсер

**Пример использования:**
```rust
let mut parser = Parser::new(tokens);
let ast = parser.parse()?;
```

### 3. gen.rs - Генератор кода

Преобразует AST в Rust код.

**Основные типы:**
- `CodeGenerator` - главный генератор кода

**Основные методы:**
- `new()` - создать новый генератор
- `generate(&mut self, items: &[TopLevel]) -> String` - сгенерировать код
- `generate_imports()` - сгенерировать импорты на основе требуемых библиотек
- `generate_function()` - сгенерировать функцию
- `generate_route_handler()` - сгенерировать HTTP обработчик
- `generate_statement()` - сгенерировать инструкцию
- `generate_expr()` - сгенерировать выражение

**Пример использования:**
```rust
let mut codegen = CodeGenerator::new();
let rust_code = codegen.generate(&ast);
```

### 4. libs.rs - Реестр библиотек

Управляет доступными Rust библиотеками и их импортами.

**Основные типы:**
- `Library` - информация о библиотеке
  - `name` - человеческое название
  - `crate_name` - имя крейта в Cargo
  - `imports` - список импортов
  - `features` - функциональность
  
- `LibraryRegistry` - реестр всех доступных библиотек

**Встроенные библиотеки:**
- `rest` - Actix-web для REST API
- `json` - serde_json для JSON сериализации
- `sqlx` - асинхронная база данных SQL
- `mongodb` - MongoDB драйвер
- `tokio` - async runtime
- `log` - логирование
- `serde` - сериализация

**Пример добавления новой библиотеки:**
```rust
let mut registry = LibraryRegistry::new();

let new_lib = Library {
    name: "PostgreSQL".to_string(),
    crate_name: "postgres".to_string(),
    imports: vec![
        "use postgres::Client;".to_string(),
    ],
    features: vec!["database".to_string()],
};

registry.add_library("postgres".to_string(), new_lib);
```

## Полный pipeline

```
Исходный код Krait
        ↓
   Lexer::tokenize()
        ↓
    Поток токенов
        ↓
   Parser::parse()
        ↓
      AST
        ↓
   CodeGenerator::generate()
   (использует LibraryRegistry)
        ↓
   Rust код
        ↓
   Rust compiler
        ↓
   Исполняемый файл
```

## Добавление поддержки новой библиотеки

### Шаг 1: Определить библиотеку в libs.rs

```rust
// В LibraryRegistry::new()
libs.insert(
    "redis".to_string(),
    Library {
        name: "Redis".to_string(),
        crate_name: "redis".to_string(),
        imports: vec![
            "use redis::Client;".to_string(),
        ],
        features: vec!["caching".to_string()],
    },
);
```

### Шаг 2: Использовать в Krait коде

```krait
import redis from redis

public func get_cached(key: txt) -> txt
    // код с redis
end
```

### Шаг 3: Генератор автоматически добавит импорты

```rust
use redis::Client;

pub fn get_cached(key: String) -> String {
    // твой код
}
```

## Расширение функциональности

### Добавить новый тип данных

1. Добавить в `lexer.rs`:
```rust
TokenType::Custom,
```

2. Добавить в `parser.rs`:
```rust
pub enum DataType {
    // ...
    Custom,
}

impl DataType {
    pub fn to_rust(&self) -> &str {
        match self {
            // ...
            DataType::Custom => "MyCustomType",
        }
    }
}
```

3. Обновить `parse_type()` в Parser

### Добавить новый statement

1. Добавить в `parser.rs`:
```rust
pub enum Statement {
    // ...
    MyNewStatement { /* поля */ },
}
```

2. Добавить обработку в `Parser::parse_statement()`

3. Добавить генерацию в `CodeGenerator::generate_statement()`

## Тестирование

```bash
# Проверить компиляцию
cargo check

# Собрать проект
cargo build

# Запустить тесты
cargo test

# Запустить с примером
cargo run -- build examples/hello.kr
```

## Производительность

- **Лексер**: O(n) где n = длина исходного кода
- **Парсер**: O(n) в среднем случае
- **Кодген**: O(AST) где AST = размер дерева

Для файлов до 10MB транслитерация обычно < 100ms.
