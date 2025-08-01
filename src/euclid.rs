use euclid::{
    Angle, Box2D, Box3D, HomogeneousVector, Length, Point2D, Point3D, Rect, RigidTransform3D,
    Rotation2D, Rotation3D, Scale, SideOffsets2D, Size2D, Size3D, Transform2D, Transform3D,
    Translation2D, Translation3D, Vector2D, Vector3D,
};

use crate::IsClose;

impl<T> IsClose for Angle<T>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.radians.is_close_tol(&rhs.radians, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for Box2D<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.min.is_close_tol(&rhs.min, rel_tol, abs_tol)
            && self.max.is_close_tol(&rhs.max, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for Box3D<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.min.is_close_tol(&rhs.min, rel_tol, abs_tol)
            && self.max.is_close_tol(&rhs.max, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for HomogeneousVector<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.x.is_close_tol(&rhs.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&rhs.y, rel_tol, abs_tol)
            && self.z.is_close_tol(&rhs.z, rel_tol, abs_tol)
            && self.w.is_close_tol(&rhs.w, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for Length<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.0.is_close_tol(&rhs.0, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for Point2D<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.x.is_close_tol(&rhs.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&rhs.y, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for Point3D<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.x.is_close_tol(&rhs.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&rhs.y, rel_tol, abs_tol)
            && self.z.is_close_tol(&rhs.z, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for Rect<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.origin.is_close_tol(&rhs.origin, rel_tol, abs_tol)
            && self.size.is_close_tol(&rhs.size, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose for RigidTransform3D<T, U1, U2>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.rotation.is_close_tol(&rhs.rotation, rel_tol, abs_tol)
            && self
                .translation
                .is_close_tol(&rhs.translation, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose for Rotation2D<T, U1, U2>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.angle.is_close_tol(&rhs.angle, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose for Rotation3D<T, U1, U2>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.i.is_close_tol(&rhs.i, rel_tol, abs_tol)
            && self.j.is_close_tol(&rhs.j, rel_tol, abs_tol)
            && self.k.is_close_tol(&rhs.k, rel_tol, abs_tol)
            && self.r.is_close_tol(&rhs.r, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose for Scale<T, U1, U2>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.0.is_close_tol(&rhs.0, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for SideOffsets2D<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.top.is_close_tol(&rhs.top, rel_tol, abs_tol)
            && self.right.is_close_tol(&rhs.right, rel_tol, abs_tol)
            && self.bottom.is_close_tol(&rhs.bottom, rel_tol, abs_tol)
            && self.left.is_close_tol(&rhs.left, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for Size2D<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.width.is_close_tol(&rhs.width, rel_tol, abs_tol)
            && self.height.is_close_tol(&rhs.height, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for Size3D<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.width.is_close_tol(&rhs.width, rel_tol, abs_tol)
            && self.height.is_close_tol(&rhs.height, rel_tol, abs_tol)
            && self.depth.is_close_tol(&rhs.depth, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose for Transform2D<T, U1, U2>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.m11.is_close_tol(&rhs.m11, rel_tol, abs_tol)
            && self.m12.is_close_tol(&rhs.m12, rel_tol, abs_tol)
            && self.m21.is_close_tol(&rhs.m21, rel_tol, abs_tol)
            && self.m22.is_close_tol(&rhs.m22, rel_tol, abs_tol)
            && self.m31.is_close_tol(&rhs.m31, rel_tol, abs_tol)
            && self.m32.is_close_tol(&rhs.m32, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose for Transform3D<T, U1, U2>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.m11.is_close_tol(&rhs.m11, rel_tol, abs_tol)
            && self.m12.is_close_tol(&rhs.m12, rel_tol, abs_tol)
            && self.m13.is_close_tol(&rhs.m13, rel_tol, abs_tol)
            && self.m14.is_close_tol(&rhs.m14, rel_tol, abs_tol)
            && self.m21.is_close_tol(&rhs.m21, rel_tol, abs_tol)
            && self.m22.is_close_tol(&rhs.m22, rel_tol, abs_tol)
            && self.m23.is_close_tol(&rhs.m23, rel_tol, abs_tol)
            && self.m24.is_close_tol(&rhs.m24, rel_tol, abs_tol)
            && self.m31.is_close_tol(&rhs.m31, rel_tol, abs_tol)
            && self.m32.is_close_tol(&rhs.m32, rel_tol, abs_tol)
            && self.m33.is_close_tol(&rhs.m33, rel_tol, abs_tol)
            && self.m34.is_close_tol(&rhs.m34, rel_tol, abs_tol)
            && self.m41.is_close_tol(&rhs.m41, rel_tol, abs_tol)
            && self.m42.is_close_tol(&rhs.m42, rel_tol, abs_tol)
            && self.m43.is_close_tol(&rhs.m43, rel_tol, abs_tol)
            && self.m44.is_close_tol(&rhs.m44, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose for Translation2D<T, U1, U2>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.x.is_close_tol(&rhs.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&rhs.y, rel_tol, abs_tol)
    }
}

impl<T, U1, U2> IsClose for Translation3D<T, U1, U2>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.x.is_close_tol(&rhs.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&rhs.y, rel_tol, abs_tol)
            && self.z.is_close_tol(&rhs.z, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for Vector2D<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.x.is_close_tol(&rhs.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&rhs.y, rel_tol, abs_tol)
    }
}

impl<T, U> IsClose for Vector3D<T, U>
where
    T: IsClose<Tolerance = T>,
{
    type Tolerance = T;
    const ZERO_TOL: T = <T as IsClose>::ZERO_TOL;
    const ABS_TOL: T = <T as IsClose>::ABS_TOL;
    const REL_TOL: T = <T as IsClose>::REL_TOL;

    #[inline]
    fn is_close_tol(&self, rhs: &Self, rel_tol: &T, abs_tol: &T) -> bool {
        self.x.is_close_tol(&rhs.x, rel_tol, abs_tol)
            && self.y.is_close_tol(&rhs.y, rel_tol, abs_tol)
            && self.z.is_close_tol(&rhs.z, rel_tol, abs_tol)
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::{FRAC_PI_3, PI};

    use euclid::default::{
        Box2D, Box3D, HomogeneousVector, Length, Point2D, Point3D, Rect, RigidTransform3D,
        Rotation2D, Rotation3D, Scale, SideOffsets2D, Size2D, Size3D, Transform2D, Transform3D,
        Translation2D, Translation3D, Vector2D, Vector3D,
    };
    use euclid::Angle;

    use crate::assert_is_close;

    #[test]
    fn angle() {
        let angle1 = Angle::degrees(180.0);
        let angle2 = Angle::radians(PI);

        assert_is_close!(angle1, angle2);
    }

    #[test]
    fn box_2d() {
        let box1 = Box2D::new(Point2D::new(1.0, 2.0), Point2D::new(3.0, 4.0));
        let box2 = Box2D::new(Point2D::origin(), Point2D::new(2.0, 2.0))
            .translate(Vector2D::new(1.0, 2.0));

        assert_is_close!(box1, box2);
    }

    #[test]
    fn box_3d() {
        let box1 = Box3D::new(Point3D::new(1.0, 2.0, 3.0), Point3D::new(4.0, 5.0, 6.0));
        let box2 = Box3D::new(Point3D::origin(), Point3D::new(3.0, 3.0, 3.0))
            .translate(Vector3D::new(1.0, 2.0, 3.0));

        assert_is_close!(box1, box2);
    }

    #[test]
    fn homogen_vec() {
        let vec1 = HomogeneousVector::new(1.0, 2.0, 3.0, 1.0);
        let vec2 = HomogeneousVector::from(vec1.to_point3d().unwrap());

        assert_is_close!(vec1, vec2);
    }

    #[test]
    fn length() {
        let length1 = Length::new(1.5);
        let length2 = -Length::default().lerp(length1, -1.0);

        assert_is_close!(length1, length2);
    }

    #[test]
    fn point_2d() {
        let point1 = Point2D::new(2.5, 3.0);
        let point2 = Point2D::new(1.5, 1.0).add_size(&Size2D::new(1.0, 2.0));

        assert_is_close!(point1, point2);
    }

    #[test]
    fn point_3d() {
        let point1 = Point3D::new(2.5, 3.0, 4.5);
        let point2 = Point3D::new(1.5, 1.0, 1.5).add_size(Size3D::new(1.0, 2.0, 3.0));

        assert_is_close!(point1, point2);
    }

    #[test]
    fn rect() {
        let rect1 = Rect::new(Point2D::new(1.0, 2.0), Size2D::new(1.0, 1.0));
        let rect2 = Box2D::new(Point2D::origin(), Point2D::splat(1.0))
            .to_rect()
            .translate(Vector2D::new(1.0, 2.0));

        assert_is_close!(rect1, rect2);
    }

    #[test]
    fn rigid_transform_3d() {
        let rt1 = RigidTransform3D::new(
            Rotation3D::around_axis(Vector3D::new(1.0, 2.0, 3.0), Angle::degrees(90.0)),
            Vector3D::new(1.0, 2.0, 3.0),
        );
        let rt2 = RigidTransform3D::new(
            Rotation3D::around_axis(Vector3D::new(1.0, 2.0, 3.0), Angle::degrees(-90.0)),
            -Vector3D::new(1.0, 2.0, 3.0),
        )
        .inverse();

        assert_is_close!(rt1, rt2);
    }

    #[test]
    fn rotation_2d() {
        use crate::IsClose as _; // W/A since Rotation2D doesn't impl Debug (servo/euclid#525)

        let rot1 = Rotation2D::new(Angle::degrees(180.0));
        let rot2 = Rotation2D::new(Angle::radians(PI));

        assert!(rot1.is_close(&rot2));
    }

    #[test]
    fn rotation_3d() {
        let rot1 = Rotation3D::around_axis(Vector3D::new(1.0, 2.0, 3.0), Angle::degrees(90.0));
        let rot2 = Rotation3D::around_axis(-Vector3D::new(1.0, 2.0, 3.0), -Angle::degrees(90.0));

        assert_is_close!(rot1, rot2);
    }

    #[test]
    fn scale() {
        let scale1 = Scale::new(2.0);
        let scale2 = Scale::new(0.5).inverse();

        assert_is_close!(scale1, scale2);
    }

    #[test]
    fn side_offsets_2d() {
        let offset1 =
            SideOffsets2D::from_vectors_inner(Vector2D::splat(1.0), Vector2D::splat(-1.0));
        let offset2 =
            SideOffsets2D::from_vectors_outer(Vector2D::splat(-1.0), Vector2D::splat(1.0));

        assert_is_close!(offset1, offset2);
    }

    #[test]
    fn size_2d() {
        let size1 = Size2D::new(2.0, 3.0);
        let size2 = Size2D::new(2.0 / 3.0, 1.0) * 3.0;

        assert_is_close!(size1, size2);
    }

    #[test]
    fn size_3d() {
        let size1 = Size3D::new(2.0, 3.0, PI);
        let size2 = Size3D::new(2.0 / 3.0, 1.0, FRAC_PI_3) * 3.0;

        assert_is_close!(size1, size2);
    }

    #[test]
    fn transform_2d() {
        let xform1 = Transform2D::scale(0.5, 0.25)
            .then_translate(Vector2D::splat(1.0))
            .then_rotate(Angle::radians(FRAC_PI_3));
        let xform2 = Transform2D::rotation(-Angle::degrees(60.0))
            .then_translate(-Vector2D::splat(1.0))
            .then_scale(2.0, 4.0)
            .inverse()
            .unwrap();

        assert_is_close!(xform1, xform2);
    }

    #[test]
    fn transform_3d() {
        let xform1 = Transform3D::scale(0.5, 0.25, 0.5).then_translate(Vector3D::splat(1.0));
        let xform2 = Transform3D::identity()
            .then_translate(-Vector3D::splat(1.0))
            .then_scale(2.0, 4.0, 2.0)
            .inverse()
            .unwrap();

        assert_is_close!(xform1, xform2);
    }

    #[test]
    fn translation_2d() {
        let xlate1 = Translation2D::new(2.5, 3.0);
        let xlate2 = Translation2D::new(1.5, 1.0).inverse() + Translation2D::new(4.0, 4.0);

        assert_is_close!(xlate1, xlate2);
    }

    #[test]
    fn translation_3d() {
        let xlate1 = Translation3D::new(2.5, 3.0, 4.5);
        let xlate2 =
            Translation3D::new(1.5, 1.0, 1.5).inverse() + Translation3D::new(4.0, 4.0, 6.0);

        assert_is_close!(xlate1, xlate2);
    }

    #[test]
    fn vector_2d() {
        let vec1 = Vector2D::new(2.5, 3.0);
        let vec2 = Vector2D::new(1.5, 1.25).yx() * 2.0;

        assert_is_close!(vec1, vec2);
    }

    #[test]
    fn vector_3d() {
        let xlate1 = Vector3D::new(2.5, 4.0, -1.0);
        let xlate2 = Vector3D::new(-1.0, 1.0, 1.5).cross(Vector3D::new(2.0, -1.0, 1.0));

        assert_is_close!(xlate1, xlate2);
    }
}
