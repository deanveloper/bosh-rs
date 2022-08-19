use std::collections::{HashMap, HashSet};

use crate::game::Line;
use crate::game::Vector2D;
use crate::linestore::raw_store::{RawStore, RemoveLineResult, StoreIndex};

const CELL_SIZE: i64 = 20;

/// Data structure used to query lines nearby the rider in
/// an efficient manner.
#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct Grid {
    lines: RawStore,

    grid: HashMap<GridIndex, Vec<StoreIndex>>,
}

impl Grid {
    pub fn new(lines: &Vec<Line>) -> Grid {
        let mut grid: Grid = Default::default();
        for line in lines {
            grid.add_line(*line);
        }

        grid
    }

    pub fn all_lines(&self) -> &Vec<Line> {
        self.lines.all_lines()
    }

    pub fn lines_near(&self, loc: Vector2D, grid_radius: u8) -> Vec<&Line> {
        self.nearby_line_indices(loc, grid_radius)
            .into_iter()
            .map(|l| self.lines.line_at(l).expect("no line at index"))
            .collect()
    }

    pub fn add_line(&mut self, line: Line) {
        let lines_idx = self.lines.add_line(line);

        for index in GridIndex::iter_over_line(&line) {
            self.grid.entry(index).or_default().push(lines_idx);
        }
    }

    pub fn remove_line(&mut self, line: &Line) {
        // remove lines_idx from grid
        let result = self.lines.remove_line(line);

        match result {
            RemoveLineResult::NoneRemoved => {}
            RemoveLineResult::RemovedNoSwap(idx) => {
                self.remove_line_for_real(line, idx);
            }
            RemoveLineResult::RemovedAndNeedsSwap {
                from: from_idx,
                to: to_idx,
            } => {
                self.remove_line_for_real(line, to_idx);

                // replace instances of line
                if let Some(line) = self.lines.line_at(to_idx) {
                    for grid_idx in GridIndex::iter_over_line(line) {
                        if let Some(idxs) = self.grid.get_mut(&grid_idx) {
                            idxs.iter_mut().for_each(|idx| {
                                if *idx == from_idx {
                                    *idx = to_idx
                                }
                            })
                        }
                    }
                }
            }
        }
    }

    fn remove_line_for_real(&mut self, line: &Line, replaced_idx: StoreIndex) {
        for grid_idx in GridIndex::iter_over_line(line) {
            if let Some(idxs) = self.grid.get_mut(&grid_idx) {
                if let Some(idx_pos) = idxs.iter().position(|idx| *idx == replaced_idx) {
                    idxs.swap_remove(idx_pos);
                }
            }
        }
    }

    fn nearby_line_indices(&self, loc: Vector2D, grid_radius: u8) -> HashSet<StoreIndex> {
        let mut nearby_line_indices: HashSet<StoreIndex> = Default::default();

        let center = GridIndex::from_location(loc);

        let grid_radius = grid_radius as i64;

        for dx in -grid_radius..=grid_radius {
            for dy in -grid_radius..=grid_radius {
                let mut grid_index = center;
                grid_index.0 += dx;
                grid_index.1 += dy;

                if let Some(store_indices) = self.grid.get(&grid_index).cloned() {
                    for store_index in store_indices {
                        nearby_line_indices.insert(store_index);
                    }
                }
            }
        }

        nearby_line_indices
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Default)]
struct GridIndex(i64, i64);

impl GridIndex {
    fn from_location(loc: Vector2D) -> GridIndex {
        GridIndex(
            (loc.0.floor() as i64).div_euclid(CELL_SIZE),
            (loc.1.floor() as i64).div_euclid(CELL_SIZE),
        )
    }

    fn iter_over_line(line: &Line) -> GridIndexLineIter {
        let points = line.ends;

        let furthest_left = [points.0.location, points.1.location]
            .into_iter()
            .min_by(|p1, p2| p1.0.total_cmp(&p2.0))
            .expect("array of two elements has no minimum...?");

        let slope = (points.1.location.1 - points.0.location.1)
            / (points.1.location.0 - points.0.location.0);
        let max_distance = line.length_squared().sqrt();

        GridIndexLineIter {
            current_point: furthest_left,
            slope,
            traveled: 0.0,
            max_distance,
        }
    }
}

struct GridIndexLineIter {
    current_point: Vector2D,
    slope: f64,

    traveled: f64,
    max_distance: f64,
}

impl Iterator for GridIndexLineIter {
    type Item = GridIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.traveled > self.max_distance {
            None
        } else {
            let prev_cell = GridIndex::from_location(self.current_point);

            let x_until_hit =
                CELL_SIZE as f64 - f64_rem_floor(self.current_point.0, CELL_SIZE as f64);
            let y_until_hit = if self.slope >= 0.0 {
                CELL_SIZE as f64 - f64_rem_floor(self.current_point.1, CELL_SIZE as f64)
            } else {
                let result = f64_rem_floor(self.current_point.1, CELL_SIZE as f64);
                if result != 0.0 {
                    result
                } else {
                    CELL_SIZE as f64
                }
            };

            let x_until_vert_border = x_until_hit;
            let x_until_horiz_border = y_until_hit / self.slope.abs();

            let prev_point = self.current_point;
            if x_until_vert_border < x_until_horiz_border {
                // if we hit a vertical border
                self.current_point.0 += x_until_hit;
                self.current_point.1 += x_until_hit * self.slope;
            } else if x_until_horiz_border < x_until_vert_border {
                // if we hit a horizontal border
                self.current_point.0 += x_until_horiz_border;
                if self.slope >= 0.0 {
                    self.current_point.1 += y_until_hit;
                } else {
                    self.current_point.1 -= y_until_hit;
                    self.current_point.1 -= f64::EPSILON * self.current_point.1.abs();
                }
            } else {
                // if we hit a corner
                self.current_point.0 += x_until_hit;
                self.current_point.1 += y_until_hit;
            }

            self.traveled += (self.current_point - prev_point).length_squared().sqrt();

            Some(prev_cell)
        }
    }
}

fn f64_rem_floor(a: f64, b: f64) -> f64 {
    let mut result = a.rem_euclid(b);

    if result < 0.0 {
        result += b;
    }

    result
}
