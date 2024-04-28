use crate::{IsClose, Zero};
use core::fmt::Debug;

/// Utility function to print the panicking error message
#[doc(hidden)]
#[track_caller]
pub fn assert_failed<Value, Tolerance>(
    lhs: &Value,
    rhs: &Value,
    rel_tol: Option<&Tolerance>,
    abs_tol: Option<&Tolerance>,
    args: Option<core::fmt::Arguments<'_>>,
) -> !
where
    Value: IsClose<Tolerance> + Debug,
    Tolerance: Zero + Debug,
{
    let zero = Tolerance::ZERO;
    let (def_rel, def_abs) = (Value::REL_TOL, Value::ABS_TOL);
    let (rel_tol, abs_tol) = match (rel_tol, abs_tol) {
        (Some(r), Some(a)) => (r, a),
        (Some(r), None) => (r, &zero),
        (None, Some(a)) => (&zero, a),
        (None, None) => (&def_rel, &def_abs),
    };

    #[allow(clippy::option_if_let_else)] // map_or_else is super ugly here
    match args {
        Some(args) => panic!(
            "assertion `left ~= right` failed: {}
    left: {:?}
   right: {:?}
 rel tol: {:?}
 abs tol: {:?}",
            args, lhs, rhs, rel_tol, abs_tol,
        ),
        None => panic!(
            "assertion `left ~= right` failed
    left: {:?}
   right: {:?}
 rel tol: {:?}
 abs tol: {:?}",
            lhs, rhs, rel_tol, abs_tol,
        ),
    }
}

#[macro_export]
macro_rules! assert_is_close {
    ($lhs:expr, $rhs:expr $(,)?) => {
        match (&$lhs, &$rhs) {
            (lhs, rhs) => {
                use core::borrow::Borrow;
                let (lhs, rhs) = ((*lhs).borrow(), (*rhs).borrow());
                if !$crate::IsClose::is_close(lhs, rhs) {
                    $crate::macros::assert_failed(
                        lhs, rhs, None, None, None,
                    );
                }
            }
        }
    };

    ($lhs:expr, $rhs:expr, $($arg:tt)+) => {
        match (&$lhs, &$rhs) {
            (lhs, rhs) => {
                use core::borrow::Borrow;
                let (lhs, rhs) = ((*lhs).borrow(), (*rhs).borrow());
                if !$crate::IsClose::is_close(lhs, rhs) {
                    $crate::macros::assert_failed(
                        lhs, rhs, None, None, Some(core::format_args!($($arg)+))
                    );
                }
            }
        }
    };
}

#[macro_export]
macro_rules! assert_is_close_rel_tol {
    ($lhs:expr, $rhs:expr, $rel_tol:expr $(,)?) => {
        match (&$lhs, &$rhs, &$rel_tol) {
            (lhs, rhs, rel_tol) => {
                use core::borrow::Borrow;
                let (lhs, rhs, rel_tol) = ((*lhs).borrow(), (*rhs).borrow(), (*rel_tol).borrow());
                if !$crate::IsClose::is_close_rel_tol(lhs, rhs, rel_tol) {
                    $crate::macros::assert_failed(
                        lhs, rhs, Some(rel_tol), None, None,
                    );
                }
            }
        }
    };

    ($lhs:expr, $rhs:expr, $rel_tol:expr, $($arg:tt)+) => {
        match (&$lhs, &$rhs, &$rel_tol) {
            (lhs, rhs, rel_tol) => {
                use core::borrow::Borrow;
                let (lhs, rhs, rel_tol) = ((*lhs).borrow(), (*rhs).borrow(), (*rel_tol).borrow());
                if !$crate::IsClose::is_close_rel_tol(lhs, rhs, rel_tol) {
                    $crate::macros::assert_failed(
                        lhs, rhs, Some(rel_tol), None, Some(core::format_args!($($arg)+)),
                    );
                }
            }
        }
    };
}

#[macro_export]
macro_rules! assert_is_close_abs_tol {
    ($lhs:expr, $rhs:expr, $abs_tol:expr $(,)?) => {
        match (&$lhs, &$rhs, &$abs_tol) {
            (lhs, rhs, abs_tol) => {
                use core::borrow::Borrow;
                let (lhs, rhs, abs_tol) = ((*lhs).borrow(), (*rhs).borrow(), (*abs_tol).borrow());
                if !$crate::IsClose::is_close_abs_tol(lhs, rhs, abs_tol) {
                    $crate::macros::assert_failed(
                        lhs, rhs, None, Some(abs_tol), None,
                    );
                }
            }
        }
    };

    ($lhs:expr, $rhs:expr, $abs_tol:expr, $($arg:tt)+) => {
        match (&$lhs, &$rhs, &$abs_tol) {
            (lhs, rhs, abs_tol) => {
                use core::borrow::Borrow;
                let (lhs, rhs, abs_tol) = ((*lhs).borrow(), (*rhs).borrow(), (*abs_tol).borrow());
                if !$crate::IsClose::is_close_abs_tol(lhs, rhs, abs_tol) {
                    $crate::macros::assert_failed(
                        lhs, rhs, None, Some(abs_tol), Some(core::format_args!($($arg)+)),
                    );
                }
            }
        }
    };
}

#[macro_export]
macro_rules! assert_is_close_tol {
    ($lhs:expr, $rhs:expr, $rel_tol:expr, $abs_tol:expr $(,)?) => {
        match (&$lhs, &$rhs, &$rel_tol, &$abs_tol) {
            (lhs, rhs, rel_tol, abs_tol) => {
                use core::borrow::Borrow;
                let (lhs, rhs, rel_tol, abs_tol) =
                    ((*lhs).borrow(), (*rhs).borrow(), (*rel_tol).borrow(), (*abs_tol).borrow());
                if !$crate::IsClose::is_close_tol(lhs, rhs, rel_tol, abs_tol) {
                    $crate::macros::assert_failed(
                        lhs, rhs, Some(rel_tol), Some(abs_tol), None,
                    );
                }
            }
        }
    };

    ($lhs:expr, $rhs:expr, $rel_tol:expr, $abs_tol:expr, $($arg:tt)+) => {
        match (&$lhs, &$rhs, &$rel_tol, &$abs_tol) {
            (lhs, rhs, rel_tol, abs_tol) => {
                use core::borrow::Borrow;
                let (lhs, rhs, rel_tol, abs_tol) =
                    ((*lhs).borrow(), (*rhs).borrow(), (*rel_tol).borrow(), (*abs_tol).borrow());
                if !$crate::IsClose::is_close_tol(lhs, rhs, rel_tol, abs_tol) {
                    $crate::macros::assert_failed(
                        lhs, rhs, Some(rel_tol), Some(abs_tol), Some(core::format_args!($($arg)+)),
                    );
                }
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
    fn assert_is_close_error() {
        let err = std::panic::catch_unwind(|| assert_is_close!(2.0_f32, 3.0)).unwrap_err();
        let msg: &String = err.downcast_ref().unwrap();

        assert_eq!(
            msg,
            &format!(
                "assertion `left ~= right` failed
    left: 2.0
   right: 3.0
 rel tol: {0:?}
 abs tol: {0:?}",
                1e-6
            ) // Rust <=1.57 formats this differently, so we need to use format!()
        );

        let err =
            std::panic::catch_unwind(|| assert_is_close!(2.0_f32, 3.0, "message")).unwrap_err();
        let msg: &String = err.downcast_ref().unwrap();

        assert_eq!(
            msg,
            &format!(
                "assertion `left ~= right` failed: message
    left: 2.0
   right: 3.0
 rel tol: {0:?}
 abs tol: {0:?}",
                1e-6
            ) // Rust <=1.57 formats this differently, so we need to use format!()
        );
    }

    #[test]
    fn assert_is_close_rel_tol() {
        assert_is_close_rel_tol!(1.0, 1.0 + 1e-2, 1e-1);
        assert_is_close_rel_tol!(&1.0, 1.0 + 1e-2, 1e-1);
        assert_is_close_rel_tol!(1.0, &(1.0 + 1e-2), 1e-1);
        assert_is_close_rel_tol!(&1.0, &(1.0 + 1e-2), 1e-1);
        assert_is_close_rel_tol!(1.0, 1.0 + 1e-2, &1e-1);
        assert_is_close_rel_tol!(&1.0, 1.0 + 1e-2, &1e-1);
        assert_is_close_rel_tol!(1.0, &(1.0 + 1e-2), &1e-1);
        assert_is_close_rel_tol!(&1.0, &(1.0 + 1e-2), &1e-1);
    }

    #[test]
    fn assert_is_close_rel_tol_error() {
        let err =
            std::panic::catch_unwind(|| assert_is_close_rel_tol!(1e-2_f32, 1e-2 + 1e-2, 1e-1))
                .unwrap_err();
        let msg: &String = err.downcast_ref().unwrap();

        assert_eq!(
            msg,
            "assertion `left ~= right` failed
    left: 0.01
   right: 0.02
 rel tol: 0.1
 abs tol: 0.0"
        );

        let err = std::panic::catch_unwind(|| {
            assert_is_close_rel_tol!(1e-2_f32, 1e-2 + 1e-2, 1e-1, "{:?}", 2.0);
        })
        .unwrap_err();
        let msg: &String = err.downcast_ref().unwrap();

        assert_eq!(
            msg,
            "assertion `left ~= right` failed: 2.0
    left: 0.01
   right: 0.02
 rel tol: 0.1
 abs tol: 0.0"
        );
    }

    #[test]
    fn assert_is_close_abs_tol() {
        assert_is_close_abs_tol!(1e-2, 1e-2 + 1e-2, 1e-1);
        assert_is_close_abs_tol!(&1e-2, 1e-2 + 1e-2, 1e-1);
        assert_is_close_abs_tol!(1e-2, &(1e-2 + 1e-2), 1e-1);
        assert_is_close_abs_tol!(&1e-2, &(1e-2 + 1e-2), 1e-1);
        assert_is_close_abs_tol!(1e-2, 1e-2 + 1e-2, &1e-1);
        assert_is_close_abs_tol!(&1e-2, 1e-2 + 1e-2, &1e-1);
        assert_is_close_abs_tol!(1e-2, &(1e-2 + 1e-2), &1e-1);
        assert_is_close_abs_tol!(&1e-2, &(1e-2 + 1e-2), &1e-1);
    }

    #[test]
    fn assert_is_close_abs_tol_error() {
        let err = std::panic::catch_unwind(|| assert_is_close_abs_tol!(1.0_f32, 1.0 + 1.0, 1e-1))
            .unwrap_err();
        let msg: &String = err.downcast_ref().unwrap();

        assert_eq!(
            msg,
            "assertion `left ~= right` failed
    left: 1.0
   right: 2.0
 rel tol: 0.0
 abs tol: 0.1"
        );

        let err = std::panic::catch_unwind(|| {
            assert_is_close_abs_tol!(1.0_f32, 1.0 + 1.0, 1e-1, "{}", false);
        })
        .unwrap_err();
        let msg: &String = err.downcast_ref().unwrap();

        assert_eq!(
            msg,
            "assertion `left ~= right` failed: false
    left: 1.0
   right: 2.0
 rel tol: 0.0
 abs tol: 0.1"
        );
    }

    #[test]
    fn assert_is_close_tol() {
        assert_is_close_tol!(PI, 22.0 / 7.0, 1e-2, 1e-2);
        assert_is_close_tol!(&PI, 22.0 / 7.0, 1e-2, 1e-2);
        assert_is_close_tol!(PI, &(22.0 / 7.0), 1e-2, 1e-2);
        assert_is_close_tol!(&PI, &(22.0 / 7.0), 1e-2, 1e-2);
        assert_is_close_tol!(PI, 22.0 / 7.0, &1e-2, 1e-2);
        assert_is_close_tol!(&PI, 22.0 / 7.0, &1e-2, 1e-2);
        assert_is_close_tol!(PI, &(22.0 / 7.0), &1e-2, 1e-2);
        assert_is_close_tol!(&PI, &(22.0 / 7.0), &1e-2, 1e-2);
        assert_is_close_tol!(PI, 22.0 / 7.0, 1e-2, &1e-2);
        assert_is_close_tol!(&PI, 22.0 / 7.0, 1e-2, &1e-2);
        assert_is_close_tol!(PI, &(22.0 / 7.0), 1e-2, &1e-2);
        assert_is_close_tol!(&PI, &(22.0 / 7.0), 1e-2, &1e-2);
        assert_is_close_tol!(PI, 22.0 / 7.0, &1e-2, &1e-2);
        assert_is_close_tol!(&PI, 22.0 / 7.0, &1e-2, &1e-2);
        assert_is_close_tol!(PI, &(22.0 / 7.0), &1e-2, &1e-2);
        assert_is_close_tol!(&PI, &(22.0 / 7.0), &1e-2, &1e-2);
    }

    #[test]
    fn assert_is_close_tol_error() {
        let err = std::panic::catch_unwind(|| assert_is_close_tol!(PI, 22.0 / 7.0, 1e-6, 1e-6))
            .unwrap_err();
        let msg: &String = err.downcast_ref().unwrap();

        assert_eq!(
            msg,
            &format!(
                "assertion `left ~= right` failed
    left: 3.1415927
   right: 3.142857
 rel tol: {0:?}
 abs tol: {0:?}",
                1e-6
            ) // Rust <=1.57 formats this differently, so we need to use format!()
        );

        let err = std::panic::catch_unwind(|| {
            assert_is_close_tol!(PI, 22.0 / 7.0, 1e-6, 1e-6, "{:?}", Option::<()>::None);
        })
        .unwrap_err();
        let msg: &String = err.downcast_ref().unwrap();

        assert_eq!(
            msg,
            &format!(
                "assertion `left ~= right` failed: None
    left: 3.1415927
   right: 3.142857
 rel tol: {0:?}
 abs tol: {0:?}",
                1e-6
            ) // Rust <=1.57 formats this differently, so we need to use format!()
        );
    }
}
