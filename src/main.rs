mod board;

use crate::board::point::new_point;

fn main() {
    println!("Hello, world!");
    let mut board = board::default_board();
    board.place_wall(new_point(3, 3), true);
    board.place_wall(new_point(3, 0), false);
    board.place_wall(new_point(5, 5), true);
    board.place_wall(new_point(0, 0), false);
    board.move_pawn(0, board::Direction::Up);
    
    board = board.move_pawn(0, board::Direction::Right);
    
    board = board.move_pawn(0, board::Direction::Right);
    
    board = board.move_pawn(0, board::Direction::Up);

    println!("{}", board.to_string());
}