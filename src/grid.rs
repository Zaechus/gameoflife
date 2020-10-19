use std::{
    io::{stdout, Write},
    sync::{Arc, Mutex},
};

use crossterm::{cursor, execute, style, terminal};

use rayon::prelude::*;

use crate::gamecell::GameCell;

pub struct Grid {
    cells: Vec<Vec<GameCell>>,
    buffer: Arc<Mutex<Vec<Vec<char>>>>,
    render: String,
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
            buffer: Arc::new(Mutex::new(vec![vec![' '; w + 1]; h])),
            render: String::with_capacity((w + 1) * h),
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
            row.par_iter().enumerate().for_each(|(x, cell)| {
                if cell.is_alive() {
                    self.buffer.lock().unwrap()[y][x] = '█';
                } else {
                    self.buffer.lock().unwrap()[y][x] = ' ';
                }
            });
            self.buffer.lock().unwrap()[y][self.w] = '\n'
        });

        self.render = String::with_capacity((self.w + 1) * self.h);
        for row in self.buffer.lock().unwrap().iter() {
            self.render.push_str(&row.iter().collect::<String>());
        }
        execute!(stdout, style::Print(&self.render))?;

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
