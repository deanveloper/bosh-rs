use crate::bosh::BoshPoint;
use crate::line::LineType;
use crate::track::Track;

pub fn update_boshpoint_position(point: BoshPoint, track: &Track<'_>) -> BoshPoint {
    let mut next_point = point;

    for line in track.lines.iter() {
        if let LineType::Scenery = line.line_type {
            continue;
        }

        let distance_below = track.distance_below_line(line, next_point);
        if distance_below == 0.0 {
            continue;
        }
        let perpendicular = line.perpendicular();

        let next_location = point.location + (perpendicular * distance_below);
        let mut previous_location =
            perpendicular.rotate90_right() * point.friction * distance_below;

        if point.previous_location.0 >= next_location.0 {
            previous_location.0 = -previous_location.0
        }
        if point.previous_location.1 >= next_location.1 {
            previous_location.1 = -previous_location.1
        }
        previous_location += point.previous_location;

        if let LineType::Accelerate { amount: accel } = line.line_type {
            next_point.previous_location += line.as_vector2d().normalize() * (0.1 * (accel as f64));
        }

        next_point.previous_location = previous_location;
        next_point.location = next_location;
    }

    next_point
}
