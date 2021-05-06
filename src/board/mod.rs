//#![crate_name = "Quoridor Game"]
pub mod point;
mod wall;
mod pawn;

use point::Point;
use wall::Wall;
use pawn::Pawn;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

use self::point::create;

const PAWN_TEXT: &str = "😎";
const GAP_TEXT: &str = "⭕";
const WALL_TEXT: &str = "❌";
const NO_WALL_TEXT: &str = "  ";
const MAX_WALLS: usize = 20;

/// One complete game state
#[derive(Clone, Copy)]
pub struct Board {
    width: i32,
    height: i32,
    turn: usize,
    pawns: [Pawn; 2],
    walls: [wall::Wall; MAX_WALLS],
    next_wall: usize,
}

impl Board {
    /// Moves a player pawn in a direction
    /// return true if that move is valid and was applied
    pub fn move_pawn(mut self, direction: Direction) -> Board { 
        if self.pawns.len() >= self.turn {
            // Invalid pawn index
            //return self;
        }
        if !self.can_move(self.pawns[self.turn].location, direction) {
            return self;
        }
        self.pawns[self.turn] = self.pawns[self.turn].move_to(direction);
         
        self.turn = (self.turn + 1) % self.pawns.len();
        return self;
    }

    // Returns true if any Pawn in is in a winning state
    pub fn is_won(self) -> bool {
        return  self.pawns[0].location.y == 8 ||
            self.pawns[1].location.y == 0;
    }

    pub fn place_wall(mut self, point: Point, vertical: bool) -> Board {
        if self.next_wall < MAX_WALLS {
            self.walls[self.next_wall] = Wall {
                location: point,
                vertical,
            };
            self.next_wall += 1;
            return self.inc_turn();
        }
        // Wall couldn't be placed, nothing changed
        return self;
    }

    fn inc_turn(mut self) -> Board {
        self.turn = (self.turn + 1) % self.pawns.len();
        return self;
    }

    fn pawn_here(self, point: point::Point) -> bool {
        for p in self.pawns.iter() {
            if p.location == point {
                return true;
            }
        }
        return false;
    }

    fn wall_here(self, here: point::Point) -> bool {
        for wall in self.walls.iter() {
            if wall.location == here {
                return true;
            }
        }
        return false;
    }
    fn can_move(self, point: point::Point, direction: Direction) -> bool {
        let point1: Point;
        let point2: Point;
        let vertical: bool;
        match direction {
            Up => {
                point1 = point;
                point2 = point.shift(-1, 0);
                vertical = false;
            }
            Left => {
                point1 = point.shift(-1, 0);
                point2 = point.shift(-1, -1);
                vertical = true;
            }
            Down => {
                point1 = point.shift(-1, -1);
                point2 = point.shift(0, -1);
                vertical = false;
            }
            Right => {
                point1 = point;
                point2 = point.shift(0, -1);
                vertical = true;
            }
        }
        for wall in self.walls.iter() {
            if wall.vertical != vertical {
                continue;
            }
            if wall.location == point1 || wall.location == point2 {
                return true;
            }
        }
        return false;
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for y in 0..self.width {
            
            let mut line: String = String::new();
            line += "  ";
            for x in 0..self.height {
                let here = create(x, y);
                line += if self.pawn_here(here) {
                    PAWN_TEXT
                } else {
                    GAP_TEXT
                };
                line += if self.can_move(here, Right) {
                    WALL_TEXT
                } else {
                    NO_WALL_TEXT
                };
            }
            line += "\n";
            if y == 8 {
                //continue;
            }
            // Add Row Letters
            result = line + &result;


            // Print walls
            let mut line2 = String::from("");
            line2.push(('A' as u8 + y as u8) as char);
            line2.push(' ');
            for x in 0..self.height {
                let here = create(x, y);

                line2 += if self.can_move(here, Up) {
                    WALL_TEXT
                } else {
                    NO_WALL_TEXT
                };
                line2 += if self.wall_here(here) {
                    WALL_TEXT
                } else {
                    NO_WALL_TEXT
                };
            }
            line2.push('\n');
            result = line2 + &result;

            
        }
        // Add Column numbers
        let mut labels: String= 
        (1..9)
        .map(|n|n.to_string())
        .map(|n| String::from("   ") + &n)
        .collect();

        labels = String::from("  ") + &labels;

        result = result + &labels;
        result += "\n";
        

        return result;
    }
}

pub fn default_board() -> Board {
    return Board {
        turn: 0,
        width: 9,
        height: 9,
        pawns: [
            Pawn::create().set_location(4, 0),
            Pawn::create().set_location(4, 8)
        ],
        walls: [wall::default_wall(); 20],
        next_wall: 0,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn winning_state() {
        let mut board = default_board();
        assert!(!board.is_won());
        board.pawns[0] = Pawn::create().set_location(3, 8);
        println!("{}", board.to_string());
        assert!(board.is_won());
        board.move_pawn(Left);
        assert!(board.is_won());
        board.pawns[0] = Pawn::create().set_location(0, 0);
        assert!(!board.is_won());
        board.pawns[1] = Pawn::create().set_location(0, 0);
        assert!(board.is_won());




    }
}