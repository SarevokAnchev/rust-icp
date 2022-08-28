use std::fmt;

use serde::{Serialize, Deserialize};

pub trait SimpleVector {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> Result<f64, MatError>;
    fn get_mut(&mut self, i: usize) -> Result<&mut f64, MatError>;
}

#[derive(Serialize, Deserialize)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    values: Vec<f64>
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            values: vec![0.; rows*cols]
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, r: usize, c: usize) -> f64 {
        self.values[self.rows*c + r]
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> &mut f64 {
        &mut self.values[self.rows*c + r]
    }

    pub fn get_column(&self, c: usize) -> Vec<f64> {
        let mut ret_vec: Vec<f64> = Vec::new();
        for i in self.rows*c..self.rows*c + self.rows {
            ret_vec.push(self.values[i]);
        }
        ret_vec
    }

    pub fn get_row(&self, r: usize) -> Vec<f64> {
        let mut ret_vec: Vec<f64> = Vec::new();
        for i in 0..self.cols {
            ret_vec.push(self.values[i*self.rows + r]);
        }
        ret_vec
    }

    pub fn set_row(&mut self, r: usize, values: &[f64]) {
        for i in 0..self.cols {
            self.values[i*self.rows + r] = values[i];
        }
    }

    pub fn set_column(&mut self, c: usize, values: &[f64]) {
        for i in 0..self.rows {
            self.values[c*self.rows + i] = values[i];
        }
    }

    pub fn dot(&self, other: &Matrix) -> Result<Matrix, MatError> {
        if self.cols != other.rows {
            return Err(MatError{msg: String::from("Invalid matrix size.")});
        }
        let mut res = Matrix::new(self.rows, other.cols);
        for r in 0..res.rows {
            for c in 0..res.cols {
                *res.get_mut(r, c) = 0.;
                for k in 0..self.cols {
                    *res.get_mut(r, c) += self.get(r, k) * other.get(k, c);
                }
            }
        }
        Ok(res)
    }

    pub fn mean_col(&self) -> Matrix {
        let mut ret_mat = Matrix::new(self.rows, 1);
        for c in 0..self.cols {
            for r in 0..self.rows {
                *ret_mat.get_mut(r, 0) += self.get(r, c);
            }
        }
        for r in 0..self.rows {
            *ret_mat.get_mut(r, 0) /= self.cols as f64;
        }
        ret_mat
    }

    pub fn mean_row(&self) -> Matrix {
        let mut ret_mat = Matrix::new(1, self.cols);
        for r in 0..self.rows {
            for c in 0..self.cols {
                *ret_mat.get_mut(0, c) += self.get(r, c);
            }
        }
        for c in 0..self.cols {
            *ret_mat.get_mut(0, c) /= self.rows as f64;
        }
        ret_mat
    }

    pub fn add_col(&mut self, other: &dyn SimpleVector) -> Result<(), MatError> {
        if other.size() != self.rows {
            return Err(MatError{msg: String::from("Invalid vector size.")});
        }
        for c in 0..self.cols {
            for r in 0..self.rows {
                *self.get_mut(r, c) += other.get(r).unwrap();
            }
        }
        Ok(())
    }

    pub fn minus(&mut self) {
        for v in self.values.iter_mut() {
            *v = -*v;
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp_string: String = format!("Matrix of size ({}, {})\n", self.rows, self.cols);
        if self.rows * self.cols > 100 {
            disp_string.push_str("[ ... ]\n");
        }
        else {
            disp_string.push_str("[\n");
            for r in 0..self.rows {
                disp_string.push('\t');
                for c in 0..self.cols {
                    disp_string.push_str(&format!("{},\t", self.get(r, c)));
                }
                disp_string.push('\n')
            }
            disp_string.push(']');
        }
        write!(f, "{}", disp_string)
    }
}

impl SimpleVector for Matrix {
    fn size(&self) -> usize {
        self.rows
    }

    fn get(&self, i: usize) -> Result<f64, MatError> {
        if self.rows <= i {
            return Err(MatError{msg: String::from("Index error.")});
        }
        Ok(self.get(i, 0))
    }

    fn get_mut(&mut self, i: usize) -> Result<&mut f64, MatError> {
        if self.rows <= i {
            return Err(MatError{msg: String::from("Index error.")});
        }
        Ok(self.get_mut(i, 0))
    }
}

impl SimpleVector for Vec<f64> {
    fn size(&self) -> usize {
        self.len()
    }

    fn get(&self, i: usize) -> Result<f64, MatError> {
        if self.len() <= i {
            return Err(MatError{msg: String::from("Index error.")});
        }
        Ok(self[i])
    }

    fn get_mut(&mut self, i: usize) -> Result<&mut f64, MatError> {
        if self.len() <= i {
            return Err(MatError{msg: String::from("Index error.")});
        }
        Ok(&mut self[i])
    }
}

#[derive(Clone, Debug)]
pub struct MatError {
    pub msg: String,
}

impl fmt::Display for MatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
