use gameoflife_cli::ConwaysGame;

fn main() {
    let game = ConwaysGame::new(100);

    game.play()
}
