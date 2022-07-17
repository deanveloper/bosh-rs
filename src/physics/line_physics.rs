use crate::bosh::MovingPoint;
use crate::line::{Line, LineType};
use crate::track::Track;
use crate::vector::Vector2D;
use std::ops::Mul;

pub fn apply_collisions(point: MovingPoint, track: &Track) -> MovingPoint {
    let mut next_point = point;

    for line in track.lines {
        let distance_below = track.distance_below_line(line, point);
        if distance_below == 0.0 {
            continue;
        }
        match line.line_type {
            LineType::Normal => {
                next_point.location += line.perpendicular() * distance_below;
            }
            LineType::Accelerate { amount: accel } => {
                next_point.location += line.perpendicular() * distance_below;
                next_point.velocity += line.as_vector2d().normalize() * (0.1 * accel.into());
            }
            _ => {}
        }
    }

    next_point
}
