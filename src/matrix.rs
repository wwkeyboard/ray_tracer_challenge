#[derive(Debug)]
pub struct Matrix {
    m: [[f32; 4]; 4],
}

impl Matrix {
    pub fn new(m: [[f32; 4]; 4]) -> Matrix {
        Matrix { m: m }
    }

    // for now this throws an index bounds panic if x,y is off the matrix
    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.m[x][y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_4x4_matrix() {
        let m = Matrix::new([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m.get(0, 0), 1.);
    }
}
