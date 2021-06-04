pub mod rowstate;
pub mod state;

use rowstate::*;
use state::*;

pub const DIMS: (u64, usize) = (1960, 10);

pub const BOARDS: &[Stock] = &[
    Stock { count: 10, length: 900 },
    Stock { count: 10, length: 450 },
    Stock { count: 20, length: 225 },
    Stock { count: 20, length: 80 },
];

#[derive(Debug)]
pub struct Stock {
    pub count: usize,
    pub length: u64,
}

pub fn starting_stock() -> Vec<usize> {
    BOARDS.iter().map(|b| b.count).collect()
}

fn main() {
    let soln = State::solve();
    match soln {
        Some(s) => s.show_soln(),
        None => println!("no solution"),
    }
}
