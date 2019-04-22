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
