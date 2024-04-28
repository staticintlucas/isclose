use core::borrow::Borrow;

use crate::{IsClose, Zero};

use half::{bf16, f16};

impl Zero for f16 {
    const ZERO: Self = Self::ZERO;
}

impl IsClose for f16 {
    const ABS_TOL: Self = Self::from_f32_const(1e-3);
    const REL_TOL: Self = Self::from_f32_const(1e-3);

    #[inline]
    fn is_close_tol(
        &self,
        other: impl Borrow<Self>,
        rel_tol: impl Borrow<Self>,
        abs_tol: impl Borrow<Self>,
    ) -> bool {
        let (other, rel_tol, abs_tol) = (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.to_f32()
            .is_close_tol(other.to_f32(), rel_tol.to_f32(), abs_tol.to_f32())
    }
}

impl Zero for bf16 {
    const ZERO: Self = Self::ZERO;
}

impl IsClose for bf16 {
    const ABS_TOL: Self = Self::from_f32_const(1e-2);
    const REL_TOL: Self = Self::from_f32_const(1e-2);

    #[inline]
    fn is_close_tol(
        &self,
        other: impl Borrow<Self>,
        rel_tol: impl Borrow<Self>,
        abs_tol: impl Borrow<Self>,
    ) -> bool {
        let (other, rel_tol, abs_tol) = (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.to_f32()
            .is_close_tol(other.to_f32(), rel_tol.to_f32(), abs_tol.to_f32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f16_is_close_tol() {
        use half::f16;
        assert!((f16::from_f32(0.1) + f16::from_f32(0.2)).is_close(f16::from_f32(0.3)));
        assert!(!(f16::from_f32(0.1) + f16::from_f32(0.2)).is_close_tol(
            f16::from_f32(0.3),
            f16::from_f32(1e-6),
            f16::from_f32(1e-6)
        ));
    }

    #[test]
    fn bf16_is_close_tol() {
        use half::bf16;
        assert!((bf16::from_f32(0.3) + bf16::EPSILON).is_close(bf16::from_f32(0.3)));
        assert!(!(bf16::from_f32(0.3) + bf16::EPSILON).is_close_tol(
            bf16::from_f32(0.3),
            bf16::from_f32(1e-4),
            bf16::from_f32(1e-4)
        ));
    }
}
