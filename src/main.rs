use std::{env, process};

use gameoflife::ConwaysGame;

fn main() {
    let speed: u32 = if let Some(arg) = env::args().nth(1) {
        if arg == "help" {
            println!("Usage: gameoflife [INTERVAL]");
            process::exit(0);
        } else if let Ok(n) = arg.parse() {
            n
        } else {
            100
        }
    } else {
        100
    };

    let game = ConwaysGame::new(speed);

    game.play()
}
