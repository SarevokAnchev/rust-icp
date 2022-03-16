extern crate rust_kdtree;
extern crate nalgebra as na;

pub mod icp {
    use std::fmt;
    use rust_kdtree::kdtree::*;
    use na::{Vector3, Matrix3xX, Matrix4, Matrix4xX};

    pub struct Matrix {
        rows: usize,
        cols: usize,
        values: Vec<f64>
    }

    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Matrix {
            Matrix {
                rows: rows,
                cols: cols,
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
    }

    fn best_transform(fixed: &Matrix3xX<f64>, moving: &Matrix3xX<f64>) -> Matrix4<f64> {
        let mut res: Matrix4<f64> = Matrix4::identity();

        let cf: Vector3<f64> = fixed.column_mean();
        let cm: Vector3<f64> = moving.column_mean();

        let mut moving_c: Matrix3xX<f64> = moving.clone_owned();
        let mut fixed_c: Matrix3xX<f64> = fixed.clone_owned();

        for i in 0..fixed.ncols() {
            fixed_c.column_mut(i).copy_from(&(fixed.column(i) - cf));
            moving_c.column_mut(i).copy_from(&(moving.column(i) - cm));
        }

        let h = moving * fixed.transpose();

        let h_svd = h.svd(true, true);

        let r = h_svd.v_t.unwrap() * h_svd.u.unwrap().transpose();

        let t: Vector3<f64> = cf - r*cm;

        res.index_mut((..3, ..3)).copy_from(&r);
        res.index_mut((..3, 3)).copy_from(&t);
        res
    }

    pub fn icp(fixed: Matrix, moving: Matrix, max_iterations: usize, tolerance: f64) -> Result<Matrix, ICPError> {
        let mut tree: KDTree<usize> = KDTree::new(3);
        for i in 0..fixed.cols {
            let c = fixed.get_column(i);
            tree.add_node(&c, i);
        }
        let mut fixed_mat: Matrix4xX<f64> = Matrix4xX::from_element(moving.cols, 1.);
        let mut moving_mat: Matrix4xX<f64> = Matrix4xX::from_element(moving.cols, 1.);
        for i in 0..moving.cols {
            let c = moving.get_column(i);
            moving_mat.index_mut((..3, i)).copy_from_slice(&c);
        }

        for it in 0..max_iterations {
            for (i, c) in moving_mat.column_iter().enumerate() {
                let closest = tree.nearest_neighbor(c.as_slice()).unwrap();
                fixed_mat.index_mut((..3, i)).copy_from_slice(&fixed.get_column(closest));
            }
            


            // best_transform

            // transformation
        }

        let mut tfm: Matrix = Matrix::new(4, 4);
        Ok(tfm)
    }

    #[derive(Clone, Debug)]
    pub struct ICPError {
        pub msg: String,
    }

    impl fmt::Display for ICPError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    }
}


