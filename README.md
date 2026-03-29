# Rusty Runtime 🦀

A TypeScript-inspired language runtime built in Rust for a custom language called **RTS (Rusty TypeScript)**.

Rusty compiles RTS code into bytecode and executes it on a custom virtual machine, with an upcoming **WebAssembly (WASM) backend** for near-native performance — without relying on JavaScript engines like V8.

---

## ✨ Vision

Modern TypeScript execution looks like:

```text
TS → JS (tsc) → V8 (Node/Bun/Deno) → JIT → machine code
```

This introduces:

* heavy runtime overhead
* unpredictable JIT optimizations
* large memory footprint

Rusty explores an alternative:

```text
RTS → Bytecode → Lightweight VM
           ↘
            WASM (future)
```

👉 A lightweight, controllable execution model
👉 No dependency on V8 or Node.js
👉 Portable and embeddable runtime

---

## ⚙️ Current Architecture

Rusty currently uses a **bytecode interpreter** with a frame-based VM:

```
Source Code → Lexer → Parser → AST → Compiler → Program → VM
                                                   ↓
                                          main bytecode + function table
                                                   ↓
                                          Frame-based execution (call stack)
```

| Stage    | What it does                                          |
| -------- | ----------------------------------------------------- |
| Lexer    | Tokenizes raw source into tokens                      |
| Parser   | Builds an AST with proper precedence                  |
| Compiler | Emits stack-based bytecode, separates functions        |
| VM       | Executes bytecode using a value stack + call frames    |

---

## 🚀 Upcoming: WASM Backend

Rusty is evolving toward a dual execution model:

```text
RTS → AST → Bytecode → VM
           ↘
            WASM → near-native execution
```

The WASM backend will:

* ⚡ improve execution speed for compute-heavy workloads
* 📦 enable portable execution across environments
* 🎯 provide predictable performance (no JIT warmup)
* 🔌 allow embedding in browsers, servers, and edge runtimes

---

## ✨ Features

* 🧠 Custom lexer, parser, and AST
* ⚡ Bytecode compiler with separate function compilation
* 🧮 Frame-based stack virtual machine
* ➕ Arithmetic with precedence and parentheses
* 📝 String literals and concatenation (`"hello" + " world"`)
* ➖ Unary operators (`-x`, `-(2 + 3)`)
* 🔀 Comparison operators (`>`, `<`, `==`, `!=`) for numbers and strings
* 🔗 Logical operators (`&&`, `||`) with short-circuit evaluation
* 🔁 Control flow (`if` / `else`) with backpatching
* 🔄 Loops (`while`, `for`) with `break` / `continue`
* 🔢 Increment / decrement (`i++`, `i--`)
* 🔧 Functions with parameters, return values, and recursion
* 🧩 Nested function calls (`add(add(1,2), 3)`)
* 📦 Arrays (literals, index read/write, pass to functions)
* 🖨️ Print statements (`print x;`)
* 🔁 REPL (interactive shell)
* 📂 File execution (`rusty run file.rts`)
* ✅ CI pipeline (GitHub Actions)

---

## 🧪 Examples

Check out the [`example/`](example/) folder for sample RTS programs, including:

- [`test.rts`](example/test.rts) — array basics (literals, indexing, mutation)
- [`merge_sort.rts`](example/merge_sort.rts) — merge sort implementation using arrays, functions, and recursion

Run any example with:

```bash
cargo run -- run example/merge_sort.rts
```

---

## 🧰 Getting Started

### Install

```bash
git clone https://github.com/shubhiscoding/rusty_runtime.git
cd rusty_runtime
cargo install --path .
```

---

### Usage

```bash
rusty run file.rts
rusty repl
rusty --version
```

---

### Development

```bash
git clone https://github.com/shubhiscoding/rusty_runtime.git
cd rusty_runtime
cargo build
```

Run locally:

```bash
cargo run -- run example/test.rts
cargo run -- repl
```

---

## 🗺️ Roadmap

### Core Language

* [x] Variable declarations (`let`)
* [x] Arithmetic expressions with precedence
* [x] Unary operators
* [x] Parentheses
* [x] Print statement
* [x] REPL

---

### Language Features

* [x] Strings (literals, concatenation, mixed-type concat)
* [x] Truthy/falsy values (`0` and `""` are falsy)
* [x] Comparisons (`>`, `<`, `==`, `!=`) for numbers and strings
* [x] `if` / `else`
* [x] `while` loops
* [x] `break` / `continue`
* [x] Variable reassignment
* [x] `for` loops
* [x] Logical operators (`&&`, `||`) with short-circuiting
* [x] Increment / decrement (`++`, `--`)
* [x] Functions (declaration, params, return)
* [x] Nested / recursive function calls
* [x] Frame-based call stack
* [x] Arrays (literals, indexing, mutation)
* [x] Array pass/return from functions
* [ ] Type annotations
* [ ] Objects / maps
* [ ] Closures
* [ ] `<=`, `>=` operators

---

### Runtime & Tooling

* [x] Bytecode VM
* [x] CI pipeline (build, test, clippy, fmt)
* [ ] WASM backend (in progress)
* [ ] Bytecode optimizations
* [ ] Register-based VM (optional)

---

### Long-term Vision

* [ ] Compile TS-like syntax to WASM
* [ ] Optional native compilation (LLVM / Cranelift)
* [ ] Embeddable scripting runtime
* [ ] Standard library (`console`, `Math`, etc.)

---

## ⚠️ Current Status

Rusty is an experimental runtime and:

* does not yet match V8 performance
* does not support full TypeScript
* focuses on execution model exploration

---

## 🤝 Contributing

Contributions are welcome — especially in:

* language features
* error handling
* performance optimizations
* WASM backend

Steps:

1. Fork the repo
2. Create a branch
3. Raise a issue or Pick a issue
4. Submit a PR

---
