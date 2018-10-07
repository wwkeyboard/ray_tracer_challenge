extern crate num;
use std::f32;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

const FLOAT_MARGIN: f32 = 0.000001;
const PPM_COLOR_SIZE: f32 = 255.;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub blue: f32,
    pub green: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn to_ppm(&self) -> String {
        let r = (self.red * PPM_COLOR_SIZE).round();
        let g = (self.green * PPM_COLOR_SIZE).round();
        let b = (self.blue * PPM_COLOR_SIZE).round();

        format!(
            "{} {} {}",
            num::clamp(r, 0., PPM_COLOR_SIZE),
            num::clamp(g, 0., PPM_COLOR_SIZE),
            num::clamp(b, 0., PPM_COLOR_SIZE)
        )
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        float_eq(self.red, other.red)
            && float_eq(self.green, other.green)
            && float_eq(self.blue, other.blue)
    }
}

fn float_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < FLOAT_MARGIN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_tuples() {
        let c1 = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c1.red, -0.5);
        assert_eq!(c1.green, 0.4);
        assert_eq!(c1.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let result = Color::new(1.6, 0.7, 1.0);

        assert_eq!(c1 + c2, result);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let result = Color::new(0.2, 0.5, 0.5);

        assert_eq!(c1 - c2, result);
    }

    #[test]
    fn multiple_color_by_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let result = Color::new(0.4, 0.6, 0.8);

        assert_eq!(c1 * 2., result);
    }

    #[test]
    fn multiply_color_by_color() {
        let c1 = Color::new(1., 0.2, 0.4);
        let c2 = Color::new(0.9, 1., 0.1);

        let result = Color::new(0.9, 0.2, 0.04);

        assert_eq!(c1 * c2, result);
    }

    #[test]
    fn to_ppm() {
        let c1 = Color::new(0.5, -1., 1.5);

        assert_eq!(c1.to_ppm(), "128 0 255");
    }
}
