# Trait Bounds

Trait bounds are currently not supported meaning that the supertraits will not be implemented for mocks.

The following traits are always implemented for mocks:

- [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)  
  Example: `cargo run --example debug`
- [Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)  
  Example: `cargo test --example clone` 

