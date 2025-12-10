# Krait Programming Language - Complete Documentation

> **Krait** - Python-like syntax with Rust performance and safety
>
> Version 0.3.0 | License: GPL 3.0

---

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [Language Syntax](#language-syntax)
5. [Type System](#type-system)
6. [Functions](#functions)
7. [Variables and Scope](#variables-and-scope)
8. [Control Flow](#control-flow)
9. [REST API](#rest-api)
10. [Modules and Imports](#modules-and-imports)
11. [Library Management](#library-management)
12. [Code Generation](#code-generation)
13. [Project Structure](#project-structure)
14. [Examples](#examples)
15. [CLI Reference](#cli-reference)
16. [Architecture](#architecture)
17. [Contributing](#contributing)

---

## Introduction

**Krait** is a compiled programming language that combines:
- 🐍 **Python-like syntax** - Easy to learn and read
- ⚡ **Rust performance** - Compiled to native code
- 🛡️ **Memory safety** - No null pointers, no data races
- 🔌 **Web-ready** - Built-in REST API support via Actix-web

### Why Krait?

| Feature | Python | Rust | Krait |
|---------|--------|------|-------|
| Easy syntax | ✅ | ❌ | ✅ |
| Performance | ❌ | ✅ | ✅ |
| Memory safe | ❌ | ✅ | ✅ |
| REST API | ✅ | ⚠️ | ✅ |

---

## Installation

### Requirements
- Rust 1.70+ (installed via rustup)
- Cargo

### Setup

```bash
# Clone repository
git clone https://github.com/yourusername/krait.git
cd krait

# Build compiler
cargo build --release

# Install globally (optional)
cargo install --path .
```

---

## Quick Start

### 1. Create a Project

```bash
krait init hello_world
cd hello_world
```

### 2. Write Krait Code

Create `main.kr`:

```krait
public func hello() -> txt
    return "Hello, Krait!"
end

public func main() -> num
    hello()
    return 0
end
```

### 3. Compile and Run

```bash
krait build
krait run
```

Output:
```
Hello, Krait!
```

---

## Language Syntax

### Comments

```krait
# Single-line comment

# This function does something
public func example() -> txt
    return "result"
end
```

### Statement Blocks

All blocks are terminated with `end`:

```krait
public func example() -> num
    # Function body
    num result = 42
    return result
end
```

### Line Continuation

Statements end with newline. No semicolons needed.

---

## Type System

### Built-in Types

```krait
txt message = "Hello"           # String
num count = 42                  # Integer (i64)
bool is_active = true           # Boolean
json data = {}                  # JSON object
```

### Type Conversions

```krait
num age = 25
txt age_text = age.to_string()

txt number = "42"
num num_value = number.to_num()
```

### Type Compatibility

| Krait | Rust | Notes |
|-------|------|-------|
| `txt` | `String` | UTF-8 text |
| `num` | `i64` | Signed integer |
| `bool` | `bool` | true/false |
| `json` | `serde_json::Value` | Any JSON type |

---

## Functions

### Function Declaration

```krait
public func greet(name txt) -> txt
    return "Hello, " + name
end
```

### Multiple Parameters

```krait
public func add(a num, b num) -> num
    return a + b
end
```

### No Return Value

```krait
public func log(message txt) -> void
    print(message)
end
```

### Private Functions

```krait
private func helper() -> txt
    return "internal"
end

public func use_helper() -> txt
    return helper()
end
```

### Function Overloading

Not supported. Use different function names.

```krait
public func format_num(n num) -> txt
    return n.to_string()
end

public func format_txt(t txt) -> txt
    return t
end
```

---

## Variables and Scope

### Declaration

```krait
public txt name = "Alice"       # Public constant
private num age = 30             # Private constant
```

### Mutable Variables

```krait
public txt greeting = "Hello"
greeting = "Hi"                 # Allowed

private num counter = 0
counter = counter + 1           # Allowed
```

### Variable Scope

Variables are block-scoped:

```krait
public func example() -> txt
    txt outer = "visible"
    
    if true
        txt inner = "block-scoped"
    end
    
    return outer                # ✅ OK
    # return inner             # ❌ Error: undefined
end
```

---

## Control Flow

### if-else

```krait
if x > 10
    return "big"
else if x > 5
    return "medium"
else
    return "small"
end
```

### while Loop

```krait
num i = 0
while i < 10
    i = i + 1
end
```

### for Loop (planned)

Currently use while loops:

```krait
num i = 0
while i < 5
    # do something
    i = i + 1
end
```

### Pattern Matching (planned)

Coming in v0.4.0

---

## REST API

### Basic Routes

```krait
import rest from actix

@route "/api/hello" GET
    return "Hello, API!"
end

@route "/api/user/:id" GET
    txt user_id = path_param("id")
    return "User: " + user_id
end
```

### HTTP Methods

```krait
@route "/api/items" GET
    return "All items"
end

@route "/api/items" POST
    json data = request_json()
    return "Created"
end

@route "/api/items/:id" PUT
    txt id = path_param("id")
    return "Updated"
end

@route "/api/items/:id" DELETE
    txt id = path_param("id")
    return "Deleted"
end
```

### Request/Response

```krait
@route "/api/process" POST
    # Get request body as JSON
    json payload = request_json()
    
    # Get path parameter
    txt id = path_param("id")
    
    # Get query parameter
    txt filter = query_param("filter")
    
    # Return JSON response
    return {
        "status": "ok",
        "data": payload
    }
end
```

### Status Codes

```krait
@route "/api/items" GET
    return json_response(200, {"items": []})
end

@route "/api/error" GET
    return json_response(500, {"error": "Server error"})
end
```

---

## Modules and Imports

### Importing Libraries

```krait
import rest from actix       # REST API support
import json from serde       # JSON handling
import db from sqlx          # Database access
```

### Module Files

Create `utils.kr`:

```krait
public func format_name(first txt, last txt) -> txt
    return first + " " + last
end
```

Use in `main.kr`:

```krait
import utils from local

public func main() -> num
    txt name = utils.format_name("John", "Doe")
    return 0
end
```

---

## Library Management

### Available Libraries

| Name | Key | Crate | Purpose |
|------|-----|-------|---------|
| REST API | `rest` | actix-web | Web routing |
| JSON | `json` | serde_json | JSON parsing |
| Database | `sqlx` | sqlx | SQL queries |
| MongoDB | `mongodb` | mongodb | NoSQL database |
| Async | `tokio` | tokio | Async runtime |
| Logging | `log` | log | Logging |
| Serialization | `serde` | serde | Serialization |

### Adding Custom Libraries

#### Step 1: Update Cargo.toml

```toml
[dependencies]
my-library = "1.0"
```

#### Step 2: Register in libs.rs

Edit `src/modules/codegen/libs.rs`:

```rust
libs.insert(
    "my-lib".to_string(),
    Library {
        name: "My Library".to_string(),
        crate_name: "my-library".to_string(),
        imports: vec![
            "use my_library::{SomeType, some_func};".to_string(),
        ],
        features: vec!["feature1".to_string()],
    },
);
```

#### Step 3: Use in Krait

```krait
import my-lib from custom

public func use_library() -> txt
    return "Using my library"
end
```

---

## Code Generation

### How It Works

```
Krait Source (.kr)
        ↓
Lexer (tokenization)
        ↓
Parser (AST building)
        ↓
Code Generator (Rust output)
        ↓
rust_code/main.rs
        ↓
cargo build
        ↓
Binary
```

### Generated Rust Examples

#### Simple Function

**Krait:**
```krait
public func greet(name txt) -> txt
    return "Hello, " + name
end
```

**Generated Rust:**
```rust
pub fn greet(name: String) -> String {
    format!("Hello, {}", name)
}
```

#### REST Route

**Krait:**
```krait
@route "/api/users" GET
    return "Users"
end
```

**Generated Rust:**
```rust
#[get("/api/users")]
async fn get_api_users() -> impl Responder {
    HttpResponse::Ok().json("Users")
}
```

#### Variables

**Krait:**
```krait
public txt greeting = "Hello"
private num age = 30
```

**Generated Rust:**
```rust
pub const GREETING: &str = "Hello";
const AGE: i32 = 30;
```

---

## Project Structure

### New Project Layout

```
myapp/
├── main.kr              # Main code
├── models.kr            # Data models
├── handlers.kr          # Route handlers
├── utils.kr             # Utilities
├── Cargo.toml          # Rust dependencies (generated)
└── rust_code/          # Generated Rust (git ignored)
    ├── main.rs
    ├── models.rs
    ├── handlers.rs
    └── utils.rs
```

### .gitignore

```
/rust_code/
/target/
Cargo.lock
*.kr.rs
*.krm.rs
.env
.DS_Store
```

---

## Examples

### Example 1: Simple CLI

**main.kr**
```krait
public func show_help() -> txt
    return "Available commands: help, version, exit"
end

public func show_version() -> txt
    return "v1.0.0"
end

public func main() -> num
    txt help = show_help()
    txt version = show_version()
    return 0
end
```

### Example 2: REST API Server

**main.kr**
```krait
import rest from actix
import json from serde

@route "/api/health" GET
    return {
        "status": "ok",
        "version": "1.0"
    }
end

@route "/api/users" GET
    return [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]
end

@route "/api/users" POST
    json user = request_json()
    return {
        "id": 3,
        "name": user["name"]
    }
end

@route "/api/users/:id" GET
    txt id = path_param("id")
    return {
        "id": id,
        "name": "User " + id
    }
end
```

### Example 3: Utility Module

**utils.kr**
```krait
public func slugify(text txt) -> txt
    return text.to_lowercase()
end

public func truncate(text txt, length num) -> txt
    return text.substring(0, length)
end

private func validate_input(text txt) -> bool
    return text.length() > 0
end
```

**main.kr**
```krait
import utils from local

public func process(input txt) -> txt
    if utils.validate_input(input)
        return utils.slugify(input)
    else
        return "invalid"
    end
end
```

---

## CLI Reference

### Commands

#### krait init

Create a new project:

```bash
krait init <project-name>
krait init my_app
```

Creates:
```
my_app/
├── main.kr
└── Cargo.toml
```

#### krait build

Compile Krait to Rust and build:

```bash
krait build
```

Generates:
```
rust_code/
├── main.rs
├── Cargo.toml
└── [other modules].rs
```

Then runs `cargo build`.

#### krait run

Build and run:

```bash
krait run
```

Equivalent to:
```bash
krait build && ./target/debug/app_name
```

#### krait list

List available commands:

```bash
krait list
```

---

## Architecture

### Compiler Stages

```
┌─────────────────────┐
│  Krait Source       │
│      (.kr)          │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Lexer (lexer.rs)   │
│  Text → Tokens      │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Parser (parser.rs)  │
│ Tokens → AST        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ CodeGen (gen.rs)    │
│ AST → Rust Code     │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Rust Source         │
│    (main.rs)        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Rust Compiler       │
│ cargo build         │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Binary              │
│ (executable)        │
└─────────────────────┘
```

### Module System

```
src/modules/
├── cli/
│   └── mod.rs          # Command handling
├── api/
│   └── mod.rs          # HTTP support
├── codegen/
│   ├── lexer.rs        # Tokenization
│   ├── parser.rs       # Parsing
│   ├── gen.rs          # Code generation
│   ├── libs.rs         # Library registry
│   └── backends/
│       └── rust.rs     # Rust backend
└── mod.rs              # Module root
```

### AST Types

Core AST nodes in `parser.rs`:

```rust
pub enum Statement {
    FunctionDef(FunctionDef),
    VariableDecl(VariableDecl),
    Route(RouteHandler),
    Import(ImportStmt),
    IfStatement(IfStatement),
    WhileLoop(WhileLoop),
    Return(Expression),
}

pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: String,
    pub visibility: Visibility,
    pub body: Vec<Statement>,
}

pub enum Visibility {
    Public,
    Private,
}
```

---

## Contributing

### Adding Features

#### 1. New Syntax

Edit `lexer.rs`:
```rust
// Add new token type
Token::NewKeyword(String)
```

Edit `parser.rs`:
```rust
// Add parsing rules
pub enum Statement {
    NewStatement(NewStmt),
    // ...
}
```

#### 2. Code Generation

Edit `gen.rs` or `backends/rust.rs`:
```rust
Statement::NewStatement(stmt) => {
    // Generate Rust code
}
```

#### 3. Tests

Add tests in each module:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_new_feature() {
        // Test implementation
    }
}
```

### Code Style

- Follow Rust conventions
- Document public APIs
- Test before submitting
- Keep modules focused

---

## Roadmap

### v0.3.0 (Current)
- ✅ Basic functions
- ✅ Variables (public/private)
- ✅ REST API routes
- ✅ Library management
- ✅ Type system

### v0.4.0 (Planned)
- 🔄 Pattern matching
- 🔄 for loops
- 🔄 Structs and enums
- 🔄 Error handling

### v0.5.0 (Planned)
- 🔄 Database integration
- 🔄 Async/await syntax
- 🔄 Generics
- 🔄 Traits

---

## FAQ

### Q: Is Krait production-ready?

**A:** Not yet. v0.3.0 is suitable for learning and simple projects. Wait for v1.0.0 for production use.

### Q: Can I use Rust libraries?

**A:** Yes! Through the library management system. Register in `libs.rs` and import in Krait.

### Q: What about performance?

**A:** Krait compiles to Rust, so performance equals native Rust. No overhead.

### Q: How does it handle errors?

**A:** Currently through return codes. Try/catch coming in v0.4.0.

### Q: Can I mix Krait and Rust?

**A:** Generated code is pure Rust. You can manually edit `rust_code/` files, but they'll be overwritten on next build.

---

## Resources

- **ARCHITECTURE.md** - Detailed compiler architecture
- **LIBRARY_GUIDE.md** - Adding custom libraries
- **QUICKSTART.md** - Getting started
- **Examples** - Sample projects

---

## License

Krait is licensed under **GNU General Public License v3.0**

See [LICENSE](LICENSE) file for details.

---

## Support

Need help?

1. Check **QUICKSTART.md** for basics
2. Read **ARCHITECTURE.md** for deep understanding
3. Look at **Examples** folder
4. Open an issue on GitHub

---

**Krait Programming Language**

*Python-like syntax, Rust performance, Web-ready* 🐍⚡

Last updated: December 10, 2024
