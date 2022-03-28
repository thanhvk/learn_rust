https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

https://app.buildspace.so/projects/CObd6d35ce-3394-4bd8-977e-cbee82ae07a3/lessons/LE65582090-a043-48db-b66d-3fc56cd058ae (program send sol)

https://soldev.app/?utm_source=solana.com

https://kristohb.medium.com/mint-tokens-on-solana-using-the-rust-sdk-3b05b07ca842

https://kristohb.medium.com/transfer-tokens-between-accounts-on-solana-f07c0fdb567c

https://blog.matanwrites.com/your-first-solana-program

```
// compiler
rustc --version
rustup update
rustup doc
rustc main.rs
```

```
// cargo
cargo new hello_cargo
cd hello_cargo
cargo run
cargo build
```

```
Cargo.toml = package.json
Cargo.lock = yarn.lock
crates = packages
https://crates.io/ = https://www.npmjs.com/
cargo doc --open
```

match expression

```
match guess.cmp(&secret_number) {
  Ordering::Less => println!("Too small!"),
  Ordering::Greater => println!("Too big!"),
  Ordering::Equal => println!("You win!"),
}
```

ownership?

traits? = class/interface defined?

stack vs heap?

Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.

An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, chances are you should use a vector.

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

Rust is an expression-based language, this is an important distinction to understand

Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

Ownership is a set of rules that governs how a Rust program manages memory.

The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.

