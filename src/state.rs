use crate::*;

pub type Soln = Vec<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub stock: Vec<usize>,
    pub used: Vec<usize>,
    pub cap: usize,
    pub choices: Vec<Soln>,
}

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
    fn starting() -> Self {
        let stock = starting_stock();
        let used = vec![];
        let cap = DIMS.1;
        let choices = RowState::legal_rows();
        State { stock, used, cap, choices }
    }

    fn is_invalid_row(&self, row: usize) -> bool {
        for (&s, &r) in self.stock.iter().zip(self.choices[row].iter()) {
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

    pub fn solve() -> Option<State> {
        let mut state = State::starting();
        state.dfs()
    }

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
