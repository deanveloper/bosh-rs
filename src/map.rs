use crate::line::Line;
use std::intrinsics::sqrtf64;

/// Represents a point on a map.
#[derive(Copy, Clone, Default)]
pub struct Point {
    x: f64,
    y: f64,
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

impl<'l> Track<'l> {
    pub fn snap_point(&self, to_snap: Point) -> Point {
        let min_point = self
            .lines
            .iter()
            .flat_map(|l| [l.points.0, l.points.1])
            .map(|p| (p, p.distance_squared(to_snap)))
            .min_by_key(|(p, dist)| dist)
            .unwrap_or_else((Point::default(), f64::INFINITY));

        if min_point.1 < 5.0 {
            min_point.0
        } else {
            to_snap
        }
    }
}
