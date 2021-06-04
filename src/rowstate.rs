use std::collections::HashSet;

use crate::*;

/// Convenience type for a row solution. A row solution is a histogram of the number of boards
/// used from each stock bin.
pub type RowSoln = Vec<usize>;

/// Search state for row search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowState {
    /// Solutions found so far.
    pub solns: HashSet<RowSoln>,
    /// Remaining row capacity.
    pub cap: u64,
    /// Histogram of used stock so far.
    pub used: Vec<usize>,
    /// Histogram of currently available stock.
    pub stock: Vec<usize>,
}

impl Default for RowState {
    fn default() -> Self {
        RowState {
            solns: HashSet::default(),
            cap: DIMS.0,
            used: vec![0; BOARDS.len()],
            stock: starting_stock(),
        }
    }
}

impl RowState {
    /// Return a list of row solutions ordered from least boards to most.
    pub fn legal_rows() -> Vec<RowSoln> {

        #[allow(clippy::ptr_arg)]
        fn boards_used(s: &RowSoln) -> usize {
            s.iter().cloned().sum()
        }

        let mut state = RowState::default();
        state.dfs();
        let mut solns: Vec<RowSoln> = state.solns.into_iter().collect();
        solns.sort_by_key(boards_used);
        solns
    }

    /// Find row solutions using depth-first search. This may find duplicates, but the set
    /// representation takes care of deduplication.
    fn dfs(&mut self) {
        if self.cap == 0 {
            self.solns.insert(self.used.clone());
            return;
        }
        let nstock = self.stock.len();
        #[allow(clippy::needless_range_loop)]
        for i in 0..nstock {
            if self.stock[i] == 0 {
                continue;
            }
            let length = BOARDS[i].length;
            if length > self.cap {
                continue;
            }
            self.stock[i] -= 1;
            self.cap -= length;
            self.used[i] += 1;
            self.dfs();
            self.used[i] -= 1;
            self.cap += length;
            self.stock[i] += 1;
        }
    }
}
