# Chapter 2

## Dynamic Dispatch VS Static Dispatch

### Overview

#### Dyn compatibility


https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

### Benchmark

```sh
cargo bench -p chapter_02
```

https://bheisler.github.io/criterion.rs/book/getting_started.html

### Inspect Assembly

```sh
cargo rustc -p chapter_02_bin_dynamic --release -- --emit asm -C 'llvm-args=-x86-asm-syntax=intel'
```

```sh
cargo rustc -p chapter_02_bin_static --release -- --emit asm -C 'llvm-args=-x86-asm-syntax=intel'
```

emit options

- mir (Rust intermediate representation)
- llvm-ir (LLVM intermediate representation)
- llvm-bc (LLVM byte code)
- asm (assembly)
