use crate::bosh::MovingPoint;
use crate::track::Track;
use crate::vector::Vector2D;
use std::ops::Mul;

const MAX_FORCE_LENGTH: f64 = 10.0;
const MAX_EXTENSION_RATIO: f64 = 0.25;

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
    /// Returns the distance below the line, if applicable. "down" is the direction
    /// 90 degrees to the right of the vector created from `self.points.0` to `self.points.1`.
    pub fn distance_below_line(self, point: MovingPoint) -> f64 {
        let (start, end) = self.points;
        let line_vec = end - start;
        let diff = point.location - start;
        let is_moving_into_line = {
            let dot = line_vec.rotate90_right().dot_product(point.velocity);
            dot > 0f64
        };
        if is_moving_into_line {
            return 0f64;
        }

        let line_length = line_vec.length_squared().sqrt();
        let line_normalized = line_vec / line_length;

        let distance_below = diff.length_projected_onto(line_normalized.rotate90_right());

        f64::max(0f64, distance_below)
    }

    /// Returns the amount that each side's hitbox should be extended by.
    pub fn hitbox_extensions(self, lines: &Vec<Line>) -> (f64, f64) {
        let mut p0_extension = 0f64;
        let mut p1_extension = 0f64;
        let length = self.length_squared().sqrt();

        for line in lines {
            if line.points.0 == self.points.0 || line.points.1 == self.points.0 {
                p0_extension = f64::min(MAX_EXTENSION_RATIO, MAX_FORCE_LENGTH / length);
            }
            if line.points.0 == self.points.1 || line.points.1 == self.points.1 {
                p1_extension = f64::min(MAX_EXTENSION_RATIO, MAX_FORCE_LENGTH / length);
            }
        }

        (p0_extension, p1_extension)
    }

    pub fn length_squared(self) -> f64 {
        self.points.0.distance_squared(self.points.1)
    }
}
