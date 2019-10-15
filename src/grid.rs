use std::io::{stdout, Write};

use rayon::prelude::*;

use crate::gamecell::GameCell;

pub struct Grid {
    cells: Vec<Vec<GameCell>>,
    w: usize,
    h: usize,
    interval: u64,
}

impl Grid {
    pub fn new(interval: u64) -> Self {
        let (w, h) = if let Some((w, h)) = term_size::dimensions() {
            (w / 2 - 1, h - 1)
        } else {
            (80, 40)
        };
        let cells = vec![vec![0; w]; h]
            .par_iter()
            .map(|v| {
                v.par_iter()
                    .map(|_| GameCell::new(rand::random()))
                    .collect()
            })
            .collect();
        Self {
            cells,
            w,
            h,
            interval,
        }
    }

    pub fn interval(&self) -> u64 {
        self.interval
    }

    pub fn print(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        stdout().flush().unwrap();
        for row in self.cells.iter() {
            for cell in row {
                if cell.is_alive() {
                    print!("0 ")
                } else {
                    print!("  ")
                }
            }
            println!("|");
        }
    }

    pub fn update_cells(&mut self) {
        self.cells
            .par_iter_mut()
            .for_each(|row| row.par_iter_mut().for_each(|cell| cell.update()));
    }

    pub fn change_cells(&mut self) {
        self.cells.par_iter().enumerate().for_each(|(y, row)| {
            row.par_iter().enumerate().for_each(|(x, cell)| {
                let mut close_cells = 0;

                if y > 0 && self.cells[y - 1][x].is_alive() {
                    close_cells += 1;
                }
                if y < self.h - 1 && self.cells[y + 1][x].is_alive() {
                    close_cells += 1;
                }
                if x > 0 && self.cells[y][x - 1].is_alive() {
                    close_cells += 1;
                }
                if x < self.w - 1 && self.cells[y][x + 1].is_alive() {
                    close_cells += 1;
                }
                if x > 0 && y > 0 && self.cells[y - 1][x - 1].is_alive() {
                    close_cells += 1;
                }
                if x < self.w - 1 && y < self.h - 1 && self.cells[y + 1][x + 1].is_alive() {
                    close_cells += 1;
                }
                if x < self.w - 1 && y > 0 && self.cells[y - 1][x + 1].is_alive() {
                    close_cells += 1;
                }
                if x > 0 && y < self.h - 1 && self.cells[y + 1][x - 1].is_alive() {
                    close_cells += 1;
                }

                if cell.is_alive() && close_cells < 2 || cell.is_alive() && close_cells > 3 {
                    cell.set_will_live(false);
                } else if cell.is_alive() && (close_cells == 2 || close_cells == 3)
                    || !cell.is_alive() && close_cells == 3
                {
                    cell.set_will_live(true);
                }
            })
        });
    }
}
