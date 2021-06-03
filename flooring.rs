use std::collections::HashSet;

const DIMS: (u64, usize) = (1960, 10);

const BOARDS: &[Stock] = &[
    Stock { count: 10, length: 900 },
    Stock { count: 10, length: 450 },
    Stock { count: 20, length: 225 },
    Stock { count: 20, length: 80 },
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct RowState {
    solns: HashSet<Soln>,
    cap: u64,
    used: Vec<usize>,
    stock: Vec<usize>,
}

#[derive(Debug)]
struct Stock {
    count: usize,
    length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    stock: Vec<usize>,
    used: Vec<usize>,
    cap: usize,
    choices: Vec<Soln>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Soln(Vec<usize>);

fn starting_stock() -> Vec<usize> {
    BOARDS.iter().map(|b| b.count).collect()
}

fn starting_used() -> Vec<usize> {
    vec![0; BOARDS.len()]
}

impl Default for RowState {
    fn default() -> Self {
        RowState {
            solns: HashSet::default(),
            cap: DIMS.0,
            used: starting_used(),
            stock: starting_stock(),
        }
    }
}

impl RowState {
    fn legal_rows() -> Vec<Soln> {
        let mut state = RowState::default();
        state.dfs();
        let mut solns: Vec<Soln> = state.solns.into_iter().collect();
        solns.sort_by_key(Soln::boards_used);
        solns
    }

    fn dfs(&mut self) {
        if self.cap == 0 {
            self.solns.insert(Soln(self.used.clone()));
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

impl Soln {
    fn boards_used(&self) -> usize {
        self.0.iter().cloned().sum()
    }
}

macro_rules! change_row {
    ($name:ident, $op:tt) => {
        fn $name(&mut self, row: usize) {
            let choices = std::mem::take(&mut self.choices);
            let row = &choices[row].0;
            for (s, &r) in self.stock.iter_mut().zip(row.iter()) {
                *s $op r;
            }
            self.choices = choices;
        }
    };
}

impl State {
    fn starting() -> Self {
        let stock = starting_stock();
        let used = vec![];
        let cap = DIMS.1;
        let choices = RowState::legal_rows();
        State { stock, used, cap, choices }
    }

    fn is_invalid_row(&self, row: usize) -> bool {
        for (&s, &r) in self.stock.iter().zip(self.choices[row].0.iter()) {
            if s < r {
                return true;
            }
        }
        false
    }

    change_row!(subtract_row, -=);
    change_row!(add_row, +=);

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

    fn solve() -> Option<State> {
        let mut state = State::starting();
        state.dfs()
    }

    fn show_soln(&self) {
        for &c in &self.used {
            for (i, &n) in self.choices[c].0.iter().enumerate() {
                for _ in 0..n {
                    print!("{} ", BOARDS[i].length as f64 / 10.0);
                }
            }
            println!();
        }
    }
}

fn main() {
    let soln = State::solve();
    match soln {
        Some(s) => s.show_soln(),
        None => println!("no solution"),
    }
}
