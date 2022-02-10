use std::{cell::RefCell, io::stdout, process, thread, time::Duration};

use crossterm::{cursor, execute, terminal};

use crate::grid::Grid;

pub struct ConwaysGame {
    grid: RefCell<Grid>,
}

impl ConwaysGame {
    pub fn new(interval: u32) -> Self {
        Self {
            grid: RefCell::new(Grid::new(interval)),
        }
    }

    pub fn play(&self) {
        ctrlc::set_handler(move || {
            execute!(
                stdout(),
                terminal::Clear(terminal::ClearType::All),
                cursor::Show
            )
            .unwrap();
            process::exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        loop {
            self.grid.borrow_mut().print().unwrap();
            self.grid.borrow_mut().change_cells();
            self.grid.borrow_mut().update_cells();

            thread::sleep(Duration::from_millis(self.grid.borrow().interval() as u64));
        }
    }
}
