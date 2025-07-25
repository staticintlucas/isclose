//! This crate provides a set of traits and macros for comparing arbitrary types.
//!
//! The trait `IsClose` is implemented by default for `f32` and `f64`.
//!
//! Additional implementations are also hidden behind the following features:
//!
//! - `half` implements `IsClose` for [half]'s `f16` and `bf16`
//! - `euclid` implements `IsClose` for [euclid]'s geometric types
//!
//! [half]: https://crates.io/crates/half
//! [euclid]: https://crates.io/crates/euclid
//!
//! ## Usage:
//!
//! ```
//! use isclose::{IsClose, assert_is_close};
//!
//! // This will fail!
//! // assert_eq!(0.1 + 0.2, 0.3)
//!
//! // This will pass
//! assert!((0.1 + 0.2).is_close(0.3));
//!
//! // Equivalent, but gives better error messages
//! assert_is_close!(0.1 + 0.2, 0.3);
//! ```
//!
//! You can also implement `IsClose` for custom types
//!
//! ```
//! use isclose::{IsClose, assert_is_close};
//! use std::borrow::Borrow;
//!
//! #[derive(Debug)]
//! struct Vector { x: f32, y: f32 }
//!
//! impl IsClose<f32> for Vector {
//!     // Use the same default tolerances as f32
//!     // You can override the defaults here if necessary
//!     const ABS_TOL: f32 = <f32 as IsClose>::ABS_TOL;
//!     const REL_TOL: f32 = <f32 as IsClose>::REL_TOL;
//!
//!     // The is_close_impl function is the only one that must be implemented
//!     // to implement IsClose. The other functions will delegate to this one.
//!     fn is_close_impl(
//!         &self,
//!         other: &Self,
//!         rel_tol: &f32,
//!         abs_tol: &f32,
//!     ) -> bool {
//!         self.x.is_close_impl(&other.x, rel_tol, abs_tol) &&
//!             self.y.is_close_impl(&other.y, rel_tol, abs_tol)
//!     }
//! }
//!
//! assert_is_close!(
//!     Vector {
//!         x: 0.1 + 0.2,
//!         y: 0.2 + 0.4,
//!     },
//!     Vector {
//!         x: 0.3,
//!         y: 0.6,
//!     }
//! )
//! ```
#![warn(
    missing_docs,
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[doc(hidden)]
pub mod macros;

#[cfg(feature = "half")]
mod half;

#[cfg(feature = "euclid")]
mod euclid;

use core::borrow::Borrow;

/// Utility crate since floats don't implement [`f32::abs`] in `no_std`
trait Abs {
    fn abs(&self) -> Self;
}

#[cfg(feature = "std")]
mod abs {
    impl crate::Abs for f32 {
        fn abs(&self) -> Self {
            Self::abs(*self)
        }
    }

    impl crate::Abs for f64 {
        fn abs(&self) -> Self {
            Self::abs(*self)
        }
    }
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
mod abs {
    impl crate::Abs for f32 {
        fn abs(&self) -> Self {
            libm::fabsf(*self)
        }
    }

    impl crate::Abs for f64 {
        fn abs(&self) -> Self {
            libm::fabs(*self)
        }
    }
}

/// Trait used to return a generic zero value for the tolerance
pub trait Zero {
    /// The zero constant
    const ZERO: Self;
}

/// Trait used for testing if floating point values are approximately equal
pub trait IsClose<Tolerance = Self>
where
    Tolerance: Zero,
{
    /// The default absolute tolerance value
    const ABS_TOL: Tolerance;

    /// The default relative tolerance value
    const REL_TOL: Tolerance;

    /// Check if two values are approximately equal using the given relative and
    /// absolute tolerances.
    ///
    /// This is the only function that must be reimplemented to implement the
    /// [`IsClose`] trait for foreign types.
    ///
    /// **Note:** This function exists to allow the other `is_close*` functions
    /// to delegate to this one. It should almost never be called directly.
    fn is_close_impl(&self, other: &Self, rel_tol: &Tolerance, abs_tol: &Tolerance) -> bool;

    /// Check if two values are approximately equal using the given relative and
    /// absolute tolerances.
    ///
    /// **Note:** When implementing [`IsClose`] for foreign types the default
    /// implementation of [`IsClose::is_close_tol`] should almost never be
    /// overridden. Instead, you should implement [`IsClose::is_close_impl`]
    /// which this function delegates to.
    #[inline]
    fn is_close_tol(
        &self,
        other: impl Borrow<Self>,
        rel_tol: impl Borrow<Tolerance>,
        abs_tol: impl Borrow<Tolerance>,
    ) -> bool {
        self.is_close_impl(other.borrow(), rel_tol.borrow(), abs_tol.borrow())
    }

    /// Check if two values are approximately equal. This is equivalent to
    /// calling [`IsClose::is_close_tol`] with [`IsClose::REL_TOL`] and
    /// [`IsClose::ABS_TOL`] as the respective tolerance arguments.
    ///
    /// **Note:** When implementing [`IsClose`] for foreign types the default
    /// implementation of [`IsClose::is_close`] should almost never be
    /// overridden. Instead, you should implement [`IsClose::is_close_impl`]
    /// which this function delegates to.
    #[inline]
    fn is_close(&self, other: impl Borrow<Self>) -> bool {
        self.is_close_impl(other.borrow(), &Self::REL_TOL, &Self::ABS_TOL)
    }

    /// Check if two values are approximately equal using the given relative
    /// tolerance. This is equivalent to calling [`IsClose::is_close_tol`] with
    /// an absolute tolerance of `0.0`.
    ///
    /// **Note:** When implementing [`IsClose`] for foreign types the default
    /// implementation of [`IsClose::is_close_rel_tol`] should almost never be
    /// overridden. Instead, you should implement [`IsClose::is_close_impl`]
    /// which this function delegates to.
    #[inline]
    fn is_close_rel_tol(&self, other: impl Borrow<Self>, rel_tol: impl Borrow<Tolerance>) -> bool {
        self.is_close_impl(other.borrow(), rel_tol.borrow(), &Tolerance::ZERO)
    }

    /// Check if two values are approximately equal using the given absolute
    /// tolerance. This is equivalent to calling [`IsClose::is_close_tol`] with
    /// an relative tolerance of `0.0`.
    ///
    /// **Note:** When implementing [`IsClose`] for foreign types the default
    /// implementation of [`IsClose::is_close_abs_tol`] should almost never be
    /// overridden. Instead, you should implement [`IsClose::is_close_impl`]
    /// which this function delegates to.
    #[inline]
    fn is_close_abs_tol(&self, other: impl Borrow<Self>, abs_tol: impl Borrow<Tolerance>) -> bool {
        self.is_close_impl(other.borrow(), &Tolerance::ZERO, abs_tol.borrow())
    }
}

impl Zero for f32 {
    const ZERO: Self = 0.0;
}

impl IsClose for f32 {
    const ABS_TOL: Self = 1e-6;
    const REL_TOL: Self = 1e-6;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &Self, abs_tol: &Self) -> bool {
        let tol = Self::max(Abs::abs(self), Abs::abs(other)) * rel_tol + abs_tol;
        (*self - *other).abs() <= tol
    }
}

impl Zero for f64 {
    const ZERO: Self = 0.0;
}

impl IsClose for f64 {
    const ABS_TOL: Self = 1e-9;
    const REL_TOL: Self = 1e-9;

    #[inline]
    fn is_close_impl(&self, other: &Self, rel_tol: &Self, abs_tol: &Self) -> bool {
        let tol = Self::max(Abs::abs(self), Abs::abs(other)) * rel_tol + abs_tol;
        (*self - *other).abs() <= tol
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI as PI_F32;
    use std::f64::consts::PI as PI_F64;

    use super::*;

    #[test]
    fn default_is_close() {
        assert!(PI_F32.is_close(355.0 / 113.0));
        assert!(!PI_F32.is_close(22.0 / 7.0));
    }

    #[test]
    fn default_is_close_tol() {
        assert!(1.0.is_close_tol(1.0 + 1e-2, 1e-1, 0.0));
        assert!(!1e-2.is_close_tol(1e-2 + 1e-2, 1e-1, 0.0));
        assert!(1e-2.is_close_tol(1e-2 + 1e-2, 0.0, 1e-1));
        assert!(!1.0.is_close_tol(1.0 + 1.0, 0.0, 1e-1));
    }

    #[test]
    fn default_is_close_rel_tol() {
        assert!(1.0.is_close_rel_tol(1.0 + 1e-2, 1e-1));
        assert!(!1e-2.is_close_rel_tol(1e-2 + 1e-2, 1e-1));
    }

    #[test]
    fn default_is_close_abs_tol() {
        assert!(1e-2.is_close_abs_tol(1e-2 + 1e-2, 1e-1));
        assert!(!1.0.is_close_abs_tol(1.0 + 1.0, 1e-1));
    }

    #[test]
    fn f32_is_close_impl() {
        assert!(PI_F32.is_close_impl(&(22.0 / 7.0), &1e-2, &1e-2));
        assert!(!PI_F32.is_close_impl(&(22.0 / 7.0), &1e-5, &1e-5));
    }

    #[test]
    fn f64_is_close_impl() {
        assert!(PI_F64.is_close_impl(&(22.0 / 7.0), &1e-2, &1e-2));
        assert!(!PI_F64.is_close_impl(&(22.0 / 7.0), &1e-5, &1e-5));
    }
}
