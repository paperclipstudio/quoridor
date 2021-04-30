pub mod point;
mod wall;

use point::Point;
use wall::Wall;
struct Pawn {
    location: point::Point,
    target: i32,
}

impl Pawn {}
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

use self::point::new_point;

const PAWN_TEXT: &str = "🕵️‍♀️";
const GAP_TEXT: &str = "⬜";
const WALL_TEXT: &str = "🟥";
const NO_WALL_TEXT: &str = "  ";
const MAX_WALLS: usize = 20;
pub struct Board {
    width: i32,
    height: i32,
    pawns: [Pawn; 2],
    walls: [wall::Wall; MAX_WALLS],
    next_wall: usize,
}

impl Board {
    pub fn place_wall(&mut self, point: Point, vertical: bool) {
        if self.next_wall < MAX_WALLS {
            self.walls[self.next_wall] = Wall {
                location: point,
                vertical,
            };
            self.next_wall += 1;
        }
    }

    fn pawn_here(&self, point: point::Point) -> bool {
        for p in self.pawns.iter() {
            if p.location == point {
                return true;
            }
        }
        return false;
    }

    fn wall_here(&self, here: point::Point) -> bool {
        for wall in self.walls.iter() {
            if wall.location == here {
                return true;
            }
        }
        return false;
    }
    fn can_move(&self, point: point::Point, direction: Direction) -> bool {
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
        let mut result = String::from("");
        for y in 0..self.width {
            let mut line: String = String::from("");
            for x in 0..self.height {
                let here = new_point( x, y);
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
            result = line + &result;

            let mut line2 = String::from("");
            for x in 0..self.height {
                let here = new_point(x ,y );
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
        return result;
    }
}

pub fn default_board() -> Board {
    return Board {
        width: 9,
        height: 9,
        pawns: [
            Pawn {
                location: new_point(4, 0),
                target: 8,
            },
            Pawn {
                location: new_point(4, 8),
                target: 0,
            },
        ],
        walls: [wall::default_wall(); 20],
        next_wall: 0,
    };
}
