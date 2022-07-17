use crate::bosh::MovingPoint;
use crate::track::Track;
use crate::vector::Vector2D;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub enum LineType {
    Normal,
    Accelerate { amount: u64 },
    Scenery,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub line_type: LineType,
    pub points: (Vector2D, Vector2D),
}

impl Line {
    pub fn length_squared(self) -> f64 {
        self.points.0.distance_squared(self.points.1)
    }
}
