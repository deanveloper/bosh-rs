use crate::game::LineType;
use crate::game::Track;
use crate::game::Vector2D;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PhysicsPoint {
    pub previous_location: Vector2D,
    pub location: Vector2D,
    pub friction: f64,
}

impl PhysicsPoint {
    pub fn apply_gravity_wells(&mut self, track: &Track) {
        for line in &track.lines_near(self.location) {
            if matches!(line.line_type, LineType::Scenery) {
                continue;
            }

            let distance_below = track.distance_below_line(line, self);
            if distance_below == 0.0 {
                continue;
            }
            let perpendicular = line.perpendicular();

            let next_location = self.location + (perpendicular * distance_below);

            let mut friction_adjustment =
                perpendicular.rotate90_right() * self.friction * distance_below;
            if self.previous_location.0 >= next_location.0 {
                friction_adjustment.0 = -friction_adjustment.0
            }
            if self.previous_location.1 >= next_location.1 {
                friction_adjustment.1 = -friction_adjustment.1
            }

            // adjust the previous location to account for acceleration and
            let next_previous_location = self.previous_location + friction_adjustment;

            if let LineType::Accelerate { amount: accel } = line.line_type {
                let direction = if line.flipped { -1.0 } else { 1.0 };

                self.previous_location +=
                    line.as_vector2d().normalize() * (accel as f64 * 0.1 * direction);
            }

            self.previous_location = next_previous_location;
            self.location = next_location;
        }
    }
}
