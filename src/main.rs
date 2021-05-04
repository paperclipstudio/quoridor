mod board;

use crate::board::point::new_point;
use crate::board::Direction::*;

fn main() {
    println!("Hello, world!");
    let mut board = board::default_board();
    board = board
        .place_wall(new_point(3, 3), true)
        .place_wall(new_point(3, 0), false)
        .place_wall(new_point(5, 5), true)
        .place_wall(new_point(0, 0), false)
        ;//.move_pawn(board::Direction::Up);

    board = board
        .move_pawn(Right)
        .move_pawn(Right)
        .move_pawn(Up)
        .move_pawn(Down)
        .move_pawn(Left);

    println!("{}", board.to_string());
}
