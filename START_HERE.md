# 🎯 START HERE - Krait v0.2.0

Welcome! This is your starting point for the **Krait Programming Language**.

> **Krait** = Python-like syntax + Rust safety + web API support

---

## ⚡ 5-Minute Quick Start

### 1️⃣ Create a new project
```bash
krait init my_first_app
cd my_first_app
```

### 2️⃣ Edit `krait_src/main.kr`
```krait
public func main()
    print("Hello from Krait!")
end
```

### 3️⃣ Build & run
```bash
krait build
./target/release/my_first_app
```

**Done! 🎉**

---

## 📚 Documentation Guide

### Choose your path:

#### 👶 **I'm completely new**
1. Read: **README_KRAIT.md** (5 min)
2. Read: **QUICK_START_GUIDE.md** (15 min)
3. Try the examples!

#### 💻 **I know programming**
1. Skim: **README_KRAIT.md** (3 min)
2. Jump to: **KRAIT_DOCUMENTATION.md** § features you need
3. Run examples from **QUICK_START_GUIDE.md**

#### 🔬 **I want to understand how it works**
1. Read: **DOCUMENTATION_INDEX.md**
2. Study: **KRAIT_DOCUMENTATION.md** § Compilation Process
3. Check: `src/modules/` in the repository

#### 🚀 **I want to build a web API**
1. Read: **QUICK_START_GUIDE.md** § Example 6 (REST API)
2. Reference: **KRAIT_DOCUMENTATION.md** § Web API with Krait
3. Start building with `krait init`

---

## 📖 Key Documentation Files

| File | What's in it | Time |
|------|-------------|------|
| **README_KRAIT.md** | Overview, features, basics | 5 min |
| **QUICK_START_GUIDE.md** | 8 working examples you can run | 15 min |
| **KRAIT_DOCUMENTATION.md** | Complete reference (all features) | 30+ min |
| **DOCUMENTATION_INDEX.md** | Map of all docs + learning paths | 10 min |
| **RELEASE_SUMMARY.md** | What's new in v0.2.0 | 5 min |

---

## 🎯 Learning Paths

### Path A: "Just Get Started" (1 hour)
```
README_KRAIT.md (overview)
    ↓
QUICK_START_GUIDE.md examples 1-3 (hands-on)
    ↓
krait init myapp (your first project)
    ↓
Reference KRAIT_DOCUMENTATION.md as needed
```

### Path B: "Deep Dive" (3 hours)
```
README_KRAIT.md (overview)
    ↓
KRAIT_DOCUMENTATION.md (everything)
    ↓
QUICK_START_GUIDE.md (all examples)
    ↓
Build multiple projects
```

### Path C: "Just Build an API" (30 min)
```
README_KRAIT.md § Features
    ↓
QUICK_START_GUIDE.md § Example 6
    ↓
KRAIT_DOCUMENTATION.md § Web API
    ↓
krait init myapi (start coding)
```

---

## 💡 Examples at a Glance

### Example 1: Hello World
```krait
public func main()
    print("Hello, World!")
end
```

### Example 2: Functions
```krait
public func add(int a, int b) -> int
    return a + b
end
```

### Example 3: Control Flow
```krait
public func check(int x)
    if x > 10
        print("Big number")
    else
        print("Small number")
    end
end
```

### Example 4: REST API
```krait
import api from rest

@route "/hello" GET
public func hello() -> json
    return {"message": "Hello!"}
end
```

**More examples in:** QUICK_START_GUIDE.md

---

## 🚀 CLI Commands You Need

```bash
krait init <app>                    # Create new project
krait <input.kr> <output.rs>        # Translate one file
krait build                         # Build your project
krait --help                        # Show help
krait --version                     # Show version
```

---

## ❓ Quick FAQ

**Q: How do I create a project?**  
A: `krait init myapp`

**Q: How do I make something private?**  
A: Use `private func` or `private txt name`

**Q: How do I build a web API?**  
A: Use `@route "/path" GET` decorator (see QUICK_START_GUIDE.md)

**Q: Can I use existing Rust code?**  
A: Yes! The generated Rust can import any crate

**Q: Where's the full documentation?**  
A: See DOCUMENTATION_INDEX.md for all docs

**Q: How do I import other files?**  
A: `import name from ./path` or `import name from rest`

---

## 🎯 What Can Krait Do?

✅ CLI applications  
✅ REST APIs (web servers)  
✅ System utilities  
✅ Data processing  
✅ Learn Rust safely  

---

## 📊 Project Structure

When you run `krait init myapp`:

```
myapp/
├── krait_src/          ← Your Krait code here (.kr files)
│   └── main.kr
├── rust_code/          ← Auto-generated Rust code
│   └── main.rs
├── Cargo.toml          ← Rust configuration
├── target/             ← Compiled binaries
└── README.md
```

---

## 🔥 Key Features (v0.2.0)

- 🎯 **Simple syntax**: Python-like, easy to read
- ⚡ **Fast**: Compiles to native Rust/LLVM code
- 🔒 **Safe**: Memory safety guaranteed
- 🌐 **Web-ready**: Built-in REST API support
- 📦 **Modular**: Import system included
- 📚 **Documented**: 1870+ lines of docs

---

## 🛠️ How It Works

```
Your .kr code
    ↓
[Lexer]   → Tokenizes
    ↓
[Parser]  → Builds AST
    ↓
[Codegen] → Generates Rust
    ↓
[Cargo]   → Compiles
    ↓
Native executable
```

---

## 📞 Need Help?

1. **Getting started?** → Read README_KRAIT.md
2. **Want examples?** → Check QUICK_START_GUIDE.md
3. **Need reference?** → KRAIT_DOCUMENTATION.md
4. **Lost?** → DOCUMENTATION_INDEX.md (navigation)
5. **Want all info?** → See docs/ folder

---

## ✅ Next Steps

Choose one:

### If you're impatient 👇
```bash
krait init hello
cd hello
krait build
./target/release/hello
```

### If you want to learn 👇
Read: **README_KRAIT.md** then **QUICK_START_GUIDE.md**

### If you want everything 👇
Read: **DOCUMENTATION_INDEX.md** (it tells you what to read)

---

## 🎊 Let's Go!

**Pick a path above and start coding! 🐍**

You're just 5 minutes away from your first Krait program.

---

**Questions?** Check the documentation files.  
**Ready?** Run `krait init` now!

**Happy coding! 🚀**

