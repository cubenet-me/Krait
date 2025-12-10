# Krait Programming Language - Complete Documentation

**Version:** 0.2.0  
**Status:** Active Development  
**Compiler:** Krait → Rust (via LLVM)

---

## Table of Contents

1. [Introduction](#introduction)
2. [Language Basics](#language-basics)
3. [Data Types](#data-types)
4. [Variables](#variables)
5. [Functions](#functions)
6. [Control Flow](#control-flow)
7. [Operators](#operators)
8. [String Operations](#string-operations)
9. [Collections](#collections)
10. [Modules & Imports](#modules--imports)
11. [Web API with Krait](#web-api-with-krait)
12. [CLI Usage](#cli-usage)
13. [Project Structure](#project-structure)
14. [Examples](#examples)
15. [Advanced Topics](#advanced-topics)

---

## Introduction

**Krait** is a modern, statically-typed programming language designed to make backend development faster and safer. It compiles to Rust, inheriting Rust's performance and memory safety while providing a more developer-friendly syntax inspired by Python.

### Key Features

- **Fast & Safe**: Compiles to Rust → LLVM → native code
- **Python-like Syntax**: Easy to read and write
- **Type System**: Optional type annotations with inference
- **Web Framework**: Built-in support for REST APIs
- **Module System**: Clean separation of concerns
- **Zero Runtime Overhead**: Everything compiles to optimized Rust

### Why Krait?

Krait is perfect for:
- Backend applications (Web APIs, microservices)
- CLI tools
- System utilities
- Data processing pipelines
- Real-time applications

---

## Language Basics

### Hello World

```krait
public func main()
    print("Hello, World!")
end
```

Compile and run:
```bash
krait hello.kr hello.rs
rustc hello.rs && ./hello
```

### Comments

```krait
// Single line comment
/* Multi-line comment */
```

### Naming Conventions

- Functions: `snake_case` → `my_function`
- Constants: `UPPER_SNAKE_CASE` → `MAX_SIZE`
- Types: `PascalCase` → `MyStruct`
- Variables: `snake_case` → `user_name`

---

## Data Types

Krait is statically typed with support for type inference.

### Primitive Types

| Type | Rust Equivalent | Example |
|------|-----------------|---------|
| `int` | `i32` | `42` |
| `i64` | `i64` | `42i64` |
| `f32` | `f32` | `3.14f32` |
| `f64` | `f64` | `3.14` |
| `bool` | `bool` | `true`, `false` |
| `txt` | `String` | `"Hello"` |
| `str` | `&'static str` | `"literal"` |

### Examples

```krait
public func types_demo()
    int count = 42
    f64 pi = 3.14159
    bool is_active = true
    txt message = "Krait is awesome"
    print(message)
end
```

### Type Inference

```krait
public func infer_types()
    // Type is inferred as int
    x = 100
    
    // Type is inferred as txt
    name = "Alice"
    
    // Type is inferred as bool
    enabled = true
end
```

---

## Variables

### Variable Declaration

#### Public Variables (exported)

```krait
public txt global_message = "Available everywhere"
public int VERSION = 1
```

#### Private Variables (internal only)

```krait
private txt secret = "Only in this module"
private int counter = 0
```

#### Local Variables

```krait
public func example()
    // Local variable - only exists in function
    txt local_text = "Inside function"
    int temp = 10
end
```

### Mutability

By default, all variables are mutable:

```krait
public func mutability()
    txt message = "Hello"
    message = "World"  // OK - reassignment works
    
    int counter = 0
    counter = counter + 1  // OK
end
```

---

## Functions

### Function Declaration

```krait
// Basic function
public func greet()
    print("Hello!")
end

// Function with parameters
public func add(int a, int b) -> int
    return a + b
end

// Function with type annotations
public func format_user(txt name, int age) -> txt
    return name + " is " + age
end
```

### Return Values

```krait
public func get_status() -> txt
    return "ok"
end

public func calculate(int x, int y) -> int
    result = x * y
    return result
end
```

### Implicit Return

```krait
public func implicit_return(int x) -> int
    x + 10  // Returns x + 10 (no return keyword needed)
end
```

### Function Parameters

```krait
// Parameters with types
public func process(txt data, int count)
    print(data)
end

// Optional parameters (with defaults)
public func greet_user(txt name)
    greeting = "Hello, " + name
    print(greeting)
end
```

### Private Functions

```krait
private func helper_function()
    // Only visible within module
    print("Internal use only")
end

public func public_function()
    helper_function()
end
```

---

## Control Flow

### If/Else

```krait
public func check_age(int age)
    if age >= 18
        print("Adult")
    else
        print("Minor")
    end
end

public func grade(int score) -> txt
    if score >= 90
        return "A"
    else if score >= 80
        return "B"
    else if score >= 70
        return "C"
    else
        return "F"
    end
end
```

### Comparison Operators

```krait
public func comparisons(int x, int y)
    if x == y
        print("Equal")
    end
    
    if x != y
        print("Not equal")
    end
    
    if x > y
        print("X is greater")
    end
    
    if x < y
        print("X is less")
    end
    
    if x >= y
        print("X is greater or equal")
    end
    
    if x <= y
        print("X is less or equal")
    end
end
```

### Logical Operators

```krait
public func logic(bool a, bool b)
    if a && b
        print("Both true")
    end
    
    if a || b
        print("At least one true")
    end
    
    if !a
        print("A is false")
    end
end
```

### While Loop

```krait
public func count_up(int limit)
    i = 0
    while i < limit
        print(i)
        i = i + 1
    end
end
```

### For Loop (Range)

```krait
public func for_range()
    // Loop from 0 to 9
    for i in 0..10
        print(i)
    end
end
```

### Break and Continue

```krait
public func search(txt target)
    i = 0
    while i < 100
        if i == target
            break
        end
        i = i + 1
    end
end

public func skip_odd()
    i = 0
    while i < 20
        i = i + 1
        if i % 2 == 0
            continue
        end
        print(i)
    end
end
```

---

## Operators

### Arithmetic Operators

```krait
public func arithmetic()
    a = 10
    b = 3
    
    sum = a + b        // 13
    diff = a - b       // 7
    prod = a * b       // 30
    quot = a / b       // 3
    rem = a % b        // 1
end
```

### Assignment Operators

```krait
public func assignments()
    x = 10
    
    x = x + 5          // x = 15
    x = x - 3          // x = 12
    x = x * 2          // x = 24
    x = x / 4          // x = 6
    x = x % 3          // x = 0
end
```

### Compound Operators (shorthand)

```krait
public func compound()
    x = 10
    x += 5             // x = 15
    x -= 3             // x = 12
    x *= 2             // x = 24
    x /= 4             // x = 6
    x %= 3             // x = 0
end
```

---

## String Operations

### String Concatenation

```krait
public func concat_strings()
    first = "Hello"
    second = "World"
    
    result = first + " " + second
    print(result)      // "Hello World"
end
```

### String Interpolation

```krait
public func interpolation()
    name = "Alice"
    age = 30
    
    message = "My name is " + name + " and I'm " + age + " years old"
    print(message)
end
```

### String Methods (via Rust)

```krait
public func string_methods(txt text)
    // .len() → string length
    length = text.len()
    
    // .to_uppercase() → convert to uppercase
    upper = text.to_uppercase()
    
    // .to_lowercase() → convert to lowercase
    lower = text.to_lowercase()
end
```

---

## Collections

### Arrays (Vectors)

```krait
public func arrays()
    // Create empty array
    items = []
    
    // Array with initial values
    numbers = [1, 2, 3, 4, 5]
    
    // Array of strings
    names = ["Alice", "Bob", "Charlie"]
    
    // Access element
    first = numbers[0]
    
    // Get length
    size = numbers.len()
end
```

### Maps (HashMaps)

```krait
public func maps()
    // Create empty map
    data = {}
    
    // Map with initial data
    user = {
        "name": "Alice",
        "age": "30"
    }
    
    // Access value
    name = user["name"]
    
    // Set value
    user["city"] = "New York"
end
```

---

## Modules & Imports

### Module System

Each `.kr` file is a module. You can organize code using modules.

### Import Syntax

```krait
// Import from standard library
import math from std
import json from std
import sys from std

// Import from local module
import helpers from ./helpers
import database from ./db/connection
```

### Creating Modules

Create a file `utils.kr`:

```krait
public func format_text(txt text) -> txt
    return text.to_uppercase()
end

public func validate_email(txt email) -> bool
    // Simple validation
    has_at = email.contains("@")
    return has_at
end

private func internal_helper()
    // Only visible in this module
end
```

Use in another file:

```krait
import utils from ./utils

public func main()
    formatted = utils.format_text("hello")
    print(formatted)
end
```

---

## Web API with Krait

### Import Web API

```krait
import api from rest
```

### Route Handlers

Routes are declared with `@route` decorator:

```krait
@route "/api/users" GET
public func get_users() -> json
    data = {
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]
    }
    return data
end
```

### Route with Path Parameters

```krait
@route "/api/users/:id" GET
public func get_user(int id) -> json
    data = {"id": id, "name": "User"}
    return data
end
```

### POST Request with Body

```krait
@route "/api/users" POST
public func create_user(txt name) -> json
    user = {
        "id": 3,
        "name": name
    }
    return user
end
```

### PUT Request

```krait
@route "/api/users/:id" PUT
public func update_user(int id, txt name) -> json
    data = {"id": id, "name": name}
    return data
end
```

### DELETE Request

```krait
@route "/api/users/:id" DELETE
public func delete_user(int id) -> json
    return {"status": "deleted"}
end
```

### Complete API Example

```krait
import api from rest

// GET /api/status
@route "/api/status" GET
public func status() -> json
    return {"status": "ok"}
end

// POST /api/data
@route "/api/data" POST
public func process_data(txt input) -> json
    result = input + " processed"
    return {"result": result}
end

// GET /api/hello
@route "/api/hello" GET
public func hello() -> txt
    return "Hello from Krait!"
end
```

When you have routes, the compiler generates:
- Async main function with `#[actix_web::main]`
- HttpServer setup
- Route handlers with proper attributes

---

## CLI Usage

### Installation

Clone and build:
```bash
git clone https://github.com/yourusername/krait
cd krait
cargo build --release
cp target/release/krait /usr/local/bin/
```

### Creating a New Project

```bash
krait init myapp
cd myapp
```

This creates:
```
myapp/
├── Cargo.toml
├── README.md
├── .gitignore
├── krait_src/
│   └── main.kr
└── rust_code/
```

### Commands

#### Translate single file
```bash
krait input.kr output.rs
```

#### Translate entire directory
```bash
krait project ./krait_src ./rust_code
```

#### Build and compile
```bash
krait build
```

This:
1. Translates all `.kr` files to `rust_code/`
2. Compiles with `cargo build --release`

#### Show version
```bash
krait --version
```

#### Show help
```bash
krait --help
```

---

## Project Structure

### Recommended Layout

```
myproject/
├── Cargo.toml                 # Rust configuration
├── README.md
├── .gitignore
├── krait_src/                 # Source code in Krait
│   ├── main.kr
│   ├── api.kr
│   ├── models.kr
│   └── utils.kr
├── rust_code/                 # Generated Rust code (git ignored)
│   ├── main.rs
│   ├── api.rs
│   ├── models.rs
│   └── utils.rs
└── target/                    # Compiled binaries (git ignored)
    └── release/
        └── myproject
```

### Modular Architecture

For larger projects, use `.krm` module files:

```
krait_src/
├── main.kr
└── modules.krm
    ├── api.kr
    ├── database.kr
    ├── auth.kr
    └── models.kr
```

In `main.kr`:
```krait
import api from modules
import database from modules
import auth from modules
```

---

## Examples

### Example 1: Simple CLI

File: `cli.kr`

```krait
public func main()
    print("=== Krait CLI ===")
    print("Enter command: ")
end

public func process_command(txt cmd) -> txt
    if cmd == "help"
        return "Available: help, exit, status"
    else if cmd == "status"
        return "System OK"
    else
        return "Unknown command"
    end
end
```

### Example 2: REST API

File: `api.kr`

```krait
import api from rest

// In-memory user storage
private int next_id = 1
private txt[] users = []

@route "/users" GET
public func list_users() -> json
    return {"users": users}
end

@route "/users" POST
public func create_user(txt name) -> json
    user = {
        "id": next_id,
        "name": name
    }
    next_id = next_id + 1
    return user
end

@route "/users/:id" DELETE
public func delete_user(int id) -> json
    return {"status": "deleted", "id": id}
end

public func main()
    print("API Server running on http://localhost:8080")
end
```

### Example 3: Data Processing

File: `process.kr`

```krait
public func sum_numbers(int[] numbers) -> int
    total = 0
    for num in numbers
        total = total + num
    end
    return total
end

public func filter_even(int[] numbers) -> int[]
    result = []
    for num in numbers
        if num % 2 == 0
            result.push(num)
        end
    end
    return result
end

public func main()
    data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    total = sum_numbers(data)
    print("Sum: " + total)
    
    evens = filter_even(data)
    print("Even numbers: " + evens)
end
```

---

## Advanced Topics

### Memory Safety

Krait inherits Rust's memory safety guarantees:
- No null pointer exceptions
- No data races
- No buffer overflows

### Performance

Krait compiles to optimized Rust/LLVM code:
- Zero-cost abstractions
- Native execution speed
- Minimal overhead

### Async/Await (via Web API)

When using `@route`, handlers automatically become async:

```krait
@route "/api/data" GET
public func fetch_data() -> json
    // This becomes async fn automatically
    // Perfect for concurrent handling
    return {"data": "async"}
end
```

### Error Handling

```krait
public func divide(int a, int b) -> int
    if b == 0
        print("Error: Division by zero")
        return 0
    end
    return a / b
end
```

Future versions will support `try/catch` syntax.

### Generics (Planned)

Planned for v0.3.0:
```krait
public func first<T>(T[] items) -> T
    return items[0]
end
```

### Traits (Planned)

Planned for v0.3.0:
```krait
trait Serializable
    func to_json() -> json
end
```

---

## Compilation Process

```
source.kr
    ↓
[Lexer] → Tokens
    ↓
[Parser] → AST
    ↓
[Codegen] → Rust code
    ↓
[Cargo] → Binary
    ↓
Executable
```

### Type Checking

- Static typing at compile time
- Type inference where annotations omitted
- No runtime type errors

### Generated Code Quality

Generated Rust code:
- Uses only necessary imports
- Includes type annotations
- Follows Rust conventions
- Optimized for performance

---

## Troubleshooting

### Compilation Errors

**Error: "Function not found"**
- Ensure function is declared before use
- Check module imports

**Error: "Type mismatch"**
- Verify parameter types
- Check return type annotations

**Error: "File not found"**
- Verify file paths are correct
- Ensure `.kr` extension is used

### Performance Tips

1. Use `krait build --release` for optimized builds
2. Minimize string allocations in hot paths
3. Use arrays instead of repeated calculations

---

## Contributing

Krait is open source! Help us improve:

- Report issues
- Submit feature requests
- Contribute code improvements
- Write documentation

---

## License

Krait is licensed under the MIT License.

---

## Changelog

### v0.2.0 (Current)
- ✅ Public/private variables
- ✅ Import system for modules
- ✅ REST API support with `@route`
- ✅ Type inference
- ✅ `krait init` command
- ✅ CLI tooling improvements

### v0.1.4
- Basic syntax
- Function declarations
- Control flow (if/while/for)
- String operations

---

## Quick Reference

| Feature | Syntax |
|---------|--------|
| Function | `public func name() ... end` |
| Variable | `txt name = "value"` |
| Public var | `public int x = 10` |
| Private var | `private txt secret = "x"` |
| If statement | `if condition ... end` |
| While loop | `while condition ... end` |
| For loop | `for i in 0..10 ... end` |
| Return | `return value` |
| Comment | `// comment` |
| Route | `@route "/path" GET` |
| Import | `import name from module` |
| Print | `print("text")` |

---

## Resources

- **Repository**: https://github.com/yourusername/krait
- **Issue Tracker**: https://github.com/yourusername/krait/issues
- **Discussions**: https://github.com/yourusername/krait/discussions

---

**Last Updated**: 2025-12-10  
**Maintained by**: Krait Development Team

