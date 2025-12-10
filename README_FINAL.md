# 🐍 Krait Programming Language

> **Python-like syntax. Rust performance. Web-ready.**
>
> A compiled programming language combining Python's simplicity with Rust's speed and safety.

---

## 📚 Quick Links

- 🎯 **START HERE**: Read [START_HERE_DOCS.md](START_HERE_DOCS.md)
- 🚀 **GET STARTED**: Read [QUICKSTART.md](QUICKSTART.md)
- 📖 **FULL REFERENCE**: Read [KRAIT_COMPLETE_DOCS.md](KRAIT_COMPLETE_DOCS.md)
- 🏗️ **HOW IT WORKS**: Read [ARCHITECTURE.md](ARCHITECTURE.md)
- 📦 **ADD LIBRARIES**: Read [LIBRARY_GUIDE.md](LIBRARY_GUIDE.md)

---

## ⚡ One Minute Example

### Write Code (main.kr)
```krait
import rest from actix

@route "/api/hello" GET
    return "Hello, Krait!"
end
```

### Build & Run
```bash
krait build
krait run
```

### Get Fast REST API
```
GET http://localhost:8080/api/hello
→ "Hello, Krait!"
```

---

## 🎯 What is Krait?

| Aspect | Details |
|--------|---------|
| **Syntax** | Python-like, easy to learn |
| **Performance** | Compiled to native Rust code |
| **Safety** | Memory safe, no null pointers |
| **Web** | Built-in REST API support |
| **Type System** | txt, num, bool, json |
| **License** | GPL 3.0 |

---

## 📖 Documentation (2934 lines)

### Core Documents

1. **START_HERE_DOCS.md** (250 lines)
   - What documents exist
   - Which to read first
   - Navigation guide

2. **QUICKSTART.md** (200+ lines)
   - Installation
   - First project
   - Basic examples

3. **KRAIT_COMPLETE_DOCS.md** (949 lines) ⭐
   - Complete language reference
   - All syntax explained
   - Type system
   - REST API
   - Examples
   - CLI reference

4. **ARCHITECTURE.md** (280 lines)
   - Compiler internals
   - Code generation pipeline
   - Module system
   - How to extend

5. **LIBRARY_GUIDE.md** (180 lines)
   - How to add libraries
   - Cargo integration
   - Step-by-step guide

6. **DOCUMENTATION_INDEX_FINAL.md** (400 lines)
   - Document index
   - Quick reference
   - Troubleshooting

7. **FINAL_SUMMARY.md** (200 lines)
   - Project overview
   - What was done
   - Features list

8. **DOCUMENTATION_MANIFEST.md** (500+ lines)
   - Complete documentation structure
   - Reading recommendations
   - Coverage matrix

---

## 🚀 Quick Start

### 1. Create Project
```bash
krait init hello_world
cd hello_world
```

### 2. Write Code
```bash
cat > main.kr << 'EOF'
public func hello(name txt) -> txt
    return "Hello, " + name
end

public func main() -> num
    hello("Krait")
    return 0
end
EOF
```

### 3. Build & Run
```bash
krait build
krait run
```

---

## 📝 Language Features

### Functions
```krait
public func greet(name txt) -> txt
    return "Hello, " + name
end

private func helper() -> txt
    return "internal"
end
```

### Variables (Public/Private)
```krait
public txt greeting = "Hi"
private num count = 0
```

### Control Flow
```krait
if x > 10
    return "big"
else
    return "small"
end

num i = 0
while i < 10
    i = i + 1
end
```

### REST API
```krait
import rest from actix

@route "/api/users" GET
    return "Users"
end

@route "/api/users/:id" GET
    txt id = path_param("id")
    return "User " + id
end
```

### Imports
```krait
import rest from actix
import json from serde
import db from sqlx
```

---

## 🔧 CLI Commands

```bash
krait init <name>       # Create new project
krait build             # Compile to Rust and build
krait run               # Build and run
krait list              # Show available commands
```

---

## 💾 File Extensions

| Extension | Purpose |
|-----------|---------|
| `.kr` | Main code (functions, logic, routes) |
| `.krm` | Module definitions |
| Generated: `rust_code/` | Git ignored |

---

## 📦 Built-in Libraries

```
rest        → actix-web (REST API)
json        → serde_json (JSON)
sqlx        → sqlx (SQL databases)
mongodb     → mongodb (NoSQL)
tokio       → tokio (Async)
log         → log (Logging)
serde       → serde (Serialization)
```

Add more using LIBRARY_GUIDE.md

---

## 🎓 Learning Path

### Beginner (1 hour)
1. Read START_HERE_DOCS.md (5 min)
2. Read QUICKSTART.md (15 min)
3. Create first project (10 min)
4. Write simple .kr file (10 min)
5. Build and test (10 min)
6. Read relevant KRAIT_COMPLETE_DOCS.md sections (10 min)

### Developer (2-3 hours)
1. Read ARCHITECTURE.md (30 min)
2. Explore src/modules/ (30 min)
3. Read LIBRARY_GUIDE.md (20 min)
4. Try adding a custom library (30 min)
5. Read KRAIT_COMPLETE_DOCS.md (as reference)

---

## 📋 Type System

```krait
txt message = "Hello"       # String → Rust: String
num count = 42              # Integer → Rust: i64
bool is_active = true       # Boolean → Rust: bool
json data = {}              # JSON → Rust: serde_json::Value
```

---

## 🛡️ Visibility Model

```krait
public txt NAME = "Alice"       # pub const
private num AGE = 30            # const (no pub)

public func public_func()        # pub fn
    return "public"
end

private func private_func()      # fn (no pub)
    return "private"
end
```

---

## 🔄 Code Generation

**Krait** → **Lexer** → **Parser** → **CodeGen** → **Rust** → **Binary**

All generated Rust code is:
- ✅ Valid and type-safe
- ✅ Well-formatted
- ✅ Minimal (no unnecessary imports)
- ✅ Ready to modify if needed

---

## 📂 Project Structure

```
myapp/
├── main.kr              # Main code
├── models.kr            # Data models
├── handlers.kr          # Route handlers
├── utils.kr             # Utilities
├── Cargo.toml          # Rust dependencies (auto)
└── rust_code/          # Generated Rust
    ├── main.rs
    ├── models.rs
    ├── handlers.rs
    └── utils.rs
```

---

## ✅ What's Done

- [x] Language design
- [x] Compiler (lexer, parser, codegen)
- [x] Type system (txt, num, bool, json)
- [x] Functions (public/private)
- [x] Variables (public/private)
- [x] Control flow (if/else, while)
- [x] REST API support (@route)
- [x] Library management
- [x] CLI commands (init, build, run)
- [x] Comprehensive documentation

---

## 🔮 Future Plans

### v0.4.0
- Pattern matching
- For loops
- Structs and enums
- Error handling

### v0.5.0
- Database integration
- Async/await syntax
- Generics
- Traits

### v1.0.0
- Production-ready
- Full type system
- Stable API

---

## 📞 Support

**Documentation**:
- 🎯 START_HERE_DOCS.md - Navigation
- 🚀 QUICKSTART.md - Getting started
- 📖 KRAIT_COMPLETE_DOCS.md - Full reference
- 🏗️ ARCHITECTURE.md - How it works
- 📦 LIBRARY_GUIDE.md - Add libraries

**Issues**:
- Check DOCUMENTATION_INDEX_FINAL.md troubleshooting section
- Review generated Rust in rust_code/
- Read examples in KRAIT_COMPLETE_DOCS.md

---

## 🎯 Common Tasks

### Create REST API
```bash
krait init my_api
cat > main.kr << 'EOF'
import rest from actix

@route "/api/hello" GET
    return "Hello"
end
EOF
krait build && krait run
```

### Add Database
```bash
# 1. Edit Cargo.toml - add sqlx
# 2. Edit src/modules/codegen/libs.rs - register sqlx
# 3. Use in code: import sqlx from database
```

### Add Custom Library
See LIBRARY_GUIDE.md for step-by-step guide

---

## 🏆 Why Krait?

| vs Python | vs Rust |
|-----------|---------|
| ✅ Faster | ✅ Simpler syntax |
| ✅ Safer | ✅ Better errors |
| ✅ Type safe | ✅ Web-ready |
| ✅ No GIL | ✅ No borrow checker |

---

## 📊 Statistics

- **Documentation**: 2934 lines
- **Compiler**: ~2000 lines of Rust
- **Type System**: 4 core types
- **Built-in Libraries**: 7
- **CLI Commands**: 4
- **Version**: 0.3.0
- **Status**: Alpha (not production-ready)
- **License**: GPL 3.0

---

## 🚀 Get Started Now

1. **Explore**: Open [START_HERE_DOCS.md](START_HERE_DOCS.md)
2. **Learn**: Read [QUICKSTART.md](QUICKSTART.md)
3. **Create**: Run `krait init myapp`
4. **Code**: Write your `.kr` files
5. **Build**: Run `krait build`
6. **Reference**: Use [KRAIT_COMPLETE_DOCS.md](KRAIT_COMPLETE_DOCS.md)

---

## 📄 License

GNU General Public License v3.0

See [LICENSE](LICENSE) file for details.

---

## 🙏 Contributing

Krait welcomes contributions!

- **Bug Reports**: Open issues
- **Features**: Suggest in discussions
- **Code**: Submit pull requests
- **Docs**: Help improve documentation

See ARCHITECTURE.md for development guide.

---

**Krait v0.3.0**

Python-like syntax × Rust performance 🐍⚡

December 10, 2024
