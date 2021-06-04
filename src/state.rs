use crate::*;

/// State for depth-first search of row orderings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    /// Histogram of currently available stock.
    pub stock: Vec<usize>,
    /// List of indices into [State] [choices] for each row layed so far.
    pub used: Vec<usize>,
    /// Number of rows remaining to process.
    pub cap: usize,
    /// List of possible rows to lay.
    pub choices: Vec<RowSoln>,
}

// Macro for deduplicating code of `subtract_row()` and `add_row()` below.
//
// This could be a function rather than a macro; it's kind of a tossup which
// would be more readable.
macro_rules! change_row {
    ($name:ident, $op:tt) => {
        fn $name(&mut self, row: usize) {
            let choices = std::mem::take(&mut self.choices);
            let row = &choices[row];
            for (s, &r) in self.stock.iter_mut().zip(row.iter()) {
                *s $op r;
            }
            self.choices = choices;
        }
    };
}

impl State {
    /// Set up the initial state.
    fn starting() -> Self {
        let stock = starting_stock();
        let used = vec![];
        let cap = DIMS.1;
        let choices = RowState::legal_rows();
        State { stock, used, cap, choices }
    }

    /// Returns `true` iff [row] would use more stock than currently available.
    fn is_invalid_row(&self, row: usize) -> bool {
        for (&s, &r) in self.stock.iter().zip(self.choices[row].iter()) {
            if s < r {
                return true;
            }
        }
        false
    }

    // Remove a row's boards from the current stock.
    change_row!(subtract_row, -=);

    // Place a row's boards into the current stock.
    change_row!(add_row, +=);

    /// Depth-first search for solution state.
    fn dfs(&mut self) -> Option<State> {
        assert_eq!(self.cap + self.used.len(), DIMS.1);
        if self.cap == 0 {
            return Some(self.clone());
        }
        let nchoices = self.choices.len();
        for c in 0..nchoices {
            if self.is_invalid_row(c) {
                continue;
            }
            self.cap -= 1;
            self.used.push(c);
            self.subtract_row(c);
            if let Some(soln) = self.dfs() {
                return Some(soln);
            }
            self.add_row(c);
            let _ = self.used.pop();
            self.cap += 1;
        }
        None
    }

    /// Find a solution to the problem instance through depth-first search.
    pub fn solve() -> Option<State> {
        let mut state = State::starting();
        state.dfs()
    }

    /// Display the current solution in something like a coherent fashion.
    pub fn show_soln(&self) {
        for &c in &self.used {
            for (i, &n) in self.choices[c].iter().enumerate() {
                for _ in 0..n {
                    print!("{} ", BOARDS[i].length as f64 / 10.0);
                }
            }
            println!();
        }
    }
}
