mod board;
mod game;
use dialoguer::{theme::ColorfulTheme, Select};

use std::process::Command;

fn main() {
    start_game();
}

fn clear_screen() {
    assert!(Command::new("clear").status().unwrap().success());
}

fn start_game() {
    let mut game = game::Quoridor::new_two_player();
    game.play(game::Turn::PlaceWall((3, 7), board::Orientation::Vertical));
    game.play(game::Turn::PlaceWall(
        (5, 4),
        board::Orientation::Horizontal,
    ));
    game.play(game::Turn::PlaceWall((0, 0), board::Orientation::Vertical));
    game.play(game::Turn::PlaceWall((1, 1), board::Orientation::Vertical));
    game.play(game::Turn::PlaceWall((2, 2), board::Orientation::Vertical));
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

        if let Some(selection) = selection {
            match selection {
                0 => move_pawn(&mut game),
                1 => place_wall(&game),
                _ => invalid_input(),
            };
        }
        println!("Turn finished");
    }
    println!("Well done some one won");
}

fn move_pawn(game: &mut game::Quoridor) {
    use board::Direction::*;
    use game::Turn::*;

    let directions: Vec<board::Direction> = vec![Up, Right, Down, Left]
        .iter()
        .filter(|x| game.can_move(**x))
        .map(|x| *x)
        .collect();

    let items: Vec<String> = directions
        .to_owned()
        .iter()
        .map(|x| format!("{:?}", (x)))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .clear(false)
        .default(0)
        .items(&items[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        if selection >= directions.len() {
            invalid_input();
        }
        println!("Moved");

        let turn = MovePawn(directions[selection]);

        game.play(turn);
    } else {
        invalid_input();
    }
}

fn place_wall(_game: &game::Quoridor) {}

fn invalid_input() {
    println!("invalid move");
}
