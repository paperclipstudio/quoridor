//#![crate_name = "Quoridor Game"]
mod pawn;
pub mod point;
mod wall;

use pawn::Pawn;
use point::Point;
use wall::Wall;

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
const USER_STARTING_WALLS: usize = 10;
const MAX_WALLS: usize = USER_STARTING_WALLS * 2;

/// One complete game state
#[derive(Clone, Copy, Debug)]
pub struct Board {
    width: i32,
    height: i32,
    turn: usize,
    pawns: [Pawn; 2],
    walls: [wall::Wall; MAX_WALLS],
    walls_used: [usize; 2],
}

impl Board {
    /// Moves a player pawn in a direction
    /// return true if that move is valid and was applied
    pub fn move_pawn(mut self, direction: Direction) -> Board {
        if self.pawns.len() <= self.turn {
            // Invalid pawn index
            return self;
        }
        if !self.can_move(self.pawns[self.turn].location, direction) {
            return self;
        }

        // TODO add complex pawn hop for when a wall is blocking the hop

        // Check for Pawn hop
        if self.is_pawn(self.pawns[self.turn].location.shift_direction(direction)) {
            self.pawns[self.turn] = self.pawns[self.turn].move_to(direction);
        }
        self.pawns[self.turn] = self.pawns[self.turn].move_to(direction);

        self.turn = (self.turn + 1) % self.pawns.len();
        return self;
    }

    // Returns true if any Pawn in is in a winning state
    pub fn is_won(self) -> bool {
        return self.pawns[0].location.y == 8 || self.pawns[1].location.y == 0;
    }

    fn is_pawn(&self, point:Point) -> bool {
        for p in self.pawns.iter() {
            if p.location == point {
                return true;
            }
        }
        return false;
    }

    fn next_wall(&self) -> usize {
        return self.walls_used[0] + self.walls_used[1];
    }

    fn valid_wall_location(&self, wall_to_check: Wall) -> bool {
        for wall in self.walls.iter() {
            if wall.clashes(wall_to_check) {
                return false;
            }
        }
        return true;
    }

    pub fn place_wall(mut self, point: Point, vertical: bool) -> Board {
        let new_wall = Wall {
            location: point,
            vertical,
        };
        if self.next_wall() < MAX_WALLS && self.valid_wall_location(new_wall) {
            self.walls[self.next_wall()] = new_wall;
            self.walls_used[self.turn] += 1;
            return self.inc_turn();
        }
        // Wall couldn't be placed, nothing changed
        return self;
    }

    fn inc_turn(mut self) -> Board {
        self.turn = (self.turn + 1) % self.pawns.len();
        return self;
    }

    fn pawn_here(self, point: Point) -> bool {
        for p in self.pawns.iter() {
            if p.location == point {
                return true;
            }
        }
        return false;
    }

    fn wall_here(self, here: Point) -> bool {
        for wall in self.walls.iter() {
            if wall.location == here {
                return true;
            }
        }
        return false;
    }

    fn walls_left(&self, player: usize) -> i32 {
        return self.walls_used[player] as i32;
    }

    fn can_move(self, point: point::Point, direction: Direction) -> bool {
        // Find out if moves off of board
        let move_to = point.shift_direction(direction);
        if move_to.x < 0 || move_to.x >= self.width || move_to.y < 0 || move_to.y >= self.height {
            return false;
        }

        let point1: Point;
        let point2: Point;
        let vertical: bool;
        // Get points which a blocking wall could be
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
                return false;
            }
        }
        return true;
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
                if x != self.width - 1 {
                    line += if !self.can_move(here, Right) {
                        WALL_TEXT
                    } else {
                        NO_WALL_TEXT
                    };
                }
            }
            line += "\n";

            // Add Row Letters
            result = line + &result;

            // Print walls
            let mut line2 = String::from("");
            line2.push(('A' as u8 + y as u8) as char);
            line2.push(' ');
            for x in 0..self.height {
                let here = create(x, y);
                line2 += if !self.can_move(here, Up) {
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
            if y != self.height - 1 {
                result = line2 + &result;
            }
        }

        // Add Column numbers
        let mut labels: String = (1..9)
            .map(|n| n.to_string())
            .map(|n| String::from("   ") + &n)
            .collect();

        labels = String::from("  ") + &labels;

        result = result + &labels;
        result += "\n";

        // Add amount of walls remaining

        let walls_left = format!(
            "Player 1 walls: {}    Player 2 walls: {} \n\n",
            self.walls_left(0),
            self.walls_left(1) );

        result += &walls_left;

        // Add current player turn
        let player_turn = 
            "Current player's turn: ".to_owned() 
            + (self.turn + 1).to_string().as_str()
            + "\n\n";

        result = player_turn + &result;

        // Add a blank line to bottom

        result += "\n";
        
        return result;
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        if self.pawns[0] != other.pawns[0]
            || self.pawns[1] != other.pawns[1]
            || self.height != other.height
            || self.pawns != self.pawns
        {
            return false;
        }

        // Check that every wall in self is in other
        for wall in self.walls.iter() {
            if !other.walls.contains(wall) {
                return false;
            }
        }
        // Check that every wall in other is in self
        for wall in other.walls.iter() {
            if !self.walls.contains(wall) {
                return false;
            }
        }
        return true;
    }
}
pub fn default_board() -> Board {
    return Board {
        turn: 0,
        width: 9,
        height: 9,
        pawns: [
            Pawn::create().set_location(4, 0),
            Pawn::create().set_location(4, 8),
        ],
        walls: [wall::default_wall(); 20],
        walls_used: [0, 0],
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

    #[test]
    fn wall_cant_be_placed_on_walls() {
        let mut board = default_board();
        board = board.place_wall(point::create(4, 4), true);
        assert_ne!(board, default_board());
        let board2 = board.place_wall(point::create(4, 4), false);
        assert_eq!(board, board2);
    }
}
