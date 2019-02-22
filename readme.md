# Mockiato

[![Build Status](https://travis-ci.com/myelin-ai/mockiato.svg?branch=master)](https://travis-ci.com/myelin-ai/mockiato)
[![Latest Version](https://img.shields.io/crates/v/mockiato.svg)](https://crates.io/crates/mockiato)
[![Documentation](https://docs.rs/mockiato/badge.svg)](https://docs.rs/mockiato)


Minimalistic mocking framework, ready for Rust 2018! 🎉

## Quickstart

```rust
#[cfg(test)]
use mockiato::*;

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
            .expect_greet(partial_eq("world"))
            .returns(String::from("Hello world"));

        assert_eq!("Hello world", greeter.greet("world"));
    }

}
```

## Trait Bounds

Trait bounds are currently not supported meaning that the supertraits will not be implemented for mocks.

The following traits are always implemented for mocks:

- [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)  
  Example: `cargo run --example debug`
- [Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)  
  Example: `cargo test --example clone` 
- [Default](https://doc.rust-lang.org/std/default/trait.Default.html)  
  Example: `cargo test --example default` 

## Contributing

### Enable debug impls in codegen

```bash
cargo test --features mockiato-codegen/debug-impls
```
