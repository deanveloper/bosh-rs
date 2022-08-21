use crate::game::LineType;
use crate::game::Track;
use crate::rider::EntityPoint;

pub fn apply_gravity_wells(point: &mut EntityPoint, track: &Track) {
    for line in &track.lines_near(point.location) {
        if matches!(line.line_type, LineType::Scenery) {
            continue;
        }

        let distance_below = track.distance_below_line(line, point);
        if distance_below == 0.0 {
            continue;
        }
        let perpendicular = line.perpendicular();

        let next_location = point.location + (perpendicular * distance_below);

        let mut friction_adjustment =
            perpendicular.rotate90_right() * point.friction * distance_below;
        if point.previous_location.0 >= next_location.0 {
            friction_adjustment.0 = -friction_adjustment.0
        }
        if point.previous_location.1 < next_location.1 {
            friction_adjustment.1 = -friction_adjustment.1
        }
        if let LineType::Accelerate { amount: accel } = line.line_type {
            let direction = if line.flipped { -1.0 } else { 1.0 };

            point.previous_location +=
                line.as_vector2d().normalize() * (accel as f64 * 0.1 * direction);
        }

        point.previous_location += friction_adjustment;
        point.location = next_location;
    }
}
