use color;

#[derive(Debug)]
pub struct Canvas {
    pixels: Vec<Vec<color::Color>>,
    height: usize,
    width: usize,
}

// internally the pixels are stored row-major because of ppm weirdness
impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            height: height,
            width: width,
            pixels: vec![vec![color::Color::new(0., 0., 0.); width]; height],
        }
    }

    // This will panic if x and y are not on the canvas, that means
    // there's been a big mistake
    fn write_pixel(&mut self, x: usize, y: usize, color: color::Color) {
        // pixels is row-major
        self.pixels[y][x] = color;
    }

    // This will panic if x and y are not on the canvas
    fn pixel_at(&self, x: usize, y: usize) -> color::Color {
        // pixels are row major
        self.pixels[y][x]
    }

    fn to_ppm(&self) -> String {
        let mut ppm = String::from("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");

        println!("xes: {}", &self.pixels.len());
        println!("yes: {}", &self.pixels[0].len());
        for y in &self.pixels {
            let mut count = 0;
            for pixel in y {
                // lines can't be longer than 70 chars, push a newline
                // every 6 pixels to prevent this
                if count == 6 {
                    ppm.pop();
                    ppm.push('\n');
                    count = 0;
                }
                ppm.push_str(&pixel.to_ppm());
                ppm.push(' ');
                count = count + 1;
            }
            ppm.pop(); // janky but remove the trailing whitespace
            ppm.push('\n');
        }

        ppm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_canvas() {
        let c = Canvas::new(4000, 2000);

        assert_eq!(c.width, 4000);
        assert_eq!(c.height, 2000);

        assert_eq!(c.pixels[10][500], color::Color::new(0., 0., 0.))
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let r1 = color::Color::new(1., 0., 0.);

        c.write_pixel(2, 3, r1);

        assert_eq!(c.pixel_at(2, 3), r1);
    }

    #[test]
    fn proper_ppm_header() {
        let c = Canvas::new(3, 6);

        let ppm = c.to_ppm();
        let mut lines = ppm.lines();

        assert_eq!(lines.next(), Some("P3"));
        assert_eq!(lines.next(), Some("3 6"));
        assert_eq!(lines.next(), Some("255"));
    }

    #[test]
    fn add_pixel_data_to_ppm() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, color::Color::new(1.5, 0., 0.));
        c.write_pixel(2, 1, color::Color::new(0., 0.5, 0.));
        c.write_pixel(4, 2, color::Color::new(-0.5, 0., 1.));

        let ppm = c.to_ppm();
        let mut lines = ppm.lines();

        lines.next();
        lines.next();
        lines.next();

        assert_eq!(lines.next(), Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"));
        assert_eq!(lines.next(), Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"));
        assert_eq!(lines.next(), Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));
    }
}
