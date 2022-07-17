use crate::line::Line;
use std::fmt::{Display, Formatter, Pointer};
use std::intrinsics::sqrtf64;

/// Represents a point on a map.
#[derive(Copy, Clone, Default)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(x: {}, y: {})", self.x, self.y)
    }
}

impl Point {
    /// Returns the distance squared between two points. If you need the
    /// actual distance (ie you need to display the value), use [`f64::sqrt`].
    pub fn distance_squared(&self, other: Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        dx * dx + dy * dy
    }
}

/// A track in linerider.
///
/// * `'l`: the lifetime of the vector of lines.
pub struct Track<'l> {
    pub start: Point,
    pub lines: &'l Vec<Line>,
}

impl Track {
    /// Snaps a point to the nearest point, or returns `to_snap` if
    /// there are no nearby points.
    pub fn snap_point(&self, max_dist: f64, to_snap: Point) -> Point {
        let max_dist_sq = max_dist * max_dist;

        self.lines
            .iter()
            .flat_map(|l| [l.points.0, l.points.1])
            .map(|p| (p, p.distance_squared(to_snap)))
            .filter(|(_, dist)| dist < &max_dist_sq)
            .min_by_key(|(_, dist)| dist)
            .unwrap_or_else(|| (to_snap, 0f64))
            .0
    }
}
