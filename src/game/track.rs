use std::cell::RefCell;

use physics::advance_frame::frame_after;

use crate::game::line::Line;
use crate::game::vector::Vector2D;
use crate::linestore::grid::Grid;
use crate::physics;
use crate::rider::{Entity, EntityPoint};

pub const GRAVITY_WELL_HEIGHT: f64 = 10.0;

/// A track in linerider.
#[derive(Debug)]
pub struct Track {
    grid: Grid,

    precomputed_rider_positions: RefCell<Vec<Vec<Entity>>>,
}

impl Track {
    pub fn new(starting_positions: &Vec<Entity>, lines: &Vec<Line>) -> Track {
        Track {
            grid: Grid::new(lines),
            precomputed_rider_positions: RefCell::new(vec![starting_positions.clone()]),
        }
    }

    /// Gets all lines in the track.
    pub fn all_lines(&self) -> &Vec<Line> {
        self.grid.all_lines()
    }

    /// Adds a line to the track.
    pub fn add_line(&mut self, line: Line) {
        self.grid.add_line(line);
        self.precomputed_rider_positions.borrow_mut().drain(1..);
    }

    /// Removes a single line from the track.
    pub fn remove_line(&mut self, line: &Line) {
        self.grid.remove_line(line);
        self.precomputed_rider_positions.borrow_mut().drain(1..);
    }

    /// Gets all of the lines near a point.
    pub fn lines_near(&self, point: Vector2D) -> Vec<&Line> {
        self.grid.lines_near(point, 1)
    }

    /// Gets the rider positions for a zero-indexed frame.
    pub fn entity_positions_at(&self, frame: usize) -> Vec<Entity> {
        let mut position_cache = self.precomputed_rider_positions.borrow_mut();
        if let Some(riders) = position_cache.get(frame) {
            riders.clone()
        } else {
            let len = position_cache.len();
            for _ in len..=frame {
                let next_positions = frame_after(position_cache.last().unwrap(), self);
                position_cache.push(next_positions);
            }

            position_cache.last().unwrap().clone()
        }
    }

    /// Adds a new rider to the track.
    pub fn create_entity(&mut self, entity: Entity) {
        let position_cache = self.precomputed_rider_positions.get_mut();
        let initial_frame = position_cache.get_mut(0).unwrap();
        initial_frame.push(entity);

        position_cache.drain(1..);
    }

    /// Removes a rider from the track.
    pub fn remove_entity(&mut self, entity: Entity) -> Option<()> {
        let position_cache = self.precomputed_rider_positions.get_mut();
        let initial_frame = position_cache.get_mut(0).unwrap();
        initial_frame.remove(initial_frame.iter().position(|e| *e == entity)?);

        position_cache.drain(1..);
        Some(())
    }

    /// Snaps a point to the nearest line ending, or returns `to_snap` if
    /// there are no nearby points.
    pub fn snap_point(&self, max_dist: f64, to_snap: Vector2D) -> Vector2D {
        let max_dist_sq = max_dist * max_dist;

        self.lines_near(to_snap)
            .iter()
            .flat_map(|l| [l.ends.0.location, l.ends.1.location])
            .map(|p| (p, p.distance_squared(to_snap)))
            .filter(|(_, dist)| dist.total_cmp(&max_dist_sq).is_lt())
            .min_by(|(_, d1), (_, d2)| d1.total_cmp(d2))
            .unwrap_or((to_snap, 0.0))
            .0
    }

    /// Returns the distance below the line, or 0 if applicable. "below" is the direction
    /// 90 degrees to the right of the vector created from `self.points.0` to `self.points.1`.
    ///
    /// Returns 0 when:
    ///  * the point is above the line
    ///  * the point is moving "upward"
    ///  * the point is outside of the line, including extensions
    pub fn distance_below_line(&self, line: &Line, point: &EntityPoint) -> f64 {
        let line_vec = line.as_vector2d();
        let point_from_start = point.location - line.ends.0.location;
        let perpendicular = line.perpendicular();

        let is_moving_into_line = {
            let dot = perpendicular.dot_product(point.location - point.previous_location);
            dot < 0.0
        };
        if !is_moving_into_line {
            return 0.0;
        }

        let line_length = line_vec.length_squared().sqrt();
        let line_normalized = line_vec / line_length;

        let (ext_l, ext_r) = line.hitbox_extensions();
        let parallel_component = point_from_start.dot_product(line_normalized);
        if parallel_component < ext_l || ext_r + line_length < parallel_component {
            return 0.0;
        }

        let distance_below = (-perpendicular).dot_product(point_from_start);
        if 0.0 < distance_below && distance_below < GRAVITY_WELL_HEIGHT {
            distance_below
        } else {
            0.0
        }
    }
}

impl Clone for Track {
    fn clone(&self) -> Self {
        Track {
            grid: self.grid.clone(),
            precomputed_rider_positions: self.precomputed_rider_positions.clone(),
        }
    }
}
