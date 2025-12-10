# 🐍 Krait Programming Language

**A fast, safe, and Python-like language that compiles to Rust**

![Version](https://img.shields.io/badge/version-0.2.0-blue)
![Status](https://img.shields.io/badge/status-active-green)
![License](https://img.shields.io/badge/license-MIT-green)

## ⚡ Quick Start

### Create a new project

```bash
krait init myapp
cd myapp
```

### Write your first program

Edit `krait_src/main.kr`:

```krait
public func main()
    print("Hello from Krait!")
end
```

### Compile and run

```bash
krait build
./target/release/myapp
```

## 📚 Documentation

**Full documentation available in:** [`KRAIT_DOCUMENTATION.md`](./KRAIT_DOCUMENTATION.md)

Covers:
- Language syntax and basics
- Type system and variables  
- Functions and control flow
- Web API development
- Module system
- CLI tools
- Advanced topics

## 🎯 Features

### ✅ Implemented

- **Dynamic typing with inference**: Automatic type detection
- **Public/Private variables**: Encapsulation support
- **Functions**: Full function support with parameters and return types
- **Control flow**: if/else, while, for loops
- **Strings**: Full string operations and concatenation
- **Collections**: Arrays and maps support
- **Web API**: REST API with `@route` decorator
- **Module system**: Import and modular code organization
- **CLI tools**: `krait init`, `krait build`, `krait translate`

### 🚀 Coming Soon (v0.3.0+)

- Error handling (`try/catch`)
- Generics
- Traits and interfaces
- Pattern matching
- Async/await improvements
- Standard library expansion

## 🏗️ Project Structure

```
myapp/
├── Cargo.toml              # Rust configuration
├── README.md
├── .gitignore
├── krait_src/              # Your Krait code
│   ├── main.kr
│   ├── api.kr
│   └── utils.kr
├── rust_code/              # Generated Rust code
└── target/                 # Compiled binaries
```

## 💻 CLI Commands

| Command | Description |
|---------|-------------|
| `krait init <app>` | Create new project |
| `krait <in.kr> <out.rs>` | Translate single file |
| `krait project <src> <out>` | Translate directory |
| `krait build` | Translate and compile |
| `krait --help` | Show help |
| `krait --version` | Show version |

## 📖 Examples

### Hello World

```krait
public func main()
    print("Hello, World!")
end
```

### Functions

```krait
public func add(int a, int b) -> int
    return a + b
end

public func main()
    result = add(5, 3)
    print(result)  // 8
end
```

### Control Flow

```krait
public func check_age(int age)
    if age >= 18
        print("You are an adult")
    else
        print("You are a minor")
    end
end
```

### Web API

```krait
import api from rest

@route "/users" GET
public func list_users() -> json
    return {
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]
    }
end

@route "/users" POST
public func create_user(txt name) -> json
    return {"id": 3, "name": name}
end
```

### CLI Application

```krait
public func main()
    print("=== Krait CLI ===")
    show_menu()
end

private func show_menu()
    print("Commands:")
    print("  1. List")
    print("  2. Add")
    print("  3. Exit")
end
```

## 🔧 How It Works

```
Your Code (Krait)
    ↓
Lexer (Tokenization)
    ↓
Parser (Syntax Analysis)
    ↓
Codegen (Rust Code)
    ↓
Cargo (Compilation)
    ↓
Native Binary
```

## 🛠️ Building from Source

### Prerequisites

- Rust 1.70+
- Cargo
- Git

### Build

```bash
git clone https://github.com/yourusername/krait.git
cd krait
cargo build --release
```

### Run

```bash
./target/release/krait --help
```

## 📦 Project Organization

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library exports
├── modules/
│   ├── lexer/          # Tokenization
│   ├── parser/         # Syntax analysis
│   ├── codegen/        # Code generation
│   ├── cli/            # Command line interface
│   ├── api/            # API generation
│   └── mod.rs          # Module definitions
└── tests/              # Tests
```

## 🐛 Troubleshooting

### Error: "Function not found"
- Ensure function is declared before use
- Check module imports

### Error: "Type mismatch"
- Verify parameter types
- Check return type annotations

### Compilation fails
- Check `.kr` file syntax
- Ensure all imports are valid
- Verify Cargo.toml is in project root

## 🤝 Contributing

We welcome contributions! Please:

1. Fork the repository
2. Create a feature branch
3. Submit a pull request
4. Follow the code style

## 📄 License

Krait is licensed under the **MIT License**.

## 🔗 Links

- **Documentation**: [`KRAIT_DOCUMENTATION.md`](./KRAIT_DOCUMENTATION.md)
- **GitHub**: https://github.com/yourusername/krait
- **Issues**: https://github.com/yourusername/krait/issues
- **Discussions**: https://github.com/yourusername/krait/discussions

## 💡 Design Philosophy

**"Python's Simplicity × Rust's Safety"**

Krait aims to:
- ✅ Make backend development faster
- ✅ Eliminate common runtime errors
- ✅ Provide familiar Python-like syntax
- ✅ Generate efficient Rust code
- ✅ Support modern web development

## 📊 Status

- **Version**: 0.2.0
- **Last Updated**: 2025-12-10
- **Maintained by**: Krait Development Team
- **Status**: Active Development

---

**Made with ❤️ for developers who value both speed and safety**
