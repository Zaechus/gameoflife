use std::cell::RefCell;
use std::thread;
use std::time::Duration;

use crate::grid::Grid;

pub struct ConwaysGame {
    grid: RefCell<Grid>,
}

impl ConwaysGame {
    pub fn new(interval: u64) -> Self {
        Self {
            grid: RefCell::new(Grid::new(interval)),
        }
    }

    pub fn play(&self) {
        loop {
            self.grid.borrow().print();
            self.grid.borrow_mut().change_cells();
            self.grid.borrow_mut().update_cells();

            thread::sleep(Duration::from_millis(self.grid.borrow().interval()));
        }
    }
}
