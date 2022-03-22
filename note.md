https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

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