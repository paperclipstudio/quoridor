mod board;
mod game;
use dialoguer::{theme::ColorfulTheme, Select, Input};

use std::process::Command;

use crate::board::Orientation;

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
                1 => place_wall(&mut game),
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

fn place_wall(game: &mut game::Quoridor) {
    let direction_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which direction?")
            .items(&["Vertical", "Horizontal"])
            .interact_opt()
            .unwrap();
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
        use game::Turn::*;
        let mut direction = Orientation::Horizontal;
        match direction_choice {
            None => return,
            Some(x) => if x == 0 {direction = Orientation::Vertical}
        }
        let turn = PlaceWall((col, row), direction);
        game.play(turn);
        return;
}

fn invalid_input() {
    println!("invalid move");
}
