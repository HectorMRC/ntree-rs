//! Rect shape implementation

use crate::point::Point;

/// Represents a rect in a bidimentional space.
#[derive(Default)]
pub struct Rect<P: Point>([P; 2]);

impl<P: Point> Rect<P> {
    pub fn new(a: P, b: P) -> Self {
        Rect([a, b])
    }
}

#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr) => {
        Rect::new(x, y)
    };
}
