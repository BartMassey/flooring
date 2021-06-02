const DIMS: (u64, usize) = (1960, 10);

const BOARDS: &[Stock] = &[
    Stock { count: 10, length: 900 },
    Stock { count: 10, length: 450 },
    Stock { count: 20, length: 225 },
    Stock { count: 10, length: 80 },
];

#[derive(Debug)]
struct Stock {
    count: usize,
    length: u64,
}

#[derive(Debug, Clone, Copy)]
struct Cap {
    row: usize,
    col: u64,
}

impl Default for Cap {
    fn default() -> Self {
        Cap { col: DIMS.0, row: DIMS.1 }
    }
}

impl Cap {
    fn is_empty(&self) -> bool {
        self.row == 0 && self.col == 0
    }

    fn advance_row(&mut self) {
        self.col = DIMS.0;
        self.row -= 1;
    }

    fn retract_row(&mut self) {
        assert_eq!(self.col, 0);
        self.col = DIMS.0;
        self.row += 1;
    }    
}
#[derive(Debug, Clone)]
struct State {
    stock: Vec<usize>,
    used: Vec<usize>,
    cap: Cap,
}

#[derive(Debug, Clone)]
struct Soln(Vec<usize>);

impl State {
    fn dfs(&mut self) -> Option<Soln> {
        if self.cap.is_empty() {
            return Some(Soln(self.used.clone()));
        }
        let nstock = self.stock.len();
        for i in 0..nstock {
            if self.stock[i] == 0 {
                continue;
            }
            let length = BOARDS[i].length;
            if length > self.cap.col {
                continue;
            }
            let col_empty = self.cap.col == 0;
            if col_empty {
                self.cap.advance_row();
            }
            self.cap.col -= length;
            self.used.push(i);
            if let Some(soln) = self.dfs() {
                return Some(soln);
            }
            let _ = self.used.pop();
            self.cap.col += length;
            if col_empty {
                self.cap.retract_row();
            }
        }
        None
    }

    fn solve() -> Option<Soln> {
        let mut state = State::default();
        state.dfs()
    }
}

impl Default for State {
    fn default() -> Self {
        let stock = BOARDS.iter().map(|b| b.count).collect();
        State { stock, ..Default::default() }
    }
}

fn main() {
    let soln = State::solve();
    println!("{:?}", soln);
}
