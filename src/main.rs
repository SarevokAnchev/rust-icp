use std::fs::File;
use std::io::prelude::*;
use rand::{Rng, thread_rng};

use icp::visualization::visuals3d::draw_registration;
use icp::utils::matrix::Matrix;
use icp::registration::icp::icp;

fn load_data() -> (Matrix, Matrix) {
    let mut fixed_file = File::open("data/test_fixed.json").unwrap();
    let mut fixed_str = String::new();
    fixed_file.read_to_string(&mut fixed_str).unwrap();
    let full_fixed: Matrix = serde_json::from_str(&fixed_str).unwrap();

    let mut fixed = Matrix::new(3, 1000);
    for i in 0usize..1000 {
        fixed.set_column(i, &full_fixed.get_column(thread_rng().gen_range(0..full_fixed.cols())));
    }

    let mut moving_file = File::open("data/test_moving.json").unwrap();
    let mut moving_str = String::new();
    moving_file.read_to_string(&mut moving_str).unwrap();
    let full_moving: Matrix = serde_json::from_str(&moving_str).unwrap();

    let mut moving = Matrix::new(3, 1000);
    for i in 0usize..1000 {
        moving.set_column(i, &full_moving.get_column(thread_rng().gen_range(0..full_moving.cols())));
    }

    (fixed, moving)
}

fn main() {
    let (fixed, moving) = load_data();
    let mut transform = icp(&fixed, &moving, 100, 0.1).unwrap();
    // TODO : apply transform
    draw_registration(&fixed, &moving).unwrap();
}
