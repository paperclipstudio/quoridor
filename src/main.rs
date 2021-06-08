#![crate_name = "quoridor"]
mod board;
mod game;
mod path_finder;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use game::{Quoridor, Turn, Turn::*};

use std::{io::Write, process::Command};

use crate::board::{Direction, Direction::*, Orientation};

fn main() {
    start_game();
}

fn clear_screen() {
    assert!(Command::new("clear").status().unwrap().success());
}

fn start_game() {
    let mut game = Quoridor::new_two_player();
    while !game.has_won() {
        let turn = get_turn(&mut game);
        if game.is_valid(turn) {
            game.play(turn);
        } else {
            invalid_input(&game);
        }
    }
    println!("Well done some one won");
}

fn new_from_file(file_name: String) -> game::Quoridor {
    let result = Quoridor::new_two_player();
    std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open(format!("saves/{}", file_name))
        .err();

    return result;
    
}

fn get_turn(game: &game::Quoridor) -> Turn {
    clear_screen();
    //println!("{}", game.history_to_str());
    std::fs::File::create("last_game.sav").unwrap();
    let mut save_file = std::fs::OpenOptions::new().append(false).write(true).open("last_game.sav").expect("Couldn't Save game");
    save_file.write(game.history_to_str().as_bytes()).unwrap();
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

    let turn: Turn;

    turn = match selection {
        Some(0) => move_pawn(game),
        Some(1) => place_wall(game),
        _ => invalid_input(game),
    };

    return turn;
}

fn move_pawn(game: &game::Quoridor) -> Turn {
    let directions: Vec<Direction> = vec![Up, Right, Down, Left]
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
            invalid_input(game);
        }

        return MovePawn(directions[selection]);
    } else {
        return invalid_input(game);
    }
}

fn place_wall(game: &game::Quoridor) -> Turn {
    let direction_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which direction?")
        .items(&["Vertical", "Horizontal", "back"])
        .interact_opt()
        .unwrap();

    let mut direction = Orientation::Horizontal;

    match direction_choice {
        Some(0) => direction = Orientation::Vertical,
        Some(1) => direction = Orientation::Horizontal,
        Some(2) => return get_turn(game),
        _ => (),
    }

    let selection: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Location")
        .validate_with(|in_text: &String| -> Result<(), &str> {
            let input = in_text.to_ascii_uppercase();
            if input.len() != 2 {
                return Err("invalid Length");
            }
            if !(input.chars().nth(0).unwrap() >= 'A' && input.chars().nth(0).unwrap() <= 'I') {
                return Err("invalid first char");
            }
            if !(input.chars().nth(1).unwrap() >= '1' && input.chars().nth(1).unwrap() <= '9') {
                return Err("invalid second char");
            }
            Ok(())
        })
        .interact_text()
        .unwrap()
        .to_ascii_uppercase();

    let col: i32 = selection.chars().nth(0).unwrap() as i32 - 'A' as i32;
    let row: i32 = selection.chars().nth(1).unwrap() as i32 - '1' as i32;
    return PlaceWall((col, row), direction);
}

fn invalid_input(_game: &game::Quoridor) -> Turn {
    let message: String = String::from("Invalid input: "); // + reason.as_str();
    let items = vec!["Conintue"];
    let _wait_for_input = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .items(&items)
        .interact()
        .unwrap();
    // TODO add a invalid turn type that does nothing.
    return PlaceWall((-1, -1), Orientation::Vertical);
}
