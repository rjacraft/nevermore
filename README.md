# nevermore

[![crates.io version](https://img.shields.io/crates/v/nevermore?style=flat-square)][crates.io]
[![crates.io downloads](https://img.shields.io/crates/d/nevermore?style=flat-square)][crates.io]
[![docs.rs](https://img.shields.io/docsrs/nevermore?style=flat-square)][docs.rs]

Derive macros for Rust's bottom type [core::convert::Infallible][Infallible].

## Features

### `FromNever`

This derive macro automatically generates `From<Infallible>` implementation on the type:

```rust
#[derive(nevermore::FromNever)]
struct User {
    name: String,
    age: u8,
}
```

will generate something similar to

```rust
impl From<Infallible> for User {
    fn from(infallible: Infallible) {
        match infallible {}
    }
}
```

[crates.io]: https://crates.io/crates/nevermore
[docs.rs]: https://docs.rs/nevermore
[Infallible]: https://doc.rust-lang.org/stable/core/convert/enum.Infallible.html
