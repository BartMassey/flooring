use std::collections::HashSet;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowState {
    pub solns: HashSet<Soln>,
    pub cap: u64,
    pub used: Vec<usize>,
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
    pub fn legal_rows() -> Vec<Soln> {

        #[allow(clippy::ptr_arg)]
        fn boards_used(s: &Soln) -> usize {
            s.iter().cloned().sum()
        }

        let mut state = RowState::default();
        state.dfs();
        let mut solns: Vec<Soln> = state.solns.into_iter().collect();
        solns.sort_by_key(boards_used);
        solns
    }

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
