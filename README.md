# isclose &emsp; [![Test Status]][actions]&thinsp;<!--[![Test Coverage]][codecov]&thinsp;--><!--[![Crate Version]][crates]&thinsp;-->[![Rust Version]][crates]

[test status]: https://img.shields.io/github/actions/workflow/status/staticintlucas/isclose/test.yml?branch=main&label=tests&style=flat-square
<!-- [test coverage]: https://img.shields.io/codecov/c/gh/staticintlucas/isclose?style=flat-square -->
<!-- [crate version]: https://img.shields.io/crates/v/isclose?style=flat-square -->
[rust version]: https://img.shields.io/badge/rust-1.46%2B-informational?style=flat-square

[actions]: https://github.com/staticintlucas/keyset-rs/actions?query=branch%3Amain
<!-- [codecov]: https://app.codecov.io/github/staticintlucas/keyset-rs -->
<!-- [crates]: https://crates.io/crates/isclose -->

This crate provides a set of traits and macros for comparing arbitrary
types.

The trait [`IsClose`] is implemented by default for [`f32`] and [`f64`].

Additional implementations are also hidden behind the following features:

- `half` implements [`IsClose`] for [`half::f16`] and [`half::bf16`]
- `euclid` implements [`IsClose`] for euclid's geometric types

## Usage:

```rust
// This will fail!
// assert_eq!(0.1 + 0.2, 0.3)

// This will pass
assert!((0.1 + 0.2).is_close(0.3));

// Equivalent, but gives better error messages
assert_is_close!(0.1 + 0.2, 0.3);
```

## Licence

Licensed under either of

* Apache License, Version 2.0 ([LICENCE-APACHE](LICENCE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0][apache-licence])
* MIT license ([LICENCE-MIT](LICENCE-MIT) or [http://opensource.org/licenses/MIT][mit-licence])

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

[apache-licence]: http://www.apache.org/licenses/LICENSE-2.0
[mit-licence]: http://opensource.org/licenses/MIT
