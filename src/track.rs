use crate::line::Line;
use crate::vector::Vector2D;
use std::fmt::{Display, Formatter};
use std::ops::Add;

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
}
