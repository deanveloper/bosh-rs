use crate::game::Line;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default, Debug)]
pub struct StoreIndex(usize);

/// A data structure that holds an unordered list of lines, with duplicates.
#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct RawStore {
    lines: Vec<Line>,
    line_to_index: HashMap<Line, Vec<usize>>,
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

        self.line_to_index.entry(line).or_default().push(idx);

        StoreIndex(idx)
    }

    /// Removes a line from the store.
    ///
    /// If a swap_remove occurred such that the user of the
    /// RawStore should need to update its indices, it returns those indices.
    pub fn remove_line(&mut self, line: Line) -> RemoveLineResult {
        let idxs = self.line_to_index.get_mut(&line);
        if idxs.is_none() {
            return RemoveLineResult::NoneRemoved;
        }
        let idxs = idxs.unwrap();

        // remove the line
        let idx = idxs.swap_remove(0);
        if idxs.is_empty() {
            self.line_to_index.remove_entry(&line);
        }

        self.lines.swap_remove(idx);

        if self.lines.len() == idx {
            return RemoveLineResult::RemovedNoSwap(StoreIndex(idx));
        }

        // since we did a swap_remove, update the line that used to be at lines.len and set it to idx
        let line = self.lines.get(idx).unwrap();
        self.line_to_index
            .get_mut(line)
            .unwrap()
            .iter_mut()
            .for_each(|each_idx| {
                if *each_idx == self.lines.len() {
                    *each_idx = idx
                }
            });

        RemoveLineResult::RemovedAndNeedsSwap {
            from: StoreIndex(self.lines.len()),
            to: StoreIndex(idx),
        }
    }
}

pub enum RemoveLineResult {
    NoneRemoved,
    RemovedAndNeedsSwap { from: StoreIndex, to: StoreIndex },
    RemovedNoSwap(StoreIndex),
}
