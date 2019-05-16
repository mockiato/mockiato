# Changelog

## 0.2.0
- Mocks now have a lifetime (`'mock`) which allows mocks to contain
  non-static references.
- To revert to the old behaviour (only static references) there is a new attribute for that:
  `#[mockable(static_references)]`.

## 0.3.0
- `ExpectedCalls::any()` has been removed. Use the range `0..` instead.
- Support for nearly equals expectations has been added. (`nearly_eq` and `nearly_eq_with_accuracy`)

## 0.4.0
- Ordered expectations are now available using `mock.expect_<method_name>_calls_in_order()`

## 0.4.1
- The `Debug` implementation of an argument's type is now correctly used when printing the expected calls for a method.
- Internal types now have separate `Debug` and `Display` implementations.
- Some unused exports have been removed from `mockiato::internal`. This is treated as a non-breaking change as no consumer should directly depend on these types.

## 0.4.2
- Fix error with `parse_quote!` macro when patch version of syn is too low in consumers `Cargo.lock`. 

## 0.5.0
- Generic type parameters on traits are now supported. Parameters with references that contain generic type parameters are explicitly disallowed.
- Full ranges (`..`) can be passed to `.times()`

## 0.5.1
- The `Debug` impl for mocks of traits with generic type params no longer require that the generic types implements `Debug`.

## 0.6.0
- **Breaking:** The `expect_*` methods now require passing a closure for each argument that receives a factory to create argument matchers. This has been changed so that crates using mocks from another crate no longer need to depend on the same version of mockiato.
- **Breaking:** It is now a hard error when `name` is specified more than once in the `#[mockable]` attribute.
- The `Display` implementation of some argument matchers has been improved.
- A lot of documentation has been added.
