# Архитектура проекта Krait

## Структура директорий

```
krait/
├── src/
│   ├── main.rs                    # Точка входа, обработка команд
│   ├── lib.rs                     # Экспорт модулей
│   └── modules/
│       ├── mod.rs                 # Корневой модуль
│       ├── cli/
│       │   └── mod.rs            # CLI команды: init, build, run
│       ├── api/
│       │   └── mod.rs            # REST API функциональность
│       └── codegen/
│           ├── mod.rs            # Интеграция всех компонентов
│           ├── lexer.rs          # Лексический анализ (.kr → токены)
│           ├── parser.rs         # Синтаксический анализ (токены → AST)
│           ├── gen.rs            # Генерация Rust кода (AST → .rs)
│           ├── libs.rs           # Система управления библиотеками
│           └── backends/
│               ├── mod.rs        # Интеграция бэкендов
│               └── rust.rs       # Rust код генератор
├── Cargo.toml                     # Зависимости проекта
├── LICENSE                        # GPL 3.0 лицензия
├── .gitignore                     # Git исключения
└── LIBRARY_GUIDE.md              # Руководство по библиотекам
```

## Компоненты

### 1. CLI (`src/modules/cli/mod.rs`)

Обрабатывает команды пользователя:

```
krait init <app-name>    # Создает новый проект
krait build              # Компилирует .kr файлы в Rust
krait run                # Запускает приложение
krait list               # Список доступных команд
```

### 2. Кодгенератор (src/modules/codegen/)

Трехэтапный процесс преобразования Krait → Rust:

#### Этап 1: Лексер (lexer.rs)
```
Input:  "public func hello() -> txt"
Output: [Keyword("public"), Keyword("func"), Ident("hello"), ...]
```

#### Этап 2: Парсер (parser.rs)
```
Input:  [токены]
Output: AST
        FunctionDef {
            visibility: Public,
            name: "hello",
            return_type: "String",
            ...
        }
```

#### Этап 3: Генератор (gen.rs + backends/rust.rs)
```
Input:  AST
Output: pub fn hello() -> String {
            ...
        }
```

### 3. Система библиотек (libs.rs)

Управляет импортами Cargo крейтов:

```rust
Library {
    name: "REST API",
    crate_name: "actix-web",
    imports: ["use actix_web::..."],
    features: ["routing", "json"],
}
```

Использование в Krait:
```krait
import rest from actix
import json from serde

@route "/api/users" GET
    return users
end
```

## Основные типы данных

### Token (lexer.rs)
```rust
enum Token {
    Keyword(String),
    Identifier(String),
    Number(i64),
    String(String),
    Operator(String),
    ...
}
```

### AST узлы (parser.rs)
```rust
pub enum Statement {
    FunctionDef(FunctionDef),
    VariableDecl(VariableDecl),
    Route(RouteHandler),
    Import(ImportStmt),
    IfStatement(IfStatement),
    WhileLoop(WhileLoop),
    ...
}

pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: String,
    pub visibility: Visibility,
    pub body: Vec<Statement>,
}
```

### Типы видимости (parser.rs)
```rust
pub enum Visibility {
    Public,    // public fn in Rust → pub fn
    Private,   // private fn in Rust → fn (no pub)
}
```

## Процесс сборки

1. **Инициализация** (`krait init myapp`)
   - Создает папку `myapp/`
   - Генерирует структуру проекта

2. **Компиляция** (`krait build`)
   - Сканирует папку на `.kr` файлы
   - Для каждого файла:
     - Лексер: текст → токены
     - Парсер: токены → AST
     - Генератор: AST → Rust код
   - Сохраняет в `rust_code/`
   - Вызывает `cargo build`

3. **Запуск** (`krait run`)
   - Сборка (см. выше)
   - Запуск скомпилированного бинаря

## Примеры преобразований

### Простая функция

**Krait:**
```krait
public func greet() -> txt
    return "Hello, World!"
end
```

**Rust:**
```rust
pub fn greet() -> String {
    "Hello, World!".to_string()
}
```

### REST API маршрут

**Krait:**
```krait
@route "/api/users" GET
    txt users = get_all_users()
    return users
end
```

**Rust:**
```rust
#[get("/api/users")]
async fn get_api_users() -> impl Responder {
    let users = get_all_users();
    HttpResponse::Ok().json(users)
}
```

### Переменная с видимостью

**Krait:**
```krait
public txt name = "Alice"
private num age = 30
```

**Rust:**
```rust
pub const NAME: &str = "Alice";
const AGE: i32 = 30;
```

## Расширяемость

### Добавить новый бэкенд

1. Создать файл `src/modules/codegen/backends/new_lang.rs`
2. Реализовать трейт `CodeGenerator`
3. Зарегистрировать в `backends/mod.rs`

### Добавить новую библиотеку

1. Добавить крейт в `Cargo.toml`
2. Добавить запись в `LibraryRegistry::new()` в `libs.rs`
3. Использовать в Krait коде через `import`

### Добавить новый синтаксис

1. Добавить токены в `Token` enum (lexer.rs)
2. Добавить правила в parser (parser.rs)
3. Добавить AST узел (parser.rs)
4. Реализовать генерацию (gen.rs)

## Рабочий процесс разработки

```
Написать .kr файл
        ↓
krait build
        ↓
Лексер: text → tokens
        ↓
Парсер: tokens → AST
        ↓
Генератор: AST → Rust
        ↓
cargo build
        ↓
Бинарик готов!
```

## Лучшие практики

1. **Модульность** - каждый компонент отвечает за одно
2. **Тестируемость** - каждый модуль тестируется отдельно
3. **Документация** - все публичные API документированы
4. **Расширяемость** - легко добавить новые функции

