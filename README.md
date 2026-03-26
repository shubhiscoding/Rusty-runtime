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

Rusty currently uses a **bytecode interpreter**:

```
Source Code → Lexer → Parser → AST → Compiler → Bytecode → VM
```

| Stage    | What it does                                     |
| -------- | ------------------------------------------------ |
| Lexer    | Tokenizes raw source into tokens                 |
| Parser   | Builds an AST with proper precedence             |
| Compiler | Emits stack-based bytecode                       |
| VM       | Executes bytecode using a stack + variable store |

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
* ⚡ Bytecode compiler
* 🧮 Stack-based virtual machine
* ➕ Arithmetic with precedence and parentheses
* 📝 String literals and concatenation (`"hello" + " world"`)
* ➖ Unary operators (`-x`, `-(2 + 3)`)
* 🔀 Comparison operators (`>`, `<`, `==`, `!=`) for numbers and strings
* 🔗 Logical operators (`&&`, `||`) with short-circuit evaluation
* 🔁 Control flow (`if` / `else`) with backpatching
* 🔄 Loops (`while`, `for`) with `break` / `continue`
* 🔢 Increment / decrement (`i++`, `i--`)
* 🖨️ Print statements (`print x;`)
* 🔁 REPL (interactive shell)
* 📂 File execution (`rusty run file.rts`)

---

## 🧪 Example

```ts
let x = (2 + (3 * (4 + 2))) / 2;
x++;

if (x > 10) {
    print "x is greater than 10";
} else {
    print "x is small";
}

print "10" + "10";

while (x > 5) {
    if (x == 8) {
        x = x - 1;
        continue;
    } else {
        if (x > 0 && 10 > 8) {
            print x;
        }
    }
    x--;
}

for (let i = 0; i < x; i++) {
    if (i == 1) {
        continue;
    }
    if (i == 2) {
        break;
    }
    print i;
}

let n = "nope" + x;
print n;

if (n) {
    print "Works";
}
```

Output:

```
x is greater than 10
1010
11
10
9
7
6
0
nope5
Works
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
* [ ] Functions
* [ ] Type annotations
* [ ] Arrays and objects

---

### Runtime Evolution

* [x] Bytecode VM
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
