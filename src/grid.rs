use std::io::{stdout, Write};

use crossterm::{cursor, execute, queue, style, terminal};

use rayon::prelude::*;

use crate::gamecell::GameCell;

pub struct Grid {
    cells: Vec<Vec<GameCell>>,
    w: usize,
    h: usize,
    interval: u32,
}

impl Grid {
    pub fn new(interval: u32) -> Self {
        let (w, h): (usize, usize) = if let Ok((w, h)) = terminal::size() {
            ((w - 1).into(), (h - 1).into())
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

        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide
        )
        .unwrap();
        Self {
            cells,
            w,
            h,
            interval,
        }
    }

    pub fn interval(&self) -> u32 {
        self.interval
    }

    pub fn print(&self) -> crossterm::Result<()> {
        let mut stdout = stdout();

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.is_alive() {
                    queue!(
                        stdout,
                        cursor::MoveTo(x as u16, y as u16),
                        style::Print("█")
                    )?;
                } else {
                    queue!(
                        stdout,
                        cursor::MoveTo(x as u16, y as u16),
                        style::Print(" ")
                    )?;
                }
            }
        }
        stdout.flush()?;
        Ok(())
    }

    pub fn update_cells(&mut self) {
        self.cells
            .par_iter_mut()
            .for_each(|row| row.par_iter_mut().for_each(|cell| cell.update()));
    }

    pub fn change_cells(&mut self) {
        self.cells.par_iter().enumerate().for_each(|(y, row)| {
            row.par_iter().enumerate().for_each(|(x, cell)| {
                let mut adjacent_count = 0;

                if y > 0 && self.cells[y - 1][x].is_alive() {
                    adjacent_count += 1;
                }
                if y < self.h - 1 && self.cells[y + 1][x].is_alive() {
                    adjacent_count += 1;
                }
                if x > 0 && self.cells[y][x - 1].is_alive() {
                    adjacent_count += 1;
                }
                if x < self.w - 1 && self.cells[y][x + 1].is_alive() {
                    adjacent_count += 1;
                }
                if x > 0 && y > 0 && self.cells[y - 1][x - 1].is_alive() {
                    adjacent_count += 1;
                }
                if x < self.w - 1 && y < self.h - 1 && self.cells[y + 1][x + 1].is_alive() {
                    adjacent_count += 1;
                }
                if x < self.w - 1 && y > 0 && self.cells[y - 1][x + 1].is_alive() {
                    adjacent_count += 1;
                }
                if x > 0 && y < self.h - 1 && self.cells[y + 1][x - 1].is_alive() {
                    adjacent_count += 1;
                }

                if cell.is_alive() && adjacent_count < 2 || cell.is_alive() && adjacent_count > 3 {
                    cell.set_will_live(false);
                } else if cell.is_alive() && (adjacent_count == 2 || adjacent_count == 3)
                    || !cell.is_alive() && adjacent_count == 3
                {
                    cell.set_will_live(true);
                }
            })
        });
    }
}
