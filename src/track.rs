use crate::bosh::MovingPoint;
use crate::line::Line;
use crate::vector::Vector2D;

const MAX_FORCE_LENGTH: f64 = 10.0;
const EXTENSION_RATIO: f64 = 0.25;

/// A track in linerider.
#[derive(Clone)]
pub struct Track {
    pub start: Vector2D,
    pub lines: Vec<Line>,
}

impl Track {
    /// Snaps a point to the nearest point, or returns `to_snap` if
    /// there are no nearby points.
    pub fn snap_point(&self, max_dist: f64, to_snap: Vector2D) -> Vector2D {
        let max_dist_sq = max_dist * max_dist;

        self.lines
            .iter()
            .flat_map(|l| [l.points.0, l.points.1])
            .map(|p| (p, p.distance_squared(to_snap)))
            .filter(|(_, dist)| dist.total_cmp(&max_dist_sq).is_lt())
            .min_by(|(_, d1), (_, d2)| d1.total_cmp(d2))
            .unwrap_or((to_snap, 0f64))
            .0
    }

    /// Returns the distance below the line, or 0 if applicable. "below" is the direction
    /// 90 degrees to the right of the vector created from `self.points.0` to `self.points.1`.
    ///
    /// Returns 0 when:
    ///  * the point is above the line
    ///  * the point is moving "upward"
    ///  * the point is outside of the line, including extensions
    pub fn distance_below_line(&self, line: Line, point: MovingPoint) -> f64 {
        let (start, end) = line.points;
        let line_vec = end - start;
        let diff = point.location - start;
        let is_moving_into_line = {
            let dot = line_vec.rotate90_right().dot_product(point.velocity);
            dot > 0f64
        };
        if !is_moving_into_line {
            return 0f64;
        }

        let line_length = line_vec.length_squared().sqrt();
        let line_normalized = line_vec / line_length;

        let distance_below = diff.length_projected_onto(line_normalized.rotate90_right());

        if 0f64 < distance_below && distance_below < MAX_FORCE_LENGTH {
            distance_below
        } else {
            0f64
        }
    }

    /// Returns the amount that each side's hitbox should be extended by.
    pub fn calculate_hitbox_extensions(&self, line: Line) -> (f64, f64) {
        // number of units to extend by
        let mut p0_extension = 0f64;
        let mut p1_extension = 0f64;
        let length = line.length_squared().sqrt();

        for other in &self.lines {
            if line.points.0 == other.points.0 && line.points.1 == other.points.1 {
                continue;
            }

            // if the left side is connected...
            if line.points.0 == other.points.0 || line.points.0 == other.points.1 {
                p0_extension = f64::min(EXTENSION_RATIO, MAX_FORCE_LENGTH / length);
            }

            // if the right side is connected...
            if line.points.1 == other.points.0 || line.points.1 == other.points.1 {
                p1_extension = f64::min(EXTENSION_RATIO, MAX_FORCE_LENGTH / length);
            }
        }

        (p0_extension, p1_extension)
    }
}
