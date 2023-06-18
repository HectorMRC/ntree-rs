//! Point implementation

pub trait Point: Default {}

/// Represents a bidimentional point.
#[derive(Default)]
pub struct Point2d([f32; 2]);

impl Point2d {
    pub fn new(x: f32, y: f32) -> Self {
        Point2d([x, y])
    }
}

#[macro_export]
macro_rules! point_2d {
    ($x:expr, $y:expr) => {
        Point::new(x, y)
    };
}

/// Represents a tridimentional point.
#[derive(Default)]
pub struct Point3d([f32; 3]);

impl Point3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3d([x, y, z])
    }
}

#[macro_export]
macro_rules! point_3d {
    ($x:expr, $y:expr) => {
        Point::new(x, y)
    };
}
