#![allow(dead_code)]
use crate::board::Direction;
use crate::board::Point;
use crate::board::Board;
use crate::board::Orientation;

enum Turn {
    PlaceWall(Point, Orientation),
    MovePawn(Direction)
}
struct Quoridor {
    board: Board,
    current_player: i32,
    walls_left: [i32; 2]
}

impl Quoridor {
    fn new_two_player() -> Quoridor {
        return Quoridor{
            board: Board::create_default(),
            current_player: 0,
            walls_left: [10, 10],
        }
    }

    fn number_of_players(&self) -> i32 {
        return 2;
    }

    fn current_player(&self) -> i32 {
        return self.current_player;
    }

    fn walls_left(&self, player: i32) -> i32 {
        if player < 0 || player > 1 {
            panic!("Invalid player index")
        }

        return self.walls_left[player as usize];
    }

    fn place_wall(&mut self, location:Point, orientation: Orientation) {
        self.board = self.board.place_wall(location, orientation);
    }

    fn pawn(&self, player: i32) -> Point {
        if player < 0 || player > 1 {
            panic!("Incorrect player index");
        }
        return self.board.get_pawn(player as i8).unwrap();
    }

    fn move_pawn(&mut self, direction:Direction) {
        self.board = self.board.move_pawn(self.current_player as i8, direction);
        self.current_player = (self.current_player + 1) % 2;
        return;
    }

    fn has_won(&self) -> bool {
        println!("0: {} ", self.board.get_pawn(0).unwrap().1);
        println!("1: {} ", self.board.get_pawn(1).unwrap().1);
        return self.board.get_pawn(0).unwrap().1 == 8
            || self.board.get_pawn(1).unwrap().1 == 0;
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
        assert_eq!((4,0), game.pawn(0));
        assert_eq!((4,8), game.pawn(1));
    }

    #[test]
    fn test_can_move_pawn() {
        let mut game = Quoridor::new_two_player();
        assert_eq!(0, game.current_player());
        game.move_pawn(Direction::Up);
        assert_eq!((4,1), game.pawn(0));
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
        game.place_wall((4,4), Orientation::Vertical);
    }

    #[test]
    fn can_win() {
        use Direction::*;
        let mut game = Quoridor::new_two_player();
        assert!(!game.has_won());
        let move_list = [
            Up, Down,
            Up, Right,
            Up, Down,
            Up, Down,
            Up, Down,
            Up, Down,
            Up, Down,
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

}