# rust-playground

learn [Rust](https://www.rust-lang.org/) programming language

## Prerequisites

* [nodemon](https://nodemon.io/) - enables compile and run on file change

## Development

```sh
# run on file(s) change (ensure `nodemon` is installed)
# turn off dead_code warning
nodemon --exec "RUSTFLAGS=\"$RUSTFLAGS -A dead_code\" cargo run" src/main.rs

# watch tests (run `cargo test` when any file changes)
cargo watch -x check -x test

# to get a stack backtrace on panic
RUST_BACKTRACE=1 cargo run

```

---

## Notes

* traits are like interfaces in other languages

---

## `Option` and `Result`

see [Option and Result | Learning Rust](https://learning-rust.github.io/docs/e3.option_and_result.html)

### `Option`


```rust
// An output can have either Some value or no value/ None.
enum Option<T> { // T is a generic and it can contain any type of value.
    Some(T),
    None,
}
```

### `Result`

* If a function can produce an error, we have to use a Result
* used for all I/O, conversions, etc. anything that can fail.

```rust
// A result can represent either success/ Ok or failure/ Err.
enum Result<T, E> { // T and E are generics. T can contain any type of value, E can be any error.
    Ok(T),
    Err(E),
}
```

## Resources

* [The Rust Programming Language book](https://doc.rust-lang.org/stable/book/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [rust-analyzer](https://rust-analyzer.github.io/) - *Bringing a great IDE experience to the Rust programming language.*
* [GitHub - watchexec/cargo-watch: Watches over your Cargo project&#39;s source.](https://github.com/watchexec/cargo-watch)
