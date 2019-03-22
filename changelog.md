# Changelog

## 0.2.0
- Mocks now have a lifetime (`'mock`) which allows mocks to contain
  non-static references.
- To revert to the old behaviour (only static references) there is a new attribute for that:
  `#[mockable(static_references)]`.

## TBD
- `ExpectedCalls::any()` has been removed. Use the range `0..` instead.
- Support for nearly equals expectations has been added. (`nearly_eq` and `nearly_eq_with_accuracy`)
