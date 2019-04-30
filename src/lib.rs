use nalgebra as na;

pub trait HasColor {
    fn color(&self) -> na::Point3<f32>;
}
