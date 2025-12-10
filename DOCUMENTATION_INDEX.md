# Krait Documentation Index

Welcome! Here's the complete documentation for the **Krait Programming Language**.

## 📚 Documentation Files

### 1. **README_KRAIT.md** - Overview & Features
- Project introduction
- Quick feature summary
- Example code snippets
- Building from source
- Troubleshooting

**Start here if you want:** A quick overview of what Krait is and what it can do.

---

### 2. **KRAIT_DOCUMENTATION.md** - Complete Reference (1044 lines)
The definitive guide covering:

#### Language Basics
- Syntax and conventions
- Comments and naming
- Hello World

#### Core Features
- Data types (int, txt, bool, json, etc.)
- Variables (public/private)
- Functions and return values
- Control flow (if/else, loops)
- Operators (arithmetic, logical, comparison)
- String operations
- Collections (arrays, maps)

#### Advanced Topics
- Module system & imports
- Web API development with `@route`
- Async/await
- Error handling (planned)
- Generics (planned)
- Traits (planned)

#### Practical Information
- CLI usage and commands
- Project structure
- Examples (CLI, API, data processing)
- Compilation process
- Troubleshooting guide

**Best for:** In-depth learning and reference

---

### 3. **QUICK_START_GUIDE.md** - Hands-On Tutorial
8 complete working examples:

1. **Math Functions** - Functions and variables
2. **Grade Calculator** - If/else statements
3. **Loops** - While and for loops
4. **Text Processing** - String operations
5. **CLI App** - Interactive applications
6. **REST API** - Web endpoints
7. **Arrays** - Working with collections
8. **Visibility** - Public/private functions

**Perfect for:** Learning by doing

**Start here if you want:** Working code examples

---

## 🎯 By Use Case

### "I'm new to Krait"
1. Read: **README_KRAIT.md** (5 min)
2. Try: **QUICK_START_GUIDE.md** (15 min)
3. Reference: **KRAIT_DOCUMENTATION.md** (as needed)

### "I need to build a Web API"
1. **KRAIT_DOCUMENTATION.md** → "Web API with Krait"
2. **QUICK_START_GUIDE.md** → "Example 6: REST API"
3. Build your project with `krait init`

### "I'm building a CLI tool"
1. **QUICK_START_GUIDE.md** → "Example 5: CLI App"
2. **KRAIT_DOCUMENTATION.md** → "Functions" & "Control Flow"
3. Use `krait init` to create your project

### "I want to understand the compiler"
1. **KRAIT_DOCUMENTATION.md** → "Compilation Process"
2. Look at `src/modules/` in the repository
3. Study the code generation in `codegen.rs`

---

## 📖 Quick Navigation

### Language Features

| Feature | Documentation |
|---------|---------------|
| Data Types | KRAIT_DOCUMENTATION.md § Data Types |
| Variables | KRAIT_DOCUMENTATION.md § Variables |
| Functions | KRAIT_DOCUMENTATION.md § Functions |
| If/Else | KRAIT_DOCUMENTATION.md § Control Flow |
| Loops | KRAIT_DOCUMENTATION.md § Control Flow |
| Strings | KRAIT_DOCUMENTATION.md § String Operations |
| Arrays | KRAIT_DOCUMENTATION.md § Collections |
| Maps | KRAIT_DOCUMENTATION.md § Collections |
| Web API | KRAIT_DOCUMENTATION.md § Web API |
| Modules | KRAIT_DOCUMENTATION.md § Modules & Imports |

### CLI Commands

| Command | Documentation |
|---------|---------------|
| `krait init` | README_KRAIT.md § CLI Commands |
| `krait build` | README_KRAIT.md § CLI Commands |
| `krait translate` | KRAIT_DOCUMENTATION.md § CLI Usage |

### Examples

| Type | Location |
|------|----------|
| Hello World | QUICK_START_GUIDE.md |
| Functions | QUICK_START_GUIDE.md § Example 1 |
| Control Flow | QUICK_START_GUIDE.md § Example 2-3 |
| Strings | QUICK_START_GUIDE.md § Example 4 |
| CLI | QUICK_START_GUIDE.md § Example 5 |
| Web API | QUICK_START_GUIDE.md § Example 6 |
| Arrays | QUICK_START_GUIDE.md § Example 7 |
| Visibility | QUICK_START_GUIDE.md § Example 8 |

---

## 🚀 Getting Started

### Step 1: Create a Project
```bash
krait init my_first_app
cd my_first_app
```

### Step 2: Write Code
Edit `krait_src/main.kr`:
```krait
public func main()
    print("Hello from Krait!")
end
```

### Step 3: Build & Run
```bash
krait build
./target/release/my_first_app
```

### Step 4: Learn More
- Read QUICK_START_GUIDE.md for examples
- Check KRAIT_DOCUMENTATION.md for deep dives

---

## 📋 File Structure

```
/
├── README_KRAIT.md               ← Start here!
├── KRAIT_DOCUMENTATION.md         ← Complete reference
├── QUICK_START_GUIDE.md           ← Working examples
├── DOCUMENTATION_INDEX.md         ← This file
├── src/                           ← Compiler source
│   ├── main.rs
│   ├── lib.rs
│   └── modules/
│       ├── lexer/                 ← Tokenization
│       ├── parser/                ← Syntax analysis
│       ├── codegen/               ← Code generation
│       ├── cli/                   ← CLI handling
│       └── api/                   ← API generation
└── test_api/                      ← Example project
    ├── krait_src/
    │   └── api.kr
    ├── rust_code/
    │   └── api.rs
    └── Cargo.toml
```

---

## 🔍 How Krait Works

```
Your Krait Code (.kr files)
    ↓
[Lexer]  → Tokenizes input
    ↓
[Parser] → Builds AST
    ↓
[Codegen] → Generates Rust code
    ↓
[Cargo] → Compiles with rustc
    ↓
[Your Binary] → Native executable
```

**More info:** KRAIT_DOCUMENTATION.md § Compilation Process

---

## 🎓 Learning Paths

### Path 1: Beginner (Start to Productive)
1. Read README_KRAIT.md
2. Try QUICK_START_GUIDE.md Examples 1-3
3. Create your first project: `krait init`
4. Reference KRAIT_DOCUMENTATION.md as needed

**Time:** ~1 hour

### Path 2: Web Developer (Build APIs)
1. Skim README_KRAIT.md
2. Read KRAIT_DOCUMENTATION.md § Web API
3. Study QUICK_START_GUIDE.md § Example 6
4. Build a real project with REST routes

**Time:** ~2 hours

### Path 3: Systems Programmer (Deep Dive)
1. Read KRAIT_DOCUMENTATION.md completely
2. Study src/modules/ code
3. Understand the codegen logic
4. Contribute improvements

**Time:** ~4+ hours

---

## ❓ FAQ

**Q: Where do I start?**  
A: Read README_KRAIT.md, then try QUICK_START_GUIDE.md

**Q: How do I build a web API?**  
A: See QUICK_START_GUIDE.md § Example 6 and KRAIT_DOCUMENTATION.md § Web API

**Q: How do I make something private?**  
A: Use `private func` or `private txt` keyword

**Q: What's the difference between `.kr` and `.krm`?**  
A: `.kr` is code, `.krm` is for organizing modules

**Q: How do I import other files?**  
A: Use `import name from ./path` (see KRAIT_DOCUMENTATION.md § Modules & Imports)

---

## 🔗 Quick Links

- **Main Repo**: https://github.com/yourusername/krait
- **Issues**: https://github.com/yourusername/krait/issues
- **Discussions**: https://github.com/yourusername/krait/discussions

---

## 📊 Documentation Stats

| File | Lines | Focus |
|------|-------|-------|
| README_KRAIT.md | ~180 | Overview & features |
| KRAIT_DOCUMENTATION.md | ~1044 | Complete reference |
| QUICK_START_GUIDE.md | ~350 | Examples & guides |
| DOCUMENTATION_INDEX.md | ~300 | Navigation (this file) |

**Total:** 1870+ lines of comprehensive documentation

---

## 🎯 Version Info

- **Krait Version:** 0.2.0
- **Status:** Active Development
- **Last Updated:** 2025-12-10
- **Compiler:** Krait → Rust → LLVM → Binary

---

## 💡 Pro Tips

1. **Type inference**: Krait infers most types automatically
2. **String concat**: Use `+` operator for strings
3. **Functions first**: Define helper functions before main
4. **Private helpers**: Use `private` for internal functions
5. **Web routes**: `@route` decorator makes REST APIs easy

---

**Made with ❤️ for developers who want fast, safe, and beautiful code**

Start with README_KRAIT.md and enjoy! 🚀
