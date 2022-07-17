use crate::bosh::BoshPoint;
use crate::line::LineType;
use crate::track::Track;

pub fn apply_collisions(point: BoshPoint, track: &Track) -> BoshPoint {
    let mut next_point = point;

    for line in track.lines.iter() {
        let distance_below = track.distance_below_line(*line, next_point);
        if distance_below == 0.0 {
            continue;
        }
        match line.line_type {
            LineType::Normal => {
                next_point.location += line.perpendicular() * distance_below;
            }
            LineType::Accelerate { amount: accel } => {
                next_point.location += line.perpendicular() * distance_below;
                next_point.velocity += line.as_vector2d().normalize() * (0.1 * (accel as f64));
            }
            _ => {}
        }
    }

    next_point
}
