use std::{io::stdout, process, thread, time::Duration};

use crossterm::{cursor, execute, terminal};

use crate::grid::Grid;

pub fn play(interval: u64) {
    ctrlc::set_handler(move || {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::Show
        )
        .unwrap();
        process::exit(0);
    })
    .unwrap();

    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide
    )
    .unwrap();

    let mut grid = Grid::new();

    loop {
        grid.print().unwrap();

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(interval));
        });

        grid.change_cells();
        grid.update_cells();

        handle.join().unwrap();
    }
}
