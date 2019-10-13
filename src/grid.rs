use std::cell::RefCell;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

use crate::GameCell;

pub struct Grid {
    cells: RefCell<Vec<Vec<Box<GameCell>>>>,
    w: usize,
    h: usize,
    interval: u64,
}

impl Grid {
    pub fn new(interval: u64) -> Self {
        let (w, h) = if let Some((w, h)) = term_size::dimensions() {
            (w / 2, h - 1)
        } else {
            (80, 40)
        };
        let cells = vec![vec![0; w]; h]
            .iter()
            .map(|v| {
                v.iter()
                    .map(|_| Box::new(GameCell::new(rand::random())))
                    .collect()
            })
            .collect();
        Self {
            cells: RefCell::new(cells),
            w,
            h,
            interval,
        }
    }

    fn print(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        stdout().flush().unwrap();
        for row in self.cells.borrow().iter() {
            for cell in row {
                match cell.is_alive() {
                    false => print!("  "),
                    true => print!("0 "),
                };
            }
            println!("|");
        }
    }

    fn update_cells(&self) {
        for row in self.cells.borrow_mut().iter_mut() {
            for cell in row {
                cell.update();
            }
        }
    }

    fn change_cells(&self) {
        for (y, row) in self.cells.borrow().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let mut close_cells = 0;

                if y > 0 && self.cells.borrow()[y - 1][x].is_alive() {
                    close_cells += 1;
                }
                if y < self.h - 1 && self.cells.borrow()[y + 1][x].is_alive() {
                    close_cells += 1;
                }
                if x > 0 && self.cells.borrow()[y][x - 1].is_alive() {
                    close_cells += 1;
                }
                if x < self.w - 1 && self.cells.borrow()[y][x + 1].is_alive() {
                    close_cells += 1;
                }
                if x > 0 && y > 0 && self.cells.borrow()[y - 1][x - 1].is_alive() {
                    close_cells += 1;
                }
                if x < self.w - 1 && y < self.h - 1 && self.cells.borrow()[y + 1][x + 1].is_alive()
                {
                    close_cells += 1;
                }
                if x < self.w - 1 && y > 0 && self.cells.borrow()[y - 1][x + 1].is_alive() {
                    close_cells += 1;
                }
                if x > 0 && y < self.h - 1 && self.cells.borrow()[y + 1][x - 1].is_alive() {
                    close_cells += 1;
                }

                if cell.is_alive() && close_cells < 2 {
                    cell.set_will_live(false);
                } else if cell.is_alive() && close_cells > 3 {
                    cell.set_will_live(false);
                } else if cell.is_alive() && close_cells == 2 || close_cells == 3 {
                    cell.set_will_live(true);
                } else if !cell.is_alive() && close_cells == 3 {
                    cell.set_will_live(true);
                }
            }
        }
    }

    pub fn events(&self) {}

    pub fn play(&self) {
        loop {
            self.print();
            self.change_cells();
            self.update_cells();

            thread::sleep(Duration::from_millis(self.interval));
        }
    }
}
