# isclose &emsp; [![Test Status]][actions]&thinsp;[![Crate Version]][crates]&thinsp;[![Rust Version]][crates]

[test status]: https://img.shields.io/github/actions/workflow/status/staticintlucas/isclose/test.yml?branch=main&label=tests&style=flat-square
[crate version]: https://img.shields.io/crates/v/isclose?style=flat-square
[rust version]: https://img.shields.io/crates/msrv/isclose?style=flat-square

[actions]: https://github.com/staticintlucas/isclose/actions?query=branch%3Amain
[crates]: https://crates.io/crates/isclose

<!-- cargo-rdme start -->

This crate provides a set of traits and macros for comparing arbitrary types.

The trait `IsClose` is implemented by default for `f32` and `f64`.

Additional implementations are also hidden behind the following features:

- `half` implements `IsClose` for [half]'s `f16` and `bf16`
- `euclid` implements `IsClose` for [euclid]'s geometric types

[half]: https://crates.io/crates/half
[euclid]: https://crates.io/crates/euclid

### Usage:

```rust
use isclose::{IsClose, assert_is_close};

// This will fail!
// assert_eq!(0.1 + 0.2, 0.3)

// This will pass
assert!((0.1 + 0.2).is_close(0.3));

// Equivalent, but gives better error messages
assert_is_close!(0.1 + 0.2, 0.3);
```

You can also implement `IsClose` for custom types

```rust
use isclose::{IsClose, assert_is_close};
use std::borrow::Borrow;

#[derive(Debug)]
struct Vector { x: f32, y: f32 }

impl IsClose<f32> for Vector {
    // Use the same default tolerances as f32
    // You can override the defaults here if necessary
    const ABS_TOL: f32 = <f32 as IsClose>::ABS_TOL;
    const REL_TOL: f32 = <f32 as IsClose>::REL_TOL;

    // The is_close_impl function is the only one that must be implemented
    // to implement IsClose. The other functions will delegate to this one.
    fn is_close_impl(
        &self,
        other: &Self,
        rel_tol: &f32,
        abs_tol: &f32,
    ) -> bool {
        self.x.is_close_impl(&other.x, rel_tol, abs_tol) &&
            self.y.is_close_impl(&other.y, rel_tol, abs_tol)
    }
}

assert_is_close!(
    Vector {
        x: 0.1 + 0.2,
        y: 0.2 + 0.4,
    },
    Vector {
        x: 0.3,
        y: 0.6,
    }
)
```

<!-- cargo-rdme end -->

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
