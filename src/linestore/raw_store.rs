use crate::line::Line;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default, Debug)]
pub struct StoreIndex(usize);

/// A data structure that holds an unordered list of lines, with duplicates,
/// without
#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct RawStore {
    lines: Vec<Line>,
    indices_of_line: HashMap<Line, Vec<usize>>,
}

impl RawStore {
    /// Returns all lines with their indices
    pub fn all_lines(&self) -> &Vec<Line> {
        &self.lines
    }

    pub fn line_at(&self, idx: StoreIndex) -> Option<Line> {
        self.lines.get(idx.0).copied()
    }

    /// Returns the index of the added line
    pub fn add_line(&mut self, line: Line) -> StoreIndex {
        self.lines.push(line);
        let idx = self.lines.len() - 1;

        self.indices_of_line.entry(line).or_default().push(idx);

        StoreIndex(idx)
    }

    /// Returns the index of the removed line
    pub fn remove_line(&mut self, line: Line) -> Option<(StoreIndex, StoreIndex)> {
        let mut idxs = self.indices_of_line.get_mut(&line)?;

        // remove the line
        let idx = idxs.swap_remove(0);
        if idxs.len() == 0 {
            self.indices_of_line.remove_entry(&line);
        }

        self.lines.swap_remove(idx);

        // since we did a swap_remove, update the line that used to be at lines.len and set it to idx
        let line = self.lines.get(idx).unwrap();
        self.indices_of_line
            .get_mut(line)
            .unwrap()
            .iter_mut()
            .for_each(|each_idx| {
                if *each_idx == self.lines.len() {
                    *each_idx = idx
                }
            });

        Some((StoreIndex(idx), StoreIndex(self.lines.len())))
    }
}
