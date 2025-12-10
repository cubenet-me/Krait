# Руководство по добавлению новых библиотек в Krait

## Обзор

Система управления библиотеками в Krait расположена в `src/modules/codegen/libs.rs`. Она позволяет легко добавлять новые зависимости из Cargo без изменения основного логики кодгенератора.

## Структура Library

```rust
pub struct Library {
    pub name: String,           // Удобное имя для отображения
    pub crate_name: String,     // Название крейта в Cargo
    pub imports: Vec<String>,   // Импорты для Rust кода
    pub features: Vec<String>,  // Категории функциональности
}
```

## Как добавить новую библиотеку

### Шаг 1: Добавьте крейт в Cargo.toml

```toml
[dependencies]
my-library = "1.0"
```

### Шаг 2: Добавьте библиотеку в LibraryRegistry

Отредактируйте файл `src/modules/codegen/libs.rs` в методе `new()`:

```rust
libs.insert(
    "my-lib".to_string(),  // Ключ для import в Krait
    Library {
        name: "My Library".to_string(),
        crate_name: "my-library".to_string(),  // Из Cargo.toml
        imports: vec![
            "use my_library::{SomeType, some_func};".to_string(),
        ],
        features: vec!["feature1".to_string(), "feature2".to_string()],
    },
);
```

### Шаг 3: Используйте в Krait коде

```krait
import my-lib from rest

public func example() -> txt
    return "Hello"
end
```

## Примеры встроенных библиотек

### REST API (Actix)
```rust
libs.insert(
    "rest".to_string(),
    Library {
        name: "REST API (Actix)".to_string(),
        crate_name: "actix-web".to_string(),
        imports: vec![
            "use actix_web::{get, post, put, delete, App, HttpServer, HttpResponse, web};".to_string(),
        ],
        features: vec!["routing".to_string(), "json".to_string()],
    },
);
```

### JSON обработка
```rust
libs.insert(
    "json".to_string(),
    Library {
        name: "JSON".to_string(),
        crate_name: "serde_json".to_string(),
        imports: vec![
            "use serde_json::{json, Value};".to_string(),
        ],
        features: vec!["serialization".to_string()],
    },
);
```

### База данных SQLx
```rust
libs.insert(
    "sqlx".to_string(),
    Library {
        name: "SQLx (SQL Database)".to_string(),
        crate_name: "sqlx".to_string(),
        imports: vec![
            "use sqlx::{PgPool, Row};".to_string(),
        ],
        features: vec!["async-runtime".to_string()],
    },
);
```

## Доступные встроенные библиотеки

| Ключ | Название | Крейт |
|------|----------|-------|
| `rest` | REST API (Actix) | actix-web |
| `json` | JSON | serde_json |
| `sqlx` | SQLx (SQL Database) | sqlx |
| `mongodb` | MongoDB | mongodb |
| `tokio` | Tokio (Async Runtime) | tokio |
| `log` | Log | log |
| `serde` | Serde | serde |

## Архитектура

```
src/modules/codegen/
├── libs.rs         <- Управление библиотеками
├── lexer.rs        <- Лексический анализ
├── parser.rs       <- Синтаксический анализ
├── gen.rs          <- Генерация Rust кода
└── mod.rs          <- Интеграция модулей
```

## Лучшие практики

1. **Группируйте импорты** - импортируйте только нужные типы и функции
2. **Используйте правильные имена крейтов** - они должны совпадать с Cargo.toml
3. **Добавляйте метаданные** - заполняйте `features` для категоризации
4. **Тестируйте** - проверяйте, что импорты генерируются корректно

## Пример: добавление библиотеки для работы с БД

```rust
// Шаг 1: Cargo.toml
[dependencies]
diesel = "2.0"

// Шаг 2: libs.rs
libs.insert(
    "diesel".to_string(),
    Library {
        name: "Diesel ORM".to_string(),
        crate_name: "diesel".to_string(),
        imports: vec![
            "use diesel::prelude::*;".to_string(),
            "use diesel::PgConnection;".to_string(),
        ],
        features: vec!["orm".to_string(), "database".to_string()],
    },
);

// Шаг 3: Krait код
import diesel from database

public func get_users() -> txt
    return "SELECT * FROM users"
end
```

## Система обнаружения импортов

Когда вы используете `import` в Krait коде, система автоматически:
1. Ищет библиотеку в LibraryRegistry
2. Добавляет необходимые импорты в сгенерированный Rust код
3. Дедупликирует импорты (не дублирует одинаковые)

