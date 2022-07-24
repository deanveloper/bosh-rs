use crate::vector::Vector2D;
use std::hash::Hash;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum LineType {
    Normal,
    Accelerate { amount: u64 },
    Scenery,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Line {
    pub flipped: bool,
    pub line_type: LineType,
    pub points: (Vector2D, Vector2D),
}

impl Line {
    pub fn as_vector2d(self) -> Vector2D {
        self.points.1 - self.points.0
    }

    /// Returns the perpendicular unit vector for this line, facing "upwards" (the direction
    /// in which it applies force).
    pub fn perpendicular(self) -> Vector2D {
        if self.flipped {
            self.as_vector2d().rotate90_right().normalize()
        } else {
            self.as_vector2d().rotate90_left().normalize()
        }
    }

    pub fn length_squared(self) -> f64 {
        self.points.0.distance_squared(self.points.1)
    }
}
