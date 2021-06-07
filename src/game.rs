#![allow(dead_code)]

use crate::board::Board;
use crate::board::Direction;
use crate::board::Orientation;
use crate::board::Point;

#[derive(Clone, Copy)]
pub enum Turn {
    PlaceWall(Point, Orientation),
    MovePawn(Direction),
}
pub struct Quoridor {
    board: Board,
    current_player: i32,
    walls_left: [i32; 2],
}

impl Quoridor {
    pub fn new_two_player() -> Quoridor {
        return Quoridor {
            board: Board::create_default(),
            current_player: 0,
            walls_left: [10, 10],
        };
    }

    fn number_of_players(&self) -> i32 {
        return 2;
    }

    fn current_player(&self) -> i32 {
        return self.current_player;
    }

    pub fn walls_left(&self, player: i32) -> i32 {
        if player < 0 || player > 1 {
            panic!("Invalid player index")
        }
        return self.walls_left[player as usize];
    }

    fn place_wall(&mut self, location: Point, orientation: Orientation) {
        self.board = self.board.place_wall(location, orientation);
    }

    fn pawn(&self, player: i32) -> Point {
        if player < 0 || player > 1 {
            panic!("Incorrect player index");
        }
        return self.board.get_pawn(player as i8).unwrap();
    }

    fn pawn_at(&self, location: Point) -> bool {
        return self.board.is_pawn(&location);
    }

    fn move_pawn(&mut self, direction: Direction) {
        self.board = self.board.move_pawn(self.current_player as i8, direction);
        self.current_player = (self.current_player + 1) % 2;
    }

    pub fn has_won(&self) -> bool {
        println!("0: {} ", self.board.get_pawn(0).unwrap().1);
        println!("1: {} ", self.board.get_pawn(1).unwrap().1);
        return self.board.get_pawn(0).unwrap().1 == 8 || self.board.get_pawn(1).unwrap().1 == 0;
    }

    pub fn play(&mut self, turn: Turn) -> bool {
        return match turn {
            Turn::MovePawn(direction) => {
                if !self.board.pawn_can_move(self.current_player() as i8, direction) {
                    return false;
                }
                self.board = self.board.move_pawn(self.current_player() as i8, direction);

                true
            }
            Turn::PlaceWall(location, orientation) => {
                self.board = self.board.place_wall(location, orientation);
                return true;
            }
        };
    }

    pub fn can_move(&self, direction: Direction) -> bool {
        return self.board.pawn_can_move(self.current_player() as i8, direction);
    }

    pub fn is_valid(&self, turn: Turn) -> bool {
        use Turn::*;
        return match turn {
            MovePawn(direction) => self.can_move(direction),
            PlaceWall(location, orientation) => self.board.can_place_wall(location, orientation)
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for not_y in 0..self.board.get_height() {
            let y = self.board.get_height() - not_y - 1;

            // Add wall row 
            for x in 0..self.board.get_width() {
                if self.board.has_wall((x - 1, y), Orientation::Horizontal)
                    || self.board.has_wall((x, y), Orientation::Horizontal)
                {
                    result.push_str("X")
                } else {
                    result.push_str("-")
                }
                if self.board.has_wall((x, y), Orientation::Horizontal)
                    || self.board.has_wall((x, y), Orientation::Vertical)
                {
                    result.push_str("X")
                } else {
                    result.push_str("-")
                }
            }

            result.push_str("\n");
            for x in 0..self.board.get_width() {
                // Add a squares row
                if self.pawn_at((x, y)) {
                    result.push_str("P")
                } else {
                    result.push_str("*");
                }
                if x != self.board.get_width() - 1 {
                    if self.board.has_wall((x, y-1), Orientation::Vertical)
                        || self.board.has_wall((x,y), Orientation::Vertical)
                    {
                        result.push_str("O")
                    } else {
                        result.push_str(" ");
                    }
                }
            }

            
            result.push_str("\n");
            
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_two_player_game() {
        let game = Quoridor::new_two_player();
        assert_eq!(2, game.number_of_players());
        assert_eq!(0, game.current_player());
        assert_eq!(10, game.walls_left(0));
        assert_eq!(10, game.walls_left(1));
        assert_eq!((4, 0), game.pawn(0));
        assert_eq!((4, 8), game.pawn(1));
    }

    #[test]
    fn test_can_move_pawn() {
        let mut game = Quoridor::new_two_player();
        assert_eq!(0, game.current_player());
        game.move_pawn(Direction::Up);
        assert_eq!((4, 1), game.pawn(0));
    }
    #[test]
    fn test_turn_swaps() {
        let mut game = Quoridor::new_two_player();
        assert_eq!(0, game.current_player());
        game.move_pawn(Direction::Up);
        assert_eq!(1, game.current_player());
    }

    #[test]
    fn test_wall_left() {
        let mut game = Quoridor::new_two_player();
        assert_eq!(10, game.walls_left(0));
        game.place_wall((4, 4), Orientation::Vertical);
    }

    #[test]
    fn can_win() {
        use Direction::*;
        let mut game = Quoridor::new_two_player();
        assert!(!game.has_won());
        let move_list = [
            Up, Down, Up, Right, Up, Down, Up, Down, Up, Down, Up, Down, Up, Down,
        ];
        for m in move_list.iter() {
            game.move_pawn(*m);
            assert!(!game.has_won());
        }

        game.move_pawn(Up);

        assert!(game.has_won());

        game.move_pawn(Down);
        game.move_pawn(Down);
        assert!(!game.has_won());
        game.move_pawn(Down);
        assert!(game.has_won());
    }

    #[test]
    fn test_playing() {
        let mut game = Quoridor::new_two_player();
        let invalid = Turn::MovePawn(Direction::Down);
        let valid = Turn::MovePawn(Direction::Up);
        assert!(!game.play(invalid));
        assert!(game.play(valid));
    }
}
