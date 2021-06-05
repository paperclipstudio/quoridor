#![allow(dead_code)]

/**
Holds the state of one game board
 */
pub type Point = (i32, i32);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Wall {
    Wall(Point, Orientation),
    None,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Board {
    width: i32,
    height: i32,
    pawns: [Point; 4],
    walls: [Wall; 20],
}

use Direction::*;
impl Board {
    pub fn create() -> Board {
        return Board {
            width: 0,
            height: 0,
            pawns: [(-1, -1); 4],
            walls: [Wall::None; 20],
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

    pub fn can_move(self, pawn_index: i8, direction: Direction) -> bool {
        let (x, y) = self.pawns[pawn_index as usize];
        // Check to see if pawn will move off board
        let (new_x, new_y) = match direction {
            Up => (x, y+1),
            Right => (x+1, y),
            Down => (x, y-1),
            Left => (x-1, y)
        };

        if new_x < 0 || new_x >= self.width {
            return false;
        }


        if new_y < 0 || new_y >= self.height {
            return false;
        }

        // Check to see if trying to walk though walls

        let orientation = if direction == Up || direction == Down {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        };

        let wall_location1: Point = if direction == Up || direction == Right {
            (x, y)
        } else {
            (x - 1, y - 1)
        };

        let wall_location2: Point = if direction == Up || direction == Left {
            (x - 1, y)
        } else {
            (x, y - 1)
        };



        return !(self.has_wall(wall_location1, orientation)
            || self.has_wall(wall_location2, orientation));
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

    fn first_empty_wall(self) -> usize {
        let mut i: usize = 0;
        for wall in self.walls.iter() {
            if wall == &Wall::None {
                return i;
            }
            i = i + 1;
        }

        return 0;
    }

    pub fn place_wall(mut self, (x, y): Point, orientation: Orientation) -> Board {
        for wall in self.walls.iter() {
            match wall {
                Wall::Wall((w_x, w_y), w_orient) => {
                    // If they have the same center then they clash
                    if *w_x == x && *w_y == y {
                        return self;
                    }

                    // If they do not have the same center 
                    // Then having different orientations
                    // means that they can't be interfering
                    if orientation != *w_orient {
                        continue;
                    }


                    if (x - 1 == *w_x || x + 1 == *w_x) && y == *w_y && orientation == Orientation::Horizontal {
                        return self;
                    }

                    if (y - 1 == *w_y || y + 1 == *w_y) && x == *w_x && orientation == Orientation::Vertical {
                        return self;
                    }
                }
                Wall::None => continue
            }
        }
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.walls[self.first_empty_wall()] = Wall::Wall((x, y), orientation);
        }
        return self;
    }

    pub fn has_wall(self, location: Point, orientation: Orientation) -> bool {
        for wall in self.walls.iter() {
            match wall {
                Wall::None => continue,
                Wall::Wall(wall_location, wall_orientation) => {
                    if wall_location == &location && wall_orientation == &orientation {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn is_pawn(&self, location:&Point) -> bool {
        for pawn in self.pawns.iter() {
            if pawn == location {
                return true;
            }
        }

        return false;

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
        let mut starting_board = Board::create_default().set_pawn(0, (0, 0));
        let board = starting_board.move_pawn(0, Left);
        assert_eq!((0, 0), board.get_pawn(0).unwrap());
        let board = starting_board.move_pawn(0, Down);
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

    #[test]
    fn test_place_wall() {
        let board = Board::create_default().place_wall((4, 4), Orientation::Horizontal);
        assert!(board.has_wall((4, 4), Orientation::Horizontal));
        assert!(!board.has_wall((4, 5), Orientation::Horizontal));
    }
    #[test]
    fn test_place_wall_off_board() {
        let starting_board = Board::create().set_width(5).set_height(3);

        let mut board = starting_board.place_wall((-1, 0), Orientation::Vertical);
        assert_eq!(starting_board, board);
        board = board.place_wall((-1, -1), Orientation::Vertical);
        assert_eq!(starting_board, board);
        board = board.place_wall((5, 1), Orientation::Vertical);
        assert_eq!(starting_board, board);
        board = board.place_wall((1, 3), Orientation::Vertical);
        assert_eq!(starting_board, board);
        board = board.place_wall((5, 3), Orientation::Vertical);
        assert_eq!(starting_board, board);
    }

    #[test]
    fn test_wall_cant_be_placed_on_wall() {
        let start_board = Board::create()
            .set_width(10)
            .set_height(10)
            .place_wall((4, 5), Orientation::Horizontal);

        let mut board = start_board.place_wall((4, 5), Orientation::Vertical);
        assert_eq!(start_board, board);

        board = start_board.place_wall((3, 5), Orientation::Horizontal);
        assert_eq!(start_board, board);


        let start_board2 = Board::create()
            .set_width(10)
            .set_height(10)
            .place_wall((4, 5), Orientation::Vertical);

        board = start_board2.place_wall((4, 6), Orientation::Vertical);
        assert_eq!(start_board2, board);

        board = start_board2.place_wall((4, 4), Orientation::Vertical);
        assert_eq!(start_board2, board);
    }
    #[test]
    fn test_pawn_cant_pawn_move_though_wall() {
        let start_board = Board::create()
            .set_width(10)
            .set_height(10)
            .set_pawn(0, (5, 5));
        assert!(start_board.can_move(0, Up));
        assert!(start_board.can_move(0, Right));
        assert!(start_board.can_move(0, Down));
        assert!(start_board.can_move(0, Left));

        let board = start_board.place_wall((5, 5), Orientation::Horizontal);
        assert!(!board.can_move(0, Up));

        let board = start_board.place_wall((4, 5), Orientation::Horizontal);
        assert!(!board.can_move(0, Up));

        let board = start_board.place_wall((5, 5), Orientation::Vertical);
        assert!(!board.can_move(0, Right));

        let board = start_board.place_wall((5, 4), Orientation::Vertical);
        assert!(!board.can_move(0, Right));

        let board = start_board.place_wall((4, 5), Orientation::Vertical);
        assert!(!board.can_move(0, Left));

        let board = start_board.place_wall((4, 4), Orientation::Vertical);
        assert!(!board.can_move(0, Left));

        let board = start_board.place_wall((4, 4), Orientation::Horizontal);
        assert!(!board.can_move(0, Down));

        let board = start_board.place_wall((5, 4), Orientation::Horizontal);
        assert!(!board.can_move(0, Down));
    }

    #[test]
    fn test_can_move() {
        let mut board = Board::create_default();
        assert!(!board.can_move(0, Down));
        assert!(board.can_move(0, Up));
        assert!(!board.can_move(1, Up));
        assert!(board.can_move(1, Down));

        board = board.set_pawn(0, (0, 0));
        assert!(!board.can_move(0, Left));
        assert!(!board.can_move(0, Down));
        assert!(board.can_move(0, Up));
        assert!(board.can_move(0, Right));
    }

    #[test]
    fn edge_case_wall_placement() {
        let board = Board::create_default()
            .place_wall((0,0), Orientation::Vertical)
            .place_wall((1,1), Orientation::Vertical);

        assert!(board.has_wall((0,0), Orientation::Vertical));
        assert!(board.has_wall((1,1), Orientation::Vertical));


        

    
    }
    
}


