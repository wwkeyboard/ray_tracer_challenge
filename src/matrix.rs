#[derive(Debug)]
pub struct Matrix {
    m: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(m: Vec<Vec<f32>>) -> Matrix {
        let mut mat = Matrix { m: vec![vec![0.;4]; 4]};

        let mut row_num = 0;
        for row in m.iter() {
            let mut col_num = 0;
            for col in row.iter() {
                mat.m[row_num][col_num] = *col;
                col_num += 1;
            }
            row_num += 1;
        }
        mat
    }

    // for now this throws an index bounds panic if x,y is off the matrix
    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.m[x][y]
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.m.len(), self.m[0].len())
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.dimensions() != other.dimensions() {
            return false
        }

        for x in 0..self.m.len() {
            for y in 0..self.m[0].len() {
                if self.get(x,y) != other.get(x,y) {
                    return false
                }
            }
        }

        return true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_4x4_matrix() {
        let m = Matrix::new(vec![
            vec![1., 2., 3., 4.],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9., 10., 11., 12.],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m.get(0, 0), 1.);
        assert_eq!(m.get(0,3), 4.);
        assert_eq!(m.get(3,2), 15.5);
    }

    #[test]
    fn new_3x3_matrix() {
        let m = Matrix::new(vec![
            vec![1., 2., 3.],
            vec![5.5, 6.5, 7.5],
            vec![9., 10., 11.],
        ]);

        assert_eq!(m.get(0, 0), 1.);
        assert_eq!(m.get(0,2), 3.);
        assert_eq!(m.get(2,1), 10.);
    }

    #[test]
    fn new_2x2_matrix() {
        let m = Matrix::new(vec![
            vec![1., 2.],
            vec![5.5, 6.5],
        ]);

        assert_eq!(m.get(0, 0), 1.);
        assert_eq!(m.get(0,1), 2.);
        assert_eq!(m.get(1,1), 6.5);
    }

    #[test]
    fn compare_matrices(){
        let m1 = Matrix::new(vec![
            vec![1., 2.],
            vec![3., 4.],
        ]);

        let m2 = Matrix::new(vec![
            vec![1., 2.],
            vec![3., 4.],
        ]);

        let m3 = Matrix::new(vec![
            vec![1., 2.],
            vec![5., 6.],
        ]);

        let m4 = Matrix::new(vec![
            vec![1., 2., 3.],
            vec![1., 2., 3.],
            vec![1., 2., 3.],
        ]);

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
        assert_ne!(m1, m4);
    }
}
