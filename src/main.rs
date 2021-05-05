#![crate_name = "quoridor"]

mod board;

use std::process::Command;

use board::point;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::board::Board;
use crate::board::Direction::*;

fn clear_screen() {
    assert!(Command::new("clear").status().unwrap().success());
}

fn turn(board: Board) -> Board {
    clear_screen();
    print!("{}", board.to_string());
    let items = vec!["Move Pawn", "Place Wall"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .clear(true)
        .default(0)
        .items(&items[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        return match selection {
            0 => move_pawn(board),
            1 => place_wall(board),
            _ => invalid_input(board),
        };
    } else {
        println!("You didn't select anything!");
    }
    return board;
}

fn invalid_input(board: Board) -> Board {
    return board;
}

fn move_pawn(board: Board) -> Board {
    let items = vec!["Up", "Right", "Down", "Left"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .clear(false)
        .default(0)
        .items(&items[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        return match selection {
            0 => board.move_pawn(Up),
            1 => board.move_pawn(Right),
            2 => board.move_pawn(Down),
            3 => board.move_pawn(Left),
            _ => invalid_input(board),
        };
    } else {
        println!("You didn't select anything!");
        return invalid_input(board);
    }
}

fn place_wall(board: Board) -> Board {
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
        .unwrap().to_ascii_uppercase();
    let row:i32 = selection.chars().nth(0).unwrap() as i32 - 'A' as i32;
    let col:i32 = selection.chars().nth(1).unwrap() as i32 - '1' as i32;
    return board.place_wall(point::Point{
        x: col,
        y: row
    },
        true);
}


fn main() {
    println!("Hello, world!");
    let mut board = board::default_board();
    while !board.is_won() {
        board = turn(board);
    }

    println!("Hey look someone has won!");

    println!("{}", board.to_string());
}
