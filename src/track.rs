use crate::bosh::BoshPoint;
use crate::line::{Line, LineType};
use crate::vector::Vector2D;
use std::collections::HashMap;

const MAX_FORCE_LENGTH: f64 = 10.0;
const EXTENSION_RATIO: f64 = 0.25;

/// A track in linerider.
#[derive(Clone)]
pub struct Track<'a> {
    pub start: Vector2D,
    pub lines: &'a Vec<Line>,

    hitbox_extensions: HashMap<Line, (f64, f64)>,
}

impl<'a> Track<'a> {
    pub fn new(start: Vector2D, lines: &'a Vec<Line>) -> Track {
        let mut hitbox_extensions: HashMap<Line, (f64, f64)> = HashMap::new();
        for line in lines.iter() {
            if line.line_type == LineType::Scenery {
                continue;
            }
            hitbox_extensions.insert(
                *line,
                Track::calculate_hitbox_extensions_for_line(line, &lines),
            );
        }

        Track {
            start,
            lines,
            hitbox_extensions,
        }
    }

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
    pub fn distance_below_line(&self, line: &Line, point: BoshPoint) -> f64 {
        let (start, end) = line.points;
        let line_vec = end - start;
        let point_from_start = point.location - start;
        let is_moving_into_line = {
            let rot_vec = if line.flipped {
                line_vec.rotate90_left()
            } else {
                line_vec.rotate90_right()
            };
            let dot = rot_vec.dot_product(point.location - point.previous_location);
            dot > 0f64
        };
        if !is_moving_into_line {
            return 0f64;
        }

        let line_length = line_vec.length_squared().sqrt();
        let line_normalized = line_vec / line_length;

        let (ext_l, ext_r) = self.hitbox_extensions.get(line).unwrap_or(&(0f64, 0f64));
        let (ext_l, ext_r) = (*ext_l, *ext_r);
        let point_projected_on_line = point_from_start.dot_product(line_normalized);
        if !(ext_l..=(ext_r + line_length)).contains(&point_projected_on_line) {
            return 0f64;
        }

        let distance_below = point_from_start.dot_product(line_normalized.rotate90_right());
        if 0f64 < distance_below && distance_below < MAX_FORCE_LENGTH {
            distance_below
        } else {
            0f64
        }
    }

    /// Returns the amount that each side's hitbox should be extended by.
    fn calculate_hitbox_extensions_for_line(line: &Line, lines: &Vec<Line>) -> (f64, f64) {
        // number of units to extend by
        let mut p0_extension = 0f64;
        let mut p1_extension = 0f64;
        let length = line.length_squared().sqrt();

        for other in lines {
            if other.line_type == LineType::Scenery {
                continue;
            }
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
