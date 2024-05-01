//! This crate provides a set of traits and macros for comparing arbitrary
//! types.
//!
//! The trait [`IsClose`] is implemented by default for [`f32`] and [`f64`].
//!
//! Additional implementations are also hidden behind the following features:
//!
//! - `half` implements [`IsClose`] for [`f16`][::half::f16] and [`bf16`][::half::bf16]
//! - `euclid` implements [`IsClose`] for [euclid]'s geometric types
//!
//! ## Usage:
//!
//! ```
//! # use isclose::{IsClose, assert_is_close};
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
    fn abs(self) -> Self;
}

#[cfg(feature = "std")]
mod abs {
impl crate::Abs for f32 {
    fn abs(self) -> Self {
        Self::abs(self)
    }
}

impl crate::Abs for f64 {
    fn abs(self) -> Self {
        Self::abs(self)
    }
}
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
mod abs {
impl crate::Abs for f32 {
    fn abs(self) -> Self {
        libm::fabsf(self)
    }
}

impl crate::Abs for f64 {
    fn abs(self) -> Self {
        libm::fabs(self)
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
    /// absolute tolerances
    ///
    /// This function must be reimplemented to implement the [`IsClose`] trait
    /// for other types.
    fn is_close_tol(
        &self,
        other: impl Borrow<Self>,
        rel_tol: impl Borrow<Tolerance>,
        abs_tol: impl Borrow<Tolerance>,
    ) -> bool;

    /// Check if two values are approximately equal. This is equivalent to
    /// calling [`IsClose::is_close_tol`] with [`IsClose::REL_TOL`] and
    /// [`IsClose::ABS_TOL`] as the respective tolerance arguments.
    #[inline]
    fn is_close(&self, other: impl Borrow<Self>) -> bool {
        self.is_close_tol(other, Self::REL_TOL, Self::ABS_TOL)
    }

    /// Check if two values are approximately equal using the given relative
    /// tolerance. This is equivalent to calling [`IsClose::is_close_tol`] with
    /// an absolute tolerance of `0.0`.
    #[inline]
    fn is_close_rel_tol(&self, other: impl Borrow<Self>, rel_tol: impl Borrow<Tolerance>) -> bool {
        self.is_close_tol(other, rel_tol, Tolerance::ZERO)
    }

    /// Check if two values are approximately equal using the given absolute
    /// tolerance. This is equivalent to calling [`IsClose::is_close_tol`] with
    /// an relative tolerance of `0.0`.
    #[inline]
    fn is_close_abs_tol(&self, other: impl Borrow<Self>, abs_tol: impl Borrow<Tolerance>) -> bool {
        self.is_close_tol(other, Tolerance::ZERO, abs_tol)
    }
}

impl Zero for f32 {
    const ZERO: Self = 0.0;
}

impl IsClose for f32 {
    const ABS_TOL: Self = 1e-6;
    const REL_TOL: Self = 1e-6;

    #[inline]
    fn is_close_tol(
        &self,
        other: impl Borrow<Self>,
        rel_tol: impl Borrow<Self>,
        abs_tol: impl Borrow<Self>,
    ) -> bool {
        let (other, rel_tol, abs_tol) = (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        let tol = Self::max(self.abs(), other.abs()) * rel_tol + abs_tol;
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
    fn is_close_tol(
        &self,
        other: impl Borrow<Self>,
        rel_tol: impl Borrow<Self>,
        abs_tol: impl Borrow<Self>,
    ) -> bool {
        let (other, rel_tol, abs_tol) = (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        let tol = Self::max(self.abs(), other.abs()) * rel_tol + abs_tol;
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
    fn f32_is_close_tol() {
        assert!(PI_F32.is_close_tol(22.0 / 7.0, 1e-2, 1e-2));
        assert!(!PI_F32.is_close_tol(22.0 / 7.0, 1e-5, 1e-5));
    }

    #[test]
    fn f64_is_close_tol() {
        assert!(PI_F64.is_close_tol(22.0 / 7.0, 1e-2, 1e-2));
        assert!(!PI_F64.is_close_tol(22.0 / 7.0, 1e-5, 1e-5));
    }
}
