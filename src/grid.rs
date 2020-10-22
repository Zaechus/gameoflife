use std::{
    io::{stdout, Write},
    sync::{Arc, Mutex},
};

use crossterm::{cursor, execute, style, terminal};

use rayon::prelude::*;

use crate::gamecell::GameCell;

pub struct Grid {
    cells: Vec<Vec<GameCell>>,
    buffer: Arc<Mutex<Vec<char>>>,
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
            buffer: Arc::new(Mutex::new(vec![' '; (w + 1) * h])),
            w,
            h,
            interval,
        }
    }

    pub fn interval(&self) -> u32 {
        self.interval
    }

    pub fn print(&mut self) -> crossterm::Result<()> {
        let mut stdout = stdout();

        execute!(stdout, cursor::MoveTo(0, 0))?;
        self.cells.par_iter().enumerate().for_each(|(y, row)| {
            let offset = y * self.w + y;
            row.par_iter().enumerate().for_each(|(x, cell)| {
                self.buffer.lock().unwrap()[offset + x] = cell.symbol();
            });
            self.buffer.lock().unwrap()[y * self.w + self.w + y] = '\n';
        });

        if let Ok(rendered) = self.buffer.lock() {
            execute!(stdout, style::Print(rendered.iter().collect::<String>()))?;
        }

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
                let (top, bottom) = if y as i32 - 1 < 0 {
                    (self.h - 1, y + 1)
                } else if y + 1 >= self.h {
                    (y - 1, 0)
                } else {
                    (y - 1, y + 1)
                };

                let (left, right) = if x as i32 - 1 < 0 {
                    (self.w - 1, x + 1)
                } else if x + 1 >= self.w {
                    (x - 1, 0)
                } else {
                    (x - 1, x + 1)
                };

                let adjacent_count: u8 = [
                    self.cells[top][x].state(),
                    self.cells[bottom][x].state(),
                    self.cells[y][left].state(),
                    self.cells[y][right].state(),
                    self.cells[top][left].state(),
                    self.cells[top][right].state(),
                    self.cells[bottom][left].state(),
                    self.cells[bottom][right].state(),
                ]
                .iter()
                .sum();

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
