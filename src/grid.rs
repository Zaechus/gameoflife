use std::io::{self, stdout};

use crossterm::{cursor, execute, style, terminal};
use rayon::prelude::*;

use crate::gamecell::GameCell;

#[derive(Default)]
pub(crate) struct Grid {
    cells: Vec<Vec<GameCell>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub(crate) fn new() -> Self {
        let (width, height): (usize, usize) = if let Ok((width, height)) = terminal::size() {
            (width.into(), height.into())
        } else {
            (80, 40)
        };

        let cells = vec![vec![0; width]; height]
            .par_iter()
            .map(|v| {
                v.par_iter()
                    .map(|_| GameCell::new(rand::random()))
                    .collect()
            })
            .collect();

        Self {
            cells,
            width,
            height,
        }
    }

    pub(crate) fn print(&mut self) -> io::Result<()> {
        execute!(
            stdout(),
            cursor::MoveTo(0, 0),
            style::Print(
                self.cells
                    .par_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .par_iter()
                    .map(|c| c.symbol())
                    .collect::<String>()
            )
        )?;

        Ok(())
    }

    pub(crate) fn update_cells(&mut self) {
        self.cells
            .par_iter_mut()
            .for_each(|row| row.par_iter_mut().for_each(|cell| cell.update()));
    }

    pub(crate) fn change_cells(&mut self) {
        self.cells.par_iter().enumerate().for_each(|(y, row)| {
            row.par_iter().enumerate().for_each(|(x, cell)| {
                let (top, bottom) = if y == 0 {
                    (self.height - 1, y + 1)
                } else if y == self.height - 1 {
                    (y - 1, 0)
                } else {
                    (y - 1, y + 1)
                };

                let (left, right) = if x == 0 {
                    (self.width - 1, x + 1)
                } else if x == self.width - 1 {
                    (x - 1, 0)
                } else {
                    (x - 1, x + 1)
                };

                let adjacent_count = self.cells[top][x].state()
                    + self.cells[bottom][x].state()
                    + self.cells[y][left].state()
                    + self.cells[y][right].state()
                    + self.cells[top][left].state()
                    + self.cells[top][right].state()
                    + self.cells[bottom][left].state()
                    + self.cells[bottom][right].state();

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
