use crate::IsClose;
use core::fmt::Debug;

/// Utility function for performing the comparison with default tolerances
#[doc(hidden)]
#[track_caller]
#[inline]
pub fn assert_is_close<Lhs, Rhs, Tol>(lhs: &Lhs, rhs: &Rhs, args: core::fmt::Arguments<'_>)
where
    Lhs: IsClose<Rhs, Tolerance = Tol> + Debug,
    Rhs: Debug,
    Tol: Debug,
{
    if !lhs.is_close(rhs) {
        assert_failed(lhs, rhs, &Lhs::REL_TOL, &Lhs::ABS_TOL, args);
    }
}

/// Utility function for performing the comparison with a relative tolerance
#[doc(hidden)]
#[track_caller]
#[inline]
pub fn assert_is_close_rel_tol<Lhs, Rhs, Tol>(
    lhs: &Lhs,
    rhs: &Rhs,
    rel_tol: &Tol,
    args: core::fmt::Arguments<'_>,
) where
    Lhs: IsClose<Rhs, Tolerance = Tol> + Debug,
    Rhs: Debug,
    Tol: Debug,
{
    if !lhs.is_close_rel_tol(rhs, rel_tol) {
        assert_failed(lhs, rhs, rel_tol, &Lhs::ZERO_TOL, args);
    }
}

/// Utility function for performing the comparison with a absolute tolerance
#[doc(hidden)]
#[track_caller]
#[inline]
pub fn assert_is_close_abs_tol<Lhs, Rhs, Tol>(
    lhs: &Lhs,
    rhs: &Rhs,
    abs_tol: &Tol,
    args: core::fmt::Arguments<'_>,
) where
    Lhs: IsClose<Rhs, Tolerance = Tol> + Debug,
    Rhs: Debug,
    Tol: Debug,
{
    if !lhs.is_close_abs_tol(rhs, abs_tol) {
        assert_failed(lhs, rhs, &Lhs::ZERO_TOL, abs_tol, args);
    }
}

/// Utility function for performing the comparison with the given tolerances
#[doc(hidden)]
#[track_caller]
#[inline]
pub fn assert_is_close_tol<Lhs, Rhs, Tol>(
    lhs: &Lhs,
    rhs: &Rhs,
    rel_tol: &Tol,
    abs_tol: &Tol,
    args: core::fmt::Arguments<'_>,
) where
    Lhs: IsClose<Rhs, Tolerance = Tol> + Debug,
    Rhs: Debug,
    Tol: Debug,
{
    if !lhs.is_close_tol(rhs, rel_tol, abs_tol) {
        assert_failed(lhs, rhs, rel_tol, abs_tol, args);
    }
}

/// Utility function to print the panicking error message
#[track_caller]
#[cold]
fn assert_failed(
    lhs: &dyn Debug,
    rhs: &dyn Debug,
    rel_tol: &dyn Debug,
    abs_tol: &dyn Debug,
    args: core::fmt::Arguments<'_>,
) -> ! {
    panic!(
        "assertion `left ~= right` failed{args}
    left: {lhs:?}
   right: {rhs:?}
 rel tol: {rel_tol:?}
 abs tol: {abs_tol:?}"
    )
}

/// Assert that two values are approximately equal
#[macro_export]
macro_rules! assert_is_close {
    ($lhs:expr, $rhs:expr, abs_tol=$abs_tol:expr, rel_tol=$rel_tol:expr $(,)?) => {
        assert_is_close!($lhs, $rhs, rel_tol=$rel_tol, abs_tol=$abs_tol);
    };

    ($lhs:expr, $rhs:expr, abs_tol=$abs_tol:expr, rel_tol=$rel_tol:expr, $($arg:tt)+) => {
        assert_is_close!($lhs, $rhs, rel_tol=$rel_tol, abs_tol=$abs_tol, $($arg)+);
    };

    ($lhs:expr, $rhs:expr, rel_tol=$rel_tol:expr, abs_tol=$abs_tol:expr $(,)?) => {
        match (&$lhs, &$rhs, &$rel_tol, &$abs_tol) {
            (lhs, rhs, rel_tol, abs_tol) => {
                $crate::macros::assert_is_close_tol(lhs, rhs, rel_tol, abs_tol, core::format_args!(""))
            }
        }
    };

    ($lhs:expr, $rhs:expr, rel_tol=$rel_tol:expr, abs_tol=$abs_tol:expr, $($arg:tt)+) => {
        match (&$lhs, &$rhs, &$rel_tol, &$abs_tol) {
            (lhs, rhs, rel_tol, abs_tol) => {
                $crate::macros::assert_is_close_tol(lhs, rhs, rel_tol, abs_tol, core::format_args!(": {}", core::format_args!($($arg)+)))
            }
        }
    };

    ($lhs:expr, $rhs:expr, rel_tol=$rel_tol:expr $(,)?) => {
        match (&$lhs, &$rhs, &$rel_tol) {
            (lhs, rhs, rel_tol) => {
                $crate::macros::assert_is_close_rel_tol(lhs, rhs, core::borrow::Borrow::borrow(rel_tol), core::format_args!(""))
            }
        }
    };

    ($lhs:expr, $rhs:expr, rel_tol=$rel_tol:expr, $($arg:tt)+) => {
        match (&$lhs, &$rhs, &$rel_tol) {
            (lhs, rhs, rel_tol) => {
                $crate::macros::assert_is_close_rel_tol(lhs, rhs, core::borrow::Borrow::borrow(rel_tol), core::format_args!(": {}", core::format_args!($($arg)+)))
            }
        }
    };

    ($lhs:expr, $rhs:expr, abs_tol=$abs_tol:expr $(,)?) => {
        match (&$lhs, &$rhs, &$abs_tol) {
            (lhs, rhs, abs_tol) => {
                $crate::macros::assert_is_close_abs_tol(lhs, rhs, core::borrow::Borrow::borrow(abs_tol), core::format_args!(""))
            }
        }
    };

    ($lhs:expr, $rhs:expr, abs_tol=$abs_tol:expr, $($arg:tt)+) => {
        match (&$lhs, &$rhs, &$abs_tol) {
            (lhs, rhs, abs_tol) => {
                $crate::macros::assert_is_close_abs_tol(lhs, rhs, core::borrow::Borrow::borrow(abs_tol), core::format_args!(": {}", core::format_args!($($arg)+)))
            }
        }
    };

    ($lhs:expr, $rhs:expr $(,)?) => {
        match (&$lhs, &$rhs) {
            (lhs, rhs) => {
                $crate::macros::assert_is_close(lhs, rhs, core::format_args!(""))
            }
        }
    };

    ($lhs:expr, $rhs:expr, $($arg:tt)+) => {
        match (&$lhs, &$rhs) {
            (lhs, rhs) => {
                $crate::macros::assert_is_close(lhs, rhs, core::format_args!(": {}", core::format_args!($($arg)+)))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use core::f32::consts::PI;

    #[test]
    fn assert_is_close() {
        assert_is_close!(PI, 355.0 / 113.0);
        assert_is_close!(&PI, 355.0 / 113.0);
        assert_is_close!(PI, &(355.0 / 113.0));
        assert_is_close!(&PI, &(355.0 / 113.0));
    }

    #[test]
    #[should_panic(expected = "assertion `left ~= right` failed
    left: 2.0
   right: 3.0
 rel tol: 1e-6
 abs tol: 1e-6")]
    fn assert_is_close_error() {
        assert_is_close!(2.0_f32, 3.0);
    }

    #[test]
    #[should_panic(expected = "assertion `left ~= right` failed: message
    left: 2.0
   right: 3.0
 rel tol: 1e-6
 abs tol: 1e-6")]
    fn assert_is_close_error_message() {
        assert_is_close!(2.0_f32, 3.0, "message");
    }

    #[test]
    fn assert_is_close_rel_tol() {
        assert_is_close!(1.0, 1.0 + 1e-2, rel_tol = 1e-1);
        assert_is_close!(&1.0, 1.0 + 1e-2, rel_tol = 1e-1);
        assert_is_close!(1.0, &(1.0 + 1e-2), rel_tol = 1e-1);
        assert_is_close!(&1.0, &(1.0 + 1e-2), rel_tol = 1e-1);
        assert_is_close!(1.0, 1.0 + 1e-2, rel_tol = &1e-1);
        assert_is_close!(&1.0, 1.0 + 1e-2, rel_tol = &1e-1);
        assert_is_close!(1.0, &(1.0 + 1e-2), rel_tol = &1e-1);
        assert_is_close!(&1.0, &(1.0 + 1e-2), rel_tol = &1e-1);
    }

    #[test]
    #[should_panic(expected = "assertion `left ~= right` failed
    left: 0.01
   right: 0.02
 rel tol: 0.1
 abs tol: 0.0")]
    fn assert_is_close_rel_tol_error() {
        assert_is_close!(1e-2_f32, 1e-2 + 1e-2, rel_tol = 1e-1);
    }

    #[test]
    #[should_panic(expected = "assertion `left ~= right` failed: 2.0
    left: 0.01
   right: 0.02
 rel tol: 0.1
 abs tol: 0.0")]
    fn assert_is_close_rel_tol_error_message() {
        assert_is_close!(1e-2_f32, 1e-2 + 1e-2, rel_tol = 1e-1, "{:?}", 2.0);
    }

    #[test]
    fn assert_is_close_abs_tol() {
        assert_is_close!(1e-2, 1e-2 + 1e-2, abs_tol = 1e-1);
        assert_is_close!(&1e-2, 1e-2 + 1e-2, abs_tol = 1e-1);
        assert_is_close!(1e-2, &(1e-2 + 1e-2), abs_tol = 1e-1);
        assert_is_close!(&1e-2, &(1e-2 + 1e-2), abs_tol = 1e-1);
        assert_is_close!(1e-2, 1e-2 + 1e-2, abs_tol = &1e-1);
        assert_is_close!(&1e-2, 1e-2 + 1e-2, abs_tol = &1e-1);
        assert_is_close!(1e-2, &(1e-2 + 1e-2), abs_tol = &1e-1);
        assert_is_close!(&1e-2, &(1e-2 + 1e-2), abs_tol = &1e-1);
    }

    #[test]
    #[should_panic(expected = "assertion `left ~= right` failed
    left: 1.0
   right: 2.0
 rel tol: 0.0
 abs tol: 0.1")]
    fn assert_is_close_abs_tol_error() {
        assert_is_close!(1.0_f32, 1.0 + 1.0, abs_tol = 1e-1);
    }

    #[test]
    #[should_panic(expected = "assertion `left ~= right` failed: false
    left: 1.0
   right: 2.0
 rel tol: 0.0
 abs tol: 0.1")]
    fn assert_is_close_abs_tol_error_message() {
        assert_is_close!(1.0_f32, 1.0 + 1.0, abs_tol = 1e-1, "{}", false);
    }

    #[test]
    fn assert_is_close_tol() {
        assert_is_close!(PI, 22.0 / 7.0, rel_tol = 1e-2, abs_tol = 1e-2);
        assert_is_close!(&PI, 22.0 / 7.0, rel_tol = 1e-2, abs_tol = 1e-2);
        assert_is_close!(PI, &(22.0 / 7.0), rel_tol = 1e-2, abs_tol = 1e-2);
        assert_is_close!(&PI, &(22.0 / 7.0), rel_tol = 1e-2, abs_tol = 1e-2);
        assert_is_close!(PI, 22.0 / 7.0, rel_tol = &1e-2, abs_tol = 1e-2);
        assert_is_close!(&PI, 22.0 / 7.0, rel_tol = &1e-2, abs_tol = 1e-2);
        assert_is_close!(PI, &(22.0 / 7.0), rel_tol = &1e-2, abs_tol = 1e-2);
        assert_is_close!(&PI, &(22.0 / 7.0), rel_tol = &1e-2, abs_tol = 1e-2);
        assert_is_close!(PI, 22.0 / 7.0, rel_tol = 1e-2, abs_tol = &1e-2);
        assert_is_close!(&PI, 22.0 / 7.0, rel_tol = 1e-2, abs_tol = &1e-2);
        assert_is_close!(PI, &(22.0 / 7.0), rel_tol = 1e-2, abs_tol = &1e-2);
        assert_is_close!(&PI, &(22.0 / 7.0), rel_tol = 1e-2, abs_tol = &1e-2);
        assert_is_close!(PI, 22.0 / 7.0, rel_tol = &1e-2, abs_tol = &1e-2);
        assert_is_close!(&PI, 22.0 / 7.0, rel_tol = &1e-2, abs_tol = &1e-2);
        assert_is_close!(PI, &(22.0 / 7.0), rel_tol = &1e-2, abs_tol = &1e-2);
        assert_is_close!(&PI, &(22.0 / 7.0), rel_tol = &1e-2, abs_tol = &1e-2);

        assert_is_close!(PI, 22.0 / 7.0, abs_tol = 1e-2, rel_tol = 1e-2);
        assert_is_close!(&PI, 22.0 / 7.0, abs_tol = 1e-2, rel_tol = 1e-2);
        assert_is_close!(PI, &(22.0 / 7.0), abs_tol = 1e-2, rel_tol = 1e-2);
        assert_is_close!(&PI, &(22.0 / 7.0), abs_tol = 1e-2, rel_tol = 1e-2);
        assert_is_close!(PI, 22.0 / 7.0, abs_tol = 1e-2, rel_tol = &1e-2);
        assert_is_close!(&PI, 22.0 / 7.0, abs_tol = 1e-2, rel_tol = &1e-2);
        assert_is_close!(PI, &(22.0 / 7.0), abs_tol = 1e-2, rel_tol = &1e-2);
        assert_is_close!(&PI, &(22.0 / 7.0), abs_tol = 1e-2, rel_tol = &1e-2);
        assert_is_close!(PI, 22.0 / 7.0, abs_tol = &1e-2, rel_tol = 1e-2);
        assert_is_close!(&PI, 22.0 / 7.0, abs_tol = &1e-2, rel_tol = 1e-2);
        assert_is_close!(PI, &(22.0 / 7.0), abs_tol = &1e-2, rel_tol = 1e-2);
        assert_is_close!(&PI, &(22.0 / 7.0), abs_tol = &1e-2, rel_tol = 1e-2);
        assert_is_close!(PI, 22.0 / 7.0, abs_tol = &1e-2, rel_tol = &1e-2);
        assert_is_close!(&PI, 22.0 / 7.0, abs_tol = &1e-2, rel_tol = &1e-2);
        assert_is_close!(PI, &(22.0 / 7.0), abs_tol = &1e-2, rel_tol = &1e-2);
        assert_is_close!(&PI, &(22.0 / 7.0), abs_tol = &1e-2, rel_tol = &1e-2);
    }

    #[test]
    #[should_panic(expected = "assertion `left ~= right` failed
    left: 3.1415927
   right: 3.142857
 rel tol: 1e-6
 abs tol: 1e-6")]
    fn assert_is_close_tol_error() {
        assert_is_close!(PI, 22.0 / 7.0, rel_tol = 1e-6, abs_tol = 1e-6);
    }

    #[test]
    #[should_panic(expected = "assertion `left ~= right` failed: None
    left: 3.1415927
   right: 3.142857
 rel tol: 1e-6
 abs tol: 1e-6")]
    fn assert_is_close_tol_error_message() {
        assert_is_close!(
            PI,
            22.0 / 7.0,
            rel_tol = 1e-6,
            abs_tol = 1e-6,
            "{:?}",
            Option::<()>::None,
        );
    }
}
