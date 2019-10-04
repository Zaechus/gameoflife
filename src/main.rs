use std::io;
use std::io::Write;

use std::thread;
use std::time::Duration;

fn main() {
    let board = vec![
        vec![" ", " ", " ", " ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " ", " ", " ", " "],
        vec![" ", " ", "0", "0", " ", " ", " ", " "],
        vec![" ", " ", " ", "0", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " ", " ", " ", " "],
    ]
    .into_iter()
    .map(|v| v.into_iter().map(|s| String::from(s)));

    for row in board {
        io::stdout().write(b"\r").unwrap();
        for cell in row {
            io::stdout().write(cell.as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        println!();
    }

    thread::sleep(Duration::from_millis(1000));
    println!();
}
