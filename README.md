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
```

## Resources

* [The Rust Programming Language book](https://doc.rust-lang.org/stable/book/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [GitHub - watchexec/cargo-watch: Watches over your Cargo project&#39;s source.](https://github.com/watchexec/cargo-watch)
