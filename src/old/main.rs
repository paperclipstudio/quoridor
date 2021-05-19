#![crate_name = "quoridor"]

mod game;
use game::*;

fn main() {
    let mut game: Game = game::new();
    game.run();
}
