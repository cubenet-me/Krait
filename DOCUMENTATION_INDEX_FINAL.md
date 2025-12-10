# Krait Documentation Index

## 📚 Complete Documentation Structure

### 🎯 Start Here

1. **QUICKSTART.md** ⭐
   - First steps with Krait
   - 5-minute introduction
   - Basic examples
   - **Best for**: New users

2. **KRAIT_COMPLETE_DOCS.md** 📖
   - Full language reference
   - All syntax and features
   - Complete CLI reference
   - **Best for**: Comprehensive learning

### 🏗️ Architecture & Design

3. **ARCHITECTURE.md**
   - Compiler structure
   - Module system
   - Code generation pipeline
   - Data flow
   - **Best for**: Understanding how Krait works

4. **REFACTOR_SUMMARY.md**
   - Recent improvements
   - Modular structure
   - What was changed
   - **Best for**: Project overview

### 📦 Libraries & Dependencies

5. **LIBRARY_GUIDE.md**
   - How to add new libraries
   - Library registry system
   - Cargo integration
   - Examples
   - **Best for**: Extending Krait

### 📋 Reference Files

- **LICENSE** - GPL 3.0 license
- **.gitignore** - Version control excludes
- **Cargo.toml** - Rust dependencies

---

## File Organization

```
krait/
├── KRAIT_COMPLETE_DOCS.md      ← Full reference (START HERE)
├── QUICKSTART.md               ← Quick introduction
├── ARCHITECTURE.md             ← How it works internally
├── LIBRARY_GUIDE.md            ← Adding new libraries
├── REFACTOR_SUMMARY.md         ← Recent changes
├── DOCUMENTATION_INDEX_FINAL.md ← This file
├── LICENSE                     ← GPL 3.0
├── .gitignore                  ← Git rules
├── Cargo.toml                  ← Dependencies
├── src/
│   ├── main.rs                 ← CLI entry point
│   ├── lib.rs                  ← Library exports
│   └── modules/
│       ├── cli/mod.rs          ← Command handling
│       ├── api/mod.rs          ← HTTP support
│       └── codegen/
│           ├── lexer.rs        ← Tokenization
│           ├── parser.rs       ← AST building
│           ├── gen.rs          ← Code generation
│           ├── libs.rs         ← Library registry
│           └── backends/
│               └── rust.rs     ← Rust code generator
└── target/                     ← Compiled output
```

---

## Reading Path by Role

### 👨‍💻 For New Users

1. Read **QUICKSTART.md** (15 min)
2. Try first example
3. Look at **KRAIT_COMPLETE_DOCS.md** sections you need
4. Experiment with `.kr` files

### 🏗️ For Contributors

1. Read **ARCHITECTURE.md** (30 min)
2. Understand the module structure
3. Review **LIBRARY_GUIDE.md** for extending
4. Look at source code in `src/modules/`

### 📚 For Reference

- **Syntax**: KRAIT_COMPLETE_DOCS.md → Language Syntax
- **Functions**: KRAIT_COMPLETE_DOCS.md → Functions
- **REST API**: KRAIT_COMPLETE_DOCS.md → REST API
- **Libraries**: LIBRARY_GUIDE.md
- **Commands**: KRAIT_COMPLETE_DOCS.md → CLI Reference
- **Architecture**: ARCHITECTURE.md

---

## Quick Links

### Syntax Quick Reference

```krait
# Comments
# Single line comment

# Function
public func hello(name txt) -> txt
    return "Hello, " + name
end

# Variables
public txt greeting = "Hi"
private num count = 0

# Control flow
if x > 10
    return "big"
else
    return "small"
end

# REST API
@route "/api/hello" GET
    return "Hello"
end

# Import
import rest from actix
import json from serde
```

### Common Tasks

**Create a project**
```bash
krait init myapp
cd myapp
```

**Write code**
```bash
vim main.kr
```

**Compile and run**
```bash
krait build
krait run
```

**Add a library**
1. Edit `Cargo.toml`
2. Register in `src/modules/codegen/libs.rs`
3. Import in `.kr` file

---

## Features Overview

### ✅ Implemented

- Public and private functions
- Variable declarations
- Basic types (txt, num, bool, json)
- Control flow (if/else, while)
- REST API routes (@route decorator)
- Library imports
- Code generation to Rust
- CLI commands (init, build, run)

### 🔄 In Progress

- Structs and data models
- Error handling
- Advanced type system

### 📅 Planned

- Pattern matching
- Async/await syntax
- Generics
- Database integration
- For loops
- Traits

---

## Important Concepts

### Krait vs Rust

| Concept | Krait | Rust |
|---------|-------|------|
| Variables | `txt name = "Alice"` | `let name: String = ...` |
| Functions | `public func hello()` | `pub fn hello()` |
| Visibility | `public`/`private` | `pub`/private |
| Return | `return "value"` | `"value"` or `return` |
| Types | `txt`, `num`, `bool`, `json` | String, i64, bool, Value |

### Type Mapping

```
Krait txt    → Rust String
Krait num    → Rust i64
Krait bool   → Rust bool
Krait json   → Rust serde_json::Value
```

### Visibility Model

```krait
public txt PUBLIC_VAR = "visible"     # pub const
private num PRIVATE_VAR = 42          # const (no pub)

public func public_func()              # pub fn
    return "public"
end

private func private_func()            # fn (no pub)
    return "private"
end
```

---

## Code Generation Process

```
main.kr (Krait source)
    ↓
Lexer (tokenize)
    ↓
Parser (build AST)
    ↓
CodeGen (generate Rust)
    ↓
rust_code/main.rs (Rust source)
    ↓
cargo build (compile)
    ↓
Binary (executable)
```

Each stage is independent and can be extended.

---

## Getting Help

1. **Syntax questions** → KRAIT_COMPLETE_DOCS.md
2. **How to add features** → ARCHITECTURE.md
3. **Adding libraries** → LIBRARY_GUIDE.md
4. **Getting started** → QUICKSTART.md
5. **Project structure** → REFACTOR_SUMMARY.md

---

## Project Statistics

- **Language**: Rust
- **License**: GPL 3.0
- **Current Version**: 0.3.0
- **Status**: Alpha (not production-ready)
- **Documentation**: 949 lines (KRAIT_COMPLETE_DOCS.md)
- **Compiler**: ~2000 lines of Rust

---

## Key Files

### For Understanding

- `src/modules/codegen/lexer.rs` - How text becomes tokens
- `src/modules/codegen/parser.rs` - How tokens become AST
- `src/modules/codegen/gen.rs` - How AST becomes Rust
- `src/modules/codegen/libs.rs` - Library management

### For Using

- `src/main.rs` - CLI entry point
- `src/modules/cli/mod.rs` - CLI commands
- KRAIT_COMPLETE_DOCS.md - Language reference

### For Contributing

- ARCHITECTURE.md - System design
- LIBRARY_GUIDE.md - Extension guide
- Tests in each module

---

## Workflow

### Development Cycle

1. Write `.kr` files
2. Run `krait build`
3. Check generated Rust in `rust_code/`
4. Run `krait run`
5. Debug if needed

### Adding Features

1. Understand requirement
2. Add tokens to lexer
3. Add parsing rules to parser
4. Implement code generation
5. Test with examples
6. Update documentation

---

## Conventions

### Naming

- Functions: `snake_case`
- Variables: `snake_case`
- Types: `PascalCase`
- Constants: `UPPER_CASE`

### Code Style

- Indentation: 4 spaces
- Line length: max 100 chars
- Comments: start with `#`
- Blocks: end with `end`

### Project Names

- Use lowercase
- Use underscores for spacing
- Examples: `my_app`, `hello_world`, `api_server`

---

## Troubleshooting

### Build fails
→ Check `rust_code/main.rs` for errors
→ Ensure all imports are registered in `libs.rs`

### Function not found
→ Check function is public with `public func`
→ Import the module correctly

### Type mismatch
→ Check type conversions in KRAIT_COMPLETE_DOCS.md
→ Use `.to_string()` or `.to_num()` to convert

### Route not working
→ Ensure `import rest from actix`
→ Check HTTP method is correct (GET, POST, PUT, DELETE)
→ Verify path syntax

---

## Performance Notes

- Krait compiles to native Rust
- No runtime overhead
- Same performance as hand-written Rust
- Actix-web is battle-tested

---

## Next Steps

1. **Learn**: Read QUICKSTART.md
2. **Try**: Create a simple project
3. **Explore**: Read KRAIT_COMPLETE_DOCS.md
4. **Extend**: Try adding a library (LIBRARY_GUIDE.md)
5. **Contribute**: Review ARCHITECTURE.md

---

## Version History

- **v0.3.0** (Current) - Modular architecture, Library management
- **v0.2.0** - REST API support
- **v0.1.0** - Basic functions and variables

---

**Happy coding with Krait! 🐍⚡**

For the latest updates, check the root directory for new documentation files.

Generated: December 10, 2024
