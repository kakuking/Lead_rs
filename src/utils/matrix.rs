use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Matrix4x4 {
    pub m: Vec<Vec<f32>>
}

impl Matrix4x4 {
    pub fn new() -> Self {
        let mut temp = Self {
            m: vec![vec![0f32; 4]; 4]
        };

        for i in 0..4 {
            temp.m[i][i] = 1f32;
        }

        temp
    }

    pub fn  init(t00: f32, t01: f32, t02: f32, t03: f32, 
                t10: f32, t11: f32, t12: f32, t13: f32, 
                t20: f32, t21: f32, t22: f32, t23: f32, 
                t30: f32, t31: f32, t32: f32, t33: f32) -> Self 
    {
        let m = vec![
            vec![t00, t01, t02, t03],
            vec![t10, t11, t12, t13],
            vec![t20, t21, t22, t23],
            vec![t30, t31, t32, t33]
        ];

        Self {
            m: m
        }
    }

    pub fn  init_copy(m: &Self) -> Self 
    {
        let m = vec![
            m[0].clone(),
            m[1].clone(),
            m[2].clone(),
            m[3].clone(),
        ];

        Self {
            m: m
        }
    }

    pub fn equal(m1: &Self, m2: &Self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if m1[i][j] != m2[i][j] {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn transpose(&self) -> Self {
        Self::init(
            self[0][0], self[0][1], self[0][2], self[0][3],
            self[1][0], self[1][1], self[1][2], self[1][3],
            self[2][0], self[2][1], self[2][2], self[2][3],
            self[3][0], self[3][1], self[3][2], self[3][3]
        )
    }

    pub fn mul(m1: &Self, m2: &Self) -> Self {
        let mut m: Self = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = m1[i][0] * m2[0][j] + m1[i][1] * m2[1][j] + m1[i][2] * m2[2][j] + m1[i][3] * m2[3][j];
            }
        }

        m
    }

    pub fn determinant(&self) -> f32 {
        let m = &self.m;

        // Laplace expansion for 4x4 determinant
        m[0][0] * Self::minor_determinant(&m, 0, 0)
            - m[0][1] * Self::minor_determinant(&m, 0, 1)
            + m[0][2] * Self::minor_determinant(&m, 0, 2)
            - m[0][3] * Self::minor_determinant(&m, 0, 3)
    }

    fn minor_determinant(matrix: &Vec<Vec<f32>>, row: usize, col: usize) -> f32 {
        let mut minor = Vec::new();

        for (i, r) in matrix.iter().enumerate() {
            if i != row {
                let mut new_row = Vec::new();
                for (j, &val) in r.iter().enumerate() {
                    if j != col {
                        new_row.push(val);
                    }
                }
                minor.push(new_row);
            }
        }

        // Compute the determinant of the 3x3 matrix
        let m = &minor;
            m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }

    pub fn inverse(&self) -> Self{
        let det = self.determinant();
        assert!(det > f32::EPSILON, "Non-invertible matrix!");

        let mut ret = Self::new();
        for r in 0..4 {
            for c in 0..4 {
                let cof = Self::minor_determinant(&self.m, r, c) * if (r + c) % 2 == 0 {1f32} else {-1f32};
                ret[r][c] = cof / det;
            }
        }

        ret
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}

// Implement IndexMut for mutable access
impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}

