pub mod rowstate;
pub mod state;

use rowstate::*;
use state::*;

/// The instance floor dimensions. the row length is given in *mm* to avoid floating-point. The
/// number of rows is given instead of the height of the floor to avoid superfluous units.
pub const DIMS: (u64, usize) = (1960, 10);

/// The stock of boards available.
pub const BOARDS: &[Stock] = &[
    Stock {
        count: 10,
        length: 900,
    },
    Stock {
        count: 10,
        length: 450,
    },
    Stock {
        count: 20,
        length: 225,
    },
    Stock {
        count: 20,
        length: 80,
    },
];

/// Description of a single bin of boards.
#[derive(Debug)]
pub struct Stock {
    pub count: usize,
    pub length: u64,
}

/// The initial stock for the instance, as number of boards in each bin.
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
