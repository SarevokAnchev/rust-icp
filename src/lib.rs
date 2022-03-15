extern crate rust_kdtree;
extern crate nalgebra as na;

mod icp {
    use nalgebra::Matrix1x2;
    use rust_kdtree::kdtree::*;
    use na::{Vector3, Matrix3xX, Matrix4};

    fn best_transform(fixed: &Matrix3xX<f64>, moving: &Matrix3xX<f64>) -> Matrix4<f64> {
        let mut res: Matrix4<f64> = Matrix4::identity();

        let cf: Vector3<f64> = fixed.column_mean();
        let cm: Vector3<f64> = moving.column_mean();

        // TODO : retrancher centroides à A et B

        let h = moving * fixed.transpose();

        let h_svd = h.svd(true, true);

        let r = h_svd.v_t.unwrap() * h_svd.u.unwrap().transpose();

        let t: Vector3<f64> = cf - r*cm;

        // TODO : remplir matrice résultat
        res
    }

    pub fn icp(fixed: &Matrix3xX<f64>, moving: &Matrix3xX<f64>, max_iterations: usize, tolerance: f64) -> Matrix4<f64> {
        let mut tree: KDTree<usize> = KDTree::new(3);
        for (i, v) in fixed.column_iter().enumerate() {
            tree.add_node(&[*v.index(0), *v.index(1), *v.index(2)], i);
        }
        Matrix4::identity()
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


