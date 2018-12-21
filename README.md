# rust-playground

learn [Rust](https://www.rust-lang.org/) progamming language

## Development

```sh
# run on change (ensure `nodemon` is installed)
# turn off dead_code warning
nodemon --exec "RUSTFLAGS=\"$RUSTFLAGS -A dead_code\" cargo run" src/main.rs
```