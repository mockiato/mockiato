# Mockiato

[![Build Status](https://travis-ci.com/myelin-ai/mockiato.svg?branch=master)](https://travis-ci.com/myelin-ai/mockiato)
[![Latest Version](https://img.shields.io/crates/v/mockiato.svg)](https://crates.io/crates/mockiato)
[![Documentation](https://docs.rs/mockiato/badge.svg)](https://docs.rs/mockiato)
[![dependency status](https://deps.rs/repo/github/myelin-ai/mockiato/status.svg)](https://deps.rs/repo/github/myelin-ai/mockiato)

A strict, yet friendly mocking library for Rust 2018

 > ⚠️ This crate requires the nightly compiler

## Quickstart

```rust
#[cfg(test)]
use mockiato::mockable;

#[cfg_attr(test, mockable)]
trait Greeter {
    fn greet(&self, name: &str) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_the_world() {
        let mut greeter = GreeterMock::new();

        greeter
            .expect_greet(|arg| arg.partial_eq("world"))
            .times(1..2)
            .returns(String::from("Hello world"));

        assert_eq!("Hello world", greeter.greet("world"));
    }
}
```

## Trait Bounds

Trait bounds are currently not supported meaning that the supertraits will not be implemented for mocks.

The following traits are always implemented for mocks:

- [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)  
  Example: [`cargo run --example debug`](./examples/debug.rs)
- [Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)  
  Example: [`cargo test --example clone`](./examples/clone.rs)
- [Default](https://doc.rust-lang.org/std/default/trait.Default.html)  
  Example: [`cargo test --example default`](./examples/default.rs)

## Downcasting

An example of how to use downcasting with mockiato can be found in the [`downcasting`](./examples/downcasting.rs) example.

## Contributing

### Enable debug impls in codegen

```bash
cargo test --features mockiato-codegen/debug-impls
```
