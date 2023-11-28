# rust-url-wasm-demo

This is a small demo app written using `yew` to test `url`'s binary size for
https://github.com/servo/rust-url/pull/887

Building and running requires [a Rust toolchain for `wasm32-unknown-unknown`, and `trunk`][0].

[0]: https://yew.rs/docs/getting-started/introduction

## Build

```sh
# In debug mode:
trunk build

# In release mode:
trunk build --release
```

Artefacts will be in `dist/`.

## Run

```sh
# In debug mode:
trunk serve

# In release mode:
trunk serve --release
```
