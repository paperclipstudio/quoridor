mod board;
mod game;
use dialoguer::{theme::{ColorfulTheme}, Input, Select};

use std::process::Command;

fn main() {
    start_game();
}

fn clear_screen() {
    assert!(Command::new("clear").status().unwrap().success());
}

fn start_game() {
    let mut game = game::Quoridor::new_two_player();
    game.play(game::Turn::PlaceWall((3,7), board::Orientation::Vertical));
    game.play(game::Turn::PlaceWall((5,4), board::Orientation::Horizontal));
    game.play(game::Turn::PlaceWall((0,0), board::Orientation::Vertical));
    game.play(game::Turn::PlaceWall((1,1), board::Orientation::Vertical));
    game.play(game::Turn::PlaceWall((2,2), board::Orientation::Vertical));
    while !game.has_won() {
        clear_screen();
        println!("Quoridor Game");
        print!("{}", game.to_string());
        let items = vec!["Move Pawn", "Place Wall"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .clear(true)
            .default(0)
            .items(&items[..])
            .interact_opt()
            .unwrap();
    }
}