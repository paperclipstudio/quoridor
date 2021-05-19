#![allow(dead_code)]

/**
Holds the state of one game board
 */

type Point = (i32, i32);

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
pub struct Board {
    width: i32,
    height: i32,
    pawns: [Point; 4],
}

use Direction::*;
impl Board {
    pub fn create() -> Board {
        return Board {
            width: 0,
            height: 0,
            pawns: [(-1, -1); 4],
        };
    }

    pub fn create_default() -> Board {
        return Board::create()
            .set_height(9)
            .set_width(9)
            .set_pawn(0, (4, 0))
            .set_pawn(1, (4, 8));
    }

    pub fn set_width(mut self, width: i32) -> Board {
        self.width = width;
        return self;
    }

    pub fn get_width(&self) -> i32 {
        return self.width;
    }

    pub fn set_height(mut self, height: i32) -> Board {
        self.height = height;
        return self;
    }

    pub fn get_height(&self) -> i32 {
        return self.height;
    }

    fn set_pawn(mut self, pawn: i8, location: Point) -> Board {
        self.pawns[pawn as usize] = location;
        return self;
    }

    pub fn get_pawn(&self, pawn: i8) -> Option<Point> {
        if pawn >= 0 && pawn < 4 {
            return Some(self.pawns[pawn as usize]);
        }
        return None;
    }

    pub fn move_pawn(self, pawn: i8, direction: Direction) -> Board {
        let (mut x, mut y) = self.pawns[pawn as usize];
        match direction {
            Up => y = y + 1,
            Down => y = y - 1,
            Right => x = x + 1,
            Left => x = x - 1,
        };

        x = std::cmp::max(x, 0);
        x = std::cmp::min(x, self.width - 1);
        y = std::cmp::max(y, 0);
        y = std::cmp::min(y, self.height - 1);

        return self.set_pawn(pawn, (x, y));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_board() {
        let _: Board = Board::create();
        assert!(true);
    }

    #[test]
    fn test_get_width() {
        let mut board: Board = Board::create();
        board = board.set_width(9);
        assert_eq!(9, board.get_width());
    }

    #[test]
    fn test_get_height() {
        let mut board: Board = Board::create();
        board = board.set_height(4);
        assert_eq!(4, board.get_height());
    }

    #[test]
    fn test_place_pawn() {
        let board = Board::create().set_pawn(0, (5, 2));

        assert_eq!((5, 2), board.get_pawn(0).unwrap());

        // TODO
    }

    #[test]
    fn test_get_pawns() {
        let mut board = Board::create();
        board = board.set_pawn(0, (4, 4));
        assert_eq!((4, 4), board.get_pawn(0).unwrap());
    }

    #[test]
    fn test_create_default_board() {
        let board = Board::create_default();
        assert_eq!(9, board.get_width());
        assert_eq!(9, board.get_height());
        assert_eq!((4, 0), board.get_pawn(0).unwrap());
        assert_eq!((4, 8), board.get_pawn(1).unwrap());
    }

    #[test]
    fn test_move_pawn() {
        let starting_board = Board::create_default().set_pawn(0, (5, 4));
        let mut board = starting_board.move_pawn(0, Down);

        assert_eq!((5, 3), board.pawns[0]);

        board = starting_board.move_pawn(0, Up);
        assert_eq!((5, 5), board.pawns[0]);

        board = starting_board.move_pawn(0, Right);
        assert_eq!((6, 4), board.pawns[0]);

        board = starting_board.move_pawn(0, Left);
        assert_eq!((4, 4), board.pawns[0]);
    }

    #[test]
    fn test_move_pawn_edge() {
        let mut starting_board = Board::create_default()
            .set_pawn(0, (0, 0));
        let board = starting_board
            .move_pawn(0, Left);
        assert_eq!((0, 0), board.get_pawn(0).unwrap());
        let board = starting_board
            .move_pawn(0, Down);
        assert_eq!((0, 0), board.get_pawn(0).unwrap());
        starting_board = Board::create()
            .set_width(5)
            .set_height(8)
            .set_pawn(0, (4, 7));
        let board = starting_board.move_pawn(0, Right);
        assert_eq!((4, 7), board.get_pawn(0).unwrap());
        let board = starting_board.move_pawn(0, Up);
        assert_eq!((4, 7), board.get_pawn(0).unwrap());
    }
}
