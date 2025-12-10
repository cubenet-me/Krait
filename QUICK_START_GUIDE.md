# Krait Quick Start Guide

## 5-Minute Quick Start

### 1. Create a Project

```bash
krait init hello_world
cd hello_world
```

### 2. Write Your Code

Edit `krait_src/main.kr`:

```krait
public func main()
    print("Hello from Krait!")
end
```

### 3. Build & Run

```bash
krait build
./target/release/hello_world
```

**Output:**
```
Hello from Krait!
```

---

## Common Examples

### Example 1: Functions & Variables

**File: `math.kr`**

```krait
public func add(int a, int b) -> int
    return a + b
end

public func multiply(int a, int b) -> int
    result = a * b
    return result
end

public func main()
    x = 5
    y = 3
    
    sum = add(x, y)
    product = multiply(x, y)
    
    print("Sum: " + sum)
    print("Product: " + product)
end
```

**Run:**
```bash
krait math.kr math.rs
rustc math.rs && ./math
```

**Output:**
```
Sum: 8
Product: 15
```

---

### Example 2: Control Flow

**File: `grade.kr`**

```krait
public func get_grade(int score) -> txt
    if score >= 90
        return "A"
    else if score >= 80
        return "B"
    else if score >= 70
        return "C"
    else if score >= 60
        return "D"
    else
        return "F"
    end
end

public func main()
    score = 85
    grade = get_grade(score)
    print("Score: " + score + " → Grade: " + grade)
end
```

---

### Example 3: Loops

**File: `loops.kr`**

```krait
public func count_to(int n)
    i = 0
    while i < n
        print(i)
        i = i + 1
    end
end

public func sum_range(int limit) -> int
    total = 0
    for i in 0..limit
        total = total + i
    end
    return total
end

public func main()
    print("Counting:")
    count_to(5)
    
    print("Sum 0-10:")
    result = sum_range(11)
    print(result)
end
```

---

### Example 4: Strings & Text

**File: `text.kr`**

```krait
public func greet(txt name) -> txt
    return "Hello, " + name + "!"
end

public func format_message(txt text) -> txt
    upper = text.to_uppercase()
    return ">>> " + upper + " <<<"
end

public func main()
    message = greet("Alice")
    print(message)
    
    formatted = format_message("welcome")
    print(formatted)
end
```

**Output:**
```
Hello, Alice!
>>> WELCOME <<<
```

---

### Example 5: Simple CLI App

**File: `cli.kr`**

```krait
public func show_menu()
    print("=== Menu ===")
    print("1. Say Hello")
    print("2. Show Info")
    print("3. Exit")
end

public func say_hello()
    print("Hello there!")
end

public func show_info()
    print("This is Krait v0.2.0")
end

public func main()
    print("Welcome to Krait CLI")
    show_menu()
    say_hello()
    show_info()
end
```

---

### Example 6: Web API (REST)

**File: `api.kr`**

```krait
import api from rest

// Mock data
private txt[] users = ["Alice", "Bob", "Charlie"]

@route "/users" GET
public func list_users() -> json
    return {
        "count": 3,
        "users": users
    }
end

@route "/users" POST
public func add_user(txt name) -> json
    return {
        "status": "created",
        "user": name
    }
end

@route "/status" GET
public func health_check() -> json
    return {
        "status": "ok",
        "version": "0.2.0"
    }
end

public func main()
    print("API Server starting...")
end
```

**Build & Run:**
```bash
krait build
./target/release/api
```

**Test with curl:**
```bash
curl http://localhost:8080/status
curl http://localhost:8080/users
curl -X POST http://localhost:8080/users -d '{"name":"Dave"}'
```

---

### Example 7: Data Structures (Arrays)

**File: `arrays.kr`**

```krait
public func print_array(txt[] items)
    for item in items
        print("- " + item)
    end
end

public func find_item(txt[] items, txt target) -> bool
    for item in items
        if item == target
            return true
        end
    end
    return false
end

public func main()
    fruits = ["apple", "banana", "cherry"]
    
    print("Fruits:")
    print_array(fruits)
    
    if find_item(fruits, "banana")
        print("Found banana!")
    end
end
```

---

### Example 8: Private & Public

**File: `visibility.kr`**

```krait
// Private function - only in this module
private func internal_setup()
    print("Setting up...")
end

// Public function - can be imported
public func initialize()
    internal_setup()
    print("Initialized!")
end

// Private variable
private int counter = 0

// Public function to use private variable
public func increment() -> int
    counter = counter + 1
    return counter
end

public func main()
    initialize()
    print("Count: " + increment())
    print("Count: " + increment())
    print("Count: " + increment())
end
```

---

## Project Templates

### Template 1: CLI Tool

```bash
krait init cli_tool
cd cli_tool
```

Edit `krait_src/main.kr`:

```krait
public func show_help()
    print("Usage: cli_tool [command]")
end

public func main()
    show_help()
end
```

### Template 2: Web API

```bash
krait init web_api
cd web_api
```

Edit `krait_src/main.kr`:

```krait
import api from rest

@route "/api/hello" GET
public func hello() -> json
    return {"message": "Hello from Krait!"}
end

public func main()
    print("API running...")
end
```

### Template 3: Data Processing

```bash
krait init data_processor
cd data_processor
```

Edit `krait_src/main.kr`:

```krait
public func process_data(int[] data) -> int
    total = 0
    for item in data
        total = total + item
    end
    return total
end

public func main()
    data = [1, 2, 3, 4, 5]
    result = process_data(data)
    print("Result: " + result)
end
```

---

## File Extensions

| Extension | Purpose | Example |
|-----------|---------|---------|
| `.kr` | Krait source code | `main.kr`, `api.kr` |
| `.krm` | Krait modules | `modules.krm` |
| `.rs` | Generated Rust | `main.rs` (auto-generated) |

---

## Common Commands

### Create project
```bash
krait init myapp
```

### Translate one file
```bash
krait input.kr output.rs
```

### Translate directory
```bash
krait project ./src ./out
```

### Build & compile
```bash
krait build
```

### Show help
```bash
krait --help
```

### Show version
```bash
krait --version
```

---

## Tips & Tricks

### 1. Use Type Inference

```krait
// Good - let Krait infer types
x = 42
name = "Alice"

// Also valid - explicit types
int age = 30
txt email = "alice@example.com"
```

### 2. String Concatenation

```krait
// Simple concatenation
greeting = "Hello " + name

// Multiple concatenation
message = "Name: " + name + ", Age: " + age
```

### 3. Return Values

```krait
// Explicit return
public func add(int a, int b) -> int
    return a + b
end

// Implicit return (last expression)
public func multiply(int a, int b) -> int
    a * b
end
```

### 4. Private Helpers

```krait
public func public_api()
    helper_function()
    print("Done")
end

private func helper_function()
    print("Internal work")
end
```

### 5. API Routes

```krait
// GET request
@route "/api/users" GET
public func list_users() -> json
    return {"users": []}
end

// POST with data
@route "/api/users" POST
public func create(txt name) -> json
    return {"id": 1, "name": name}
end

// Dynamic path
@route "/api/users/:id" GET
public func get_user(int id) -> json
    return {"id": id}
end
```

---

## Debugging Tips

### Check syntax errors

```bash
krait file.kr output.rs 2>&1
```

### View generated Rust code

```bash
cat rust_code/main.rs
```

### Compile generated code directly

```bash
rustc rust_code/main.rs && ./main
```

---

## Next Steps

1. ✅ Read the [full documentation](./KRAIT_DOCUMENTATION.md)
2. ✅ Explore more [examples](./examples/)
3. ✅ Join the community discussions
4. ✅ Contribute improvements

---

**Happy coding with Krait! 🐍**
