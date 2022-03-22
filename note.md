https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
https://app.buildspace.so/projects/CObd6d35ce-3394-4bd8-977e-cbee82ae07a3/lessons/LE65582090-a043-48db-b66d-3fc56cd058ae (program send sol)

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