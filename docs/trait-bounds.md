# Trait Bounds

Trait bounds are currently not supported.

The only exception are traits from `std` that can be automatically derived.
Mockiato automatically adds the correct `#[derive]` attribute to the generated mock.
A list of derivable traits can be found in the [Rust Book](https://doc.rust-lang.org/book/2018-edition/appendix-03-derivable-traits.html).

## Example

An example can be found in [`examples/auto-derive.rs`](../mockiato/examples/auto-derive.rs)

It can be run using: `cargo run --example auto-derive`
