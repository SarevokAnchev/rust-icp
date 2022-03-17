extern crate rust_kdtree;
extern crate nalgebra as na;
extern crate rand;

pub mod icp {
    use std::fmt;

    use na::{Vector3, Matrix3xX, Matrix4, Matrix4xX};
    use serde::{Serialize, Deserialize};

    use rust_kdtree::kdtree::*;

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

        pub fn set_row(&mut self, r: usize, values: &Vec<f64>) {
            for i in 0..self.cols {
                self.values[i*self.rows + r] = values[i];
            }
        }

        pub fn set_column(&mut self, c: usize, values: &Vec<f64>) {
            for i in 0..self.rows {
                self.values[c*self.rows + i] = values[i];
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
                    disp_string.push_str("\t");
                    for c in 0..self.cols {
                        disp_string.push_str(&format!("{},\t", self.get(r, c)));
                    }
                    disp_string.push_str("\n")
                }
                disp_string.push_str("]");
            }
            write!(f, "{}", disp_string)
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

        let mut f: Matrix3xX<f64> = Matrix3xX::zeros(moving.cols);
        let mut m: Matrix3xX<f64> = Matrix3xX::zeros(moving.cols);
        let mut cur_tfm: Matrix4<f64> = Matrix4::identity();

        let mut error: f64 = 0.;

        for it in 0..max_iterations {
            let mut new_error: f64 = 0.;
            for (i, c) in moving_mat.column_iter().enumerate() {
                let closest = tree.nearest_neighbor(c.index((..3, 0)).as_slice()).unwrap();
                fixed_mat.index_mut((..3, i)).copy_from_slice(&fixed.get_column(closest));
                new_error += tree.get_dist(closest, c.index((..3, 0)).as_slice()).unwrap();
            }
            new_error /= moving_mat.ncols() as f64;
            println!("{} - {}", it, new_error);
            if it != 0 && (error - new_error).abs() < tolerance{
                break;
            }
            error = new_error;

            f.copy_from(&fixed_mat.index((..3, ..)));
            m.copy_from(&moving_mat.index((..3, ..)));
            let best = best_transform(&f, &m);
            cur_tfm = best * cur_tfm;
            moving_mat = best * moving_mat;
        }

        let mut tfm: Matrix = Matrix::new(4, 4);
        for i in 0..3 {
            for j in 0..3 {
                *tfm.get_mut(i, j) = cur_tfm[(i, j)];
            }
            *tfm.get_mut(i, 3) = cur_tfm[(i, 3)];
        }
        *tfm.get_mut(3, 3) = 1.;
        println!("End of optimization. Transform matrix:\n{}", tfm);
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
        use super::{icp, Matrix};

        use std::fs::File;
        use std::io::prelude::*;
        use rand::{Rng, thread_rng};

        fn load_data() -> (Matrix, Matrix) {
            let mut fixed_file = File::open("data/test_fixed.json").unwrap();
            let mut fixed_str = String::new();
            fixed_file.read_to_string(&mut fixed_str).unwrap();
            let full_fixed: Matrix = serde_json::from_str(&fixed_str).unwrap();

            let mut fixed = Matrix::new(3, 1000);
            for i in 0usize..1000 {
                fixed.set_column(i, &full_fixed.get_column(thread_rng().gen_range(0..full_fixed.cols)));
            }

            let mut moving_file = File::open("data/test_moving.json").unwrap();
            let mut moving_str = String::new();
            moving_file.read_to_string(&mut moving_str).unwrap();
            let full_moving: Matrix = serde_json::from_str(&moving_str).unwrap();

            let mut moving = Matrix::new(3, 1000);
            for i in 0usize..1000 {
                moving.set_column(i, &full_moving.get_column(thread_rng().gen_range(0..full_moving.cols)));
            }

            (fixed, moving)
        }

        #[test]
        fn test_icp() {
            let (fixed, moving) = load_data();
            let _tfm = icp(fixed, moving, 50, 0.0005);
        }
    }
}
