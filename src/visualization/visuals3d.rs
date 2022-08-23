extern crate kiss3d;

use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::light::Light;
use nalgebra::Translation3;

use std::fmt;

use crate::utils::matrix::Matrix;

pub fn draw_registration(fixed: &Matrix, moving: &Matrix) -> Result<(), VisualError> {
    let mut window = Window::new("Registration");

    let mut fixed_nodes = Vec::<SceneNode>::with_capacity(fixed.cols());
    for i in 0..fixed.cols() {
        let translation = Translation3::new(fixed.get(0, i) as f32, fixed.get(1, i) as f32, fixed.get(2, i) as f32);
        let mut sphere = window.add_sphere(1.);
        sphere.prepend_to_local_translation(&translation);
        sphere.set_color(0.0, 1.0, 0.0);
        fixed_nodes.push(sphere);
    }

    let mut moving_nodes = Vec::<SceneNode>::with_capacity(moving.cols());
    for i in 0..moving.cols() {
        let translation = Translation3::new(moving.get(0, i) as f32, moving.get(1, i) as f32, moving.get(2, i) as f32);
        let mut sphere = window.add_sphere(1.);
        sphere.prepend_to_local_translation(&translation);
        sphere.set_color(1.0, 0.0, 1.0);
        moving_nodes.push(sphere);
    }
    
    window.set_light(Light::StickToCamera);
    while window.render() {}

    Ok(())
}

#[derive(Clone, Debug)]
pub struct VisualError {
    pub msg: String,
}

impl fmt::Display for VisualError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}