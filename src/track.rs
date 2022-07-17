use crate::line::Line;
use std::fmt::{Display, Formatter};
use std::ops::Add;

/// Represents a point on a track. The origin of the track is at { x: 0, y: 0 }.
///
/// While the units are technically arbitrary, they are typically thought of as "pixels".
#[derive(Copy, Clone, Default)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(f64, f64)> for Point {
    fn from(tuple: (f64, f64)) -> Self {
        Point {
            x: tuple.0,
            y: tuple.1,
        }
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
#[derive(Clone)]
pub struct Track {
    pub start: Point,
    pub lines: Vec<Line>,
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
            .filter(|(_, dist)| dist.total_cmp(&max_dist_sq).is_lt())
            .min_by(|(_, d1), (_, d2)| d1.total_cmp(d2))
            .unwrap_or((to_snap, 0f64))
            .0
    }
}
