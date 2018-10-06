use std::ops::Add;

const FLOAT_MARGIN: f32 = 0.0000001;

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
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 1.0,
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
}
