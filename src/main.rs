use std::{env, process};

fn main() {
    let interval = if let Some(arg) = env::args().nth(1) {
        if arg == "help" {
            println!("Usage: gameoflife [INTERVAL]");
            process::exit(0);
        } else if let Ok(n) = arg.parse() {
            n
        } else {
            128
        }
    } else {
        128
    };

    gameoflife::play(interval)
}
