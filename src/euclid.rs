use euclid::{
    Angle, Box2D, Box3D, HomogeneousVector, Length, Point2D, Point3D, Rect, RigidTransform3D,
    Rotation2D, Rotation3D, Scale, SideOffsets2D, Size2D, Size3D, Transform2D, Transform3D,
    Translation2D, Translation3D, Vector2D, Vector3D,
};

use crate::{IsClose, Zero};

impl<T> IsClose<T> for Angle<T>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.radians.is_close_tol(&other.radians, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for Box2D<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.min.is_close_tol(&other.min, rel_tol, abs_tol)
            && self.max.is_close_tol(&other.max, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for Box3D<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.min.is_close_tol(&other.min, rel_tol, abs_tol)
            && self.max.is_close_tol(&other.max, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for HomogeneousVector<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.x.is_close_tol(&other.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&other.y, rel_tol, abs_tol)
            && self.z.is_close_tol(&other.z, rel_tol, abs_tol)
            && self.w.is_close_tol(&other.w, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for Length<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.0.is_close_tol(&other.0, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for Point2D<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.x.is_close_tol(&other.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&other.y, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for Point3D<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.x.is_close_tol(&other.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&other.y, rel_tol, abs_tol)
            && self.z.is_close_tol(&other.z, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for Rect<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.origin.is_close_tol(&other.origin, rel_tol, abs_tol)
            && self.size.is_close_tol(&other.size, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose<T> for RigidTransform3D<T, U1, U2>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.rotation
            .is_close_tol(&other.rotation, rel_tol, abs_tol)
            && self
                .translation
                .is_close_tol(&other.translation, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose<T> for Rotation2D<T, U1, U2>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.angle.is_close_tol(&other.angle, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose<T> for Rotation3D<T, U1, U2>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.i.is_close_tol(&other.i, rel_tol, abs_tol)
            && self.j.is_close_tol(&other.j, rel_tol, abs_tol)
            && self.k.is_close_tol(&other.k, rel_tol, abs_tol)
            && self.r.is_close_tol(&other.r, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose<T> for Scale<T, U1, U2>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.0.is_close_tol(&other.0, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for SideOffsets2D<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.top.is_close_tol(&other.top, rel_tol, abs_tol)
            && self.right.is_close_tol(&other.right, rel_tol, abs_tol)
            && self.bottom.is_close_tol(&other.bottom, rel_tol, abs_tol)
            && self.left.is_close_tol(&other.left, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for Size2D<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.width.is_close_tol(&other.width, rel_tol, abs_tol)
            && self.height.is_close_tol(&other.height, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for Size3D<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.width.is_close_tol(&other.width, rel_tol, abs_tol)
            && self.height.is_close_tol(&other.height, rel_tol, abs_tol)
            && self.depth.is_close_tol(&other.depth, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose<T> for Transform2D<T, U1, U2>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.m11.is_close_tol(&other.m11, rel_tol, abs_tol)
            && self.m12.is_close_tol(&other.m12, rel_tol, abs_tol)
            && self.m21.is_close_tol(&other.m21, rel_tol, abs_tol)
            && self.m22.is_close_tol(&other.m22, rel_tol, abs_tol)
            && self.m31.is_close_tol(&other.m31, rel_tol, abs_tol)
            && self.m32.is_close_tol(&other.m32, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose<T> for Transform3D<T, U1, U2>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.m11.is_close_tol(&other.m11, rel_tol, abs_tol)
            && self.m12.is_close_tol(&other.m12, rel_tol, abs_tol)
            && self.m13.is_close_tol(&other.m13, rel_tol, abs_tol)
            && self.m14.is_close_tol(&other.m14, rel_tol, abs_tol)
            && self.m21.is_close_tol(&other.m21, rel_tol, abs_tol)
            && self.m22.is_close_tol(&other.m22, rel_tol, abs_tol)
            && self.m23.is_close_tol(&other.m23, rel_tol, abs_tol)
            && self.m24.is_close_tol(&other.m24, rel_tol, abs_tol)
            && self.m31.is_close_tol(&other.m31, rel_tol, abs_tol)
            && self.m32.is_close_tol(&other.m32, rel_tol, abs_tol)
            && self.m33.is_close_tol(&other.m33, rel_tol, abs_tol)
            && self.m34.is_close_tol(&other.m34, rel_tol, abs_tol)
            && self.m41.is_close_tol(&other.m41, rel_tol, abs_tol)
            && self.m42.is_close_tol(&other.m42, rel_tol, abs_tol)
            && self.m43.is_close_tol(&other.m43, rel_tol, abs_tol)
            && self.m44.is_close_tol(&other.m44, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose<T> for Translation2D<T, U1, U2>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.x.is_close_tol(&other.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&other.y, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose<T> for Translation3D<T, U1, U2>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.x.is_close_tol(&other.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&other.y, rel_tol, abs_tol)
            && self.z.is_close_tol(&other.z, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for Vector2D<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.x.is_close_tol(&other.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&other.y, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose<T> for Vector3D<T, U>
where
    T: IsClose<T> + Zero,
{
    const ABS_TOL: T = T::ABS_TOL;
    const REL_TOL: T = T::REL_TOL;

    fn is_close_tol(
        &self,
        other: impl core::borrow::Borrow<Self>,
        rel_tol: impl core::borrow::Borrow<T>,
        abs_tol: impl core::borrow::Borrow<T>,
    ) -> bool {
        let (other, rel_tol, abs_tol): (&Self, &T, &T) =
            (other.borrow(), rel_tol.borrow(), abs_tol.borrow());
        self.x.is_close_tol(&other.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&other.y, rel_tol, abs_tol)
            && self.z.is_close_tol(&other.z, rel_tol, abs_tol)
    }
}
