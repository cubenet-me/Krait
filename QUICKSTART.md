# Krait - Quick Start Guide

## 📦 Что это?

**Krait** - это язык программирования, похожий на Python, но скомпилированный в безопасный и быстрый Rust код. Идеален для построения:
- 🌐 REST API приложений
- 💻 CLI инструментов
- ⚡ Высокопроизводительных сервисов

## 🚀 Установка и первый проект

### 1. Клонирование репозитория
```bash
git clone <krait-repo>
cd krait
cargo build --release
```

### 2. Создание нового проекта
```bash
./target/release/krait init myapp
cd myapp
```

### 3. Написание кода на Krait

Создайте файл `main.kr`:

```krait
public func hello() -> txt
    return "Hello, World!"
end

public func main() -> num
    return 0
end
```

### 4. Компиляция в Rust
```bash
krait build
```

Это создаст `rust_code/main.rs` с валидным Rust кодом.

### 5. Запуск
```bash
krait run
```

## 📝 Синтаксис Krait

### Типы данных

| Тип | Rust | Описание |
|-----|------|---------|
| `txt` | `String` | Текст |
| `num` | `i64` | Целое число |
| `bool` | `bool` | Логическое значение |
| `json` | `serde_json::Value` | JSON объект |

### Функции

```krait
public func greet(name txt) -> txt
    return "Hello, " + name
end

private func helper() -> num
    return 42
end
```

### Переменные

```krait
public txt greeting = "Hello"      # Публичная переменная
private num count = 10             # Приватная переменная
```

### Условия

```krait
if x > 5
    return "big"
else
    return "small"
end
```

### Циклы

```krait
while i < 10
    i = i + 1
end
```

## 🌐 REST API

### Базовый маршрут

```krait
import rest from actix
import json from serde

@route "/api/hello" GET
    return "Hello API"
end

@route "/api/user/:id" GET
    txt id = path_param("id")
    return "User " + id
end
```

Это сгенерирует полный Actix-web сервер с:
- ✅ Корректными импортами
- ✅ Async функциями
- ✅ JSON сериализацией
- ✅ HttpServer инициализацией

## 💾 Структура проекта

```
myapp/
├── main.kr              # Основной код (функции, логика)
├── models.kr            # Модели данных
├── utils.kr             # Утилиты
├── Cargo.toml           # Rust зависимости (генерируется)
└── rust_code/           # Сгенерированный Rust (git ignored)
    ├── main.rs
    ├── models.rs
    └── utils.rs
```

## 📚 Примеры

### Пример 1: Simple CLI

**main.kr**
```krait
public func cmd_help() -> txt
    return "Available: help, info, exit"
end

public func cmd_info() -> txt
    return "Krait CLI v1.0"
end

public func main() -> num
    txt help = cmd_help()
    txt info = cmd_info()
    return 0
end
```

### Пример 2: REST API с пользователями

**main.kr**
```krait
import rest from actix
import json from serde

@route "/api/users" GET
    return "All users"
end

@route "/api/users/:id" GET
    txt user_id = path_param("id")
    return "User: " + user_id
end

@route "/api/users" POST
    txt data = request_json()
    return "Created user"
end
```

## 🔧 Добавление новой библиотеки

### 1. Добавьте в Cargo.toml
```toml
[dependencies]
my-library = "1.0"
```

### 2. Зарегистрируйте в src/modules/codegen/libs.rs
```rust
libs.insert(
    "my-lib".to_string(),
    Library {
        name: "My Library".to_string(),
        crate_name: "my-library".to_string(),
        imports: vec!["use my_library::{Type};".to_string()],
        features: vec!["feature".to_string()],
    },
);
```

### 3. Используйте в коде
```krait
import my-lib from crate

public func example() -> txt
    return "Using my library"
end
```

## 📋 Команды

```bash
krait init <name>       # Создать новый проект
krait build             # Скомпилировать в Rust
krait run               # Запустить приложение
krait list              # Список доступных команд
```

## 🎓 Документация

- **ARCHITECTURE.md** - Полная архитектура компилятора
- **LIBRARY_GUIDE.md** - Добавление библиотек
- **REFACTOR_SUMMARY.md** - Что было сделано

## ⚠️ Ограничения текущей версии

- REST API только с Actix-web
- Основные типы: txt, num, bool, json
- Функции, переменные, условия, циклы

## 🤝 Контрибьюции

Krait распространяется под лицензией GPL 3.0.

Для добавления новых функций:
1. Отредактируйте lexer.rs (новые токены)
2. Обновите parser.rs (грамматика)
3. Расширьте gen.rs (генерация)
4. Добавьте примеры в документацию

## 📞 Поддержка

- Проверьте ARCHITECTURE.md для понимания компилятора
- Смотрите LIBRARY_GUIDE.md для новых библиотек
- Примеры в этом файле - START HERE для новичков

---

**Happy coding with Krait! 🐍⚡**
