use std::ops::Add;
use std::ops::Sub;

const FLOAT_MARGIN: f32 = 0.000001;

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
}
