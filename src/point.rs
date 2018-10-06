use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

const FLOAT_MARGIN: f32 = 0.0000001;
const ZERO_VECTOR: Tuple = Tuple {
    x: 0.,
    y: 0.,
    z: 0.,
    w: 0.,
};

fn float_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < FLOAT_MARGIN
}

#[derive(Debug)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_equal(&self, t: Tuple) -> bool {
        float_eq(self.x, t.x)
            && float_eq(self.y, t.y)
            && float_eq(self.z, t.z)
            && float_eq(self.w, t.w)
    }

    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    pub fn negate(self) -> Tuple {
        ZERO_VECTOR - self
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();

        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, mult: f32) -> Tuple {
        Tuple {
            x: self.x * mult,
            y: self.y * mult,
            z: self.z * mult,
            w: self.w * mult,
        }
    }
}

impl Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, d: f32) -> Tuple {
        Tuple {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
            w: self.w / d,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 1.0);
        assert_eq!(t.is_point(), true);
        assert_eq!(t.is_vector(), false);
    }

    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 0.0);
        assert_eq!(t.is_point(), false);
        assert_eq!(t.is_vector(), true);
    }

    #[test]
    fn point_factory_creates_point() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        let t = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0,
        };

        assert_eq!(p.is_equal(t), true);
    }

    #[test]
    fn vector_factory_creates_vector() {
        let v = Tuple::vector(4.0, -4.0, 3.0);
        let t = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 0.0,
        };

        assert_eq!(v.is_equal(t), true);
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::point(3., -2., 5.);
        let a2 = Tuple::vector(-2., 3., 1.);

        let result = Tuple::point(1., 1., 6.);

        assert_eq!((a1 + a2).is_equal(result), true);
    }

    #[test]
    fn adding_two_vectors_gives_a_vector() {
        let a1 = Tuple::vector(3., -2., 5.);
        let a2 = Tuple::vector(-2., 3., 1.);

        let result = Tuple::vector(1., 1., 6.);

        assert_eq!((a1 + a2).is_equal(result), true);
    }

    #[test]
    fn subtracting_two_points() {
        let a1 = Tuple::point(3., 2., 1.);
        let a2 = Tuple::point(5., 6., 7.);

        let result = Tuple::vector(-2., -4., -6.);

        assert_eq!((a1 - a2).is_equal(result), true);
    }

    #[test]
    fn subtracting_vector_from_a_point() {
        let a1 = Tuple::point(3., 2., 1.);
        let a2 = Tuple::vector(5., 6., 7.);

        let result = Tuple::point(-2., -4., -6.);

        assert_eq!((a1 - a2).is_equal(result), true);
    }

    #[test]
    fn subtracting_two_vectors() {
        let a1 = Tuple::vector(3., 2., 1.);
        let a2 = Tuple::vector(5., 6., 7.);

        let result = Tuple::vector(-2., -4., -6.);

        assert_eq!((a1 - a2).is_equal(result), true);
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = Tuple::vector(0., 0., 0.);
        let a1 = Tuple::vector(1., -2., 3.);

        let result = Tuple::vector(-1., 2., -3.);

        assert_eq!((zero - a1).is_equal(result), true);
    }

    #[test]
    fn negate_a_vector() {
        let a1 = Tuple::vector(1., -2., 3.);

        let result = Tuple::vector(-1., 2., -3.);

        assert_eq!(a1.negate().is_equal(result), true);
    }

    #[test]
    fn multiple_tuple_by_scalar() {
        let a1 = Tuple::vector(1., -2., 3.);

        let result = Tuple::vector(3.5, -7., 10.5);

        assert_eq!((a1 * 3.5).is_equal(result), true);
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let a1 = Tuple::vector(1., -2., 3.);

        let result = Tuple::vector(0.5, -1., 1.5);

        assert_eq!((a1 / 2.).is_equal(result), true);
    }

    #[test]
    fn magnitude_of_vectors() {
        let a1 = Tuple::vector(1., 0., 0.);
        let a2 = Tuple::vector(0., 1., 0.);
        let a3 = Tuple::vector(0., 0., 1.);

        assert_eq!(a1.magnitude(), 1.);
        assert_eq!(a2.magnitude(), 1.);
        assert_eq!(a3.magnitude(), 1.);

        let m1: f32 = 14.;
        assert_eq!(Tuple::vector(1., 2., 3.).magnitude(), m1.sqrt());
        assert_eq!(Tuple::vector(-1., -2., -3.).magnitude(), m1.sqrt());
    }

    #[test]
    fn normalizing_vector() {
        let a1 = Tuple::vector(4., 0., 0.);

        let result = Tuple::vector(1., 0., 0.);

        assert_eq!(a1.normalize().is_equal(result), true);

        let a2 = Tuple::vector(1., 2., 3.);
        assert_eq!(float_eq(a2.normalize().magnitude(), 1.), true);
    }
}
