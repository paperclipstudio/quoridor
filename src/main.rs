const PAWN_TEXT: &str = "🕵️‍♀️";
const GAP_TEXT: &str = "⬜";
const WALL_TEXT: &str = "🟥";
const NO_WALL_TEXT: &str = "  ";

type Row = i32;
type Col = i32;

#[derive(PartialEq, Clone, Copy)]
struct Point {
    x: Col,
    y: Row
}

impl Point {
    fn shift(&self, change_x:i32, change_y:i32) -> Point {
        return Point{
            x: self.x + change_x,
            y: self.y + change_y
        }
    }
}

enum Direction {
    Up,
    Down, 
    Left,
    Right
}

#[derive(Clone, Copy)]
struct Wall {
    location: Point,
    vertical: bool
}


fn default_wall() -> Wall {
    return Wall {
        location: Point{ x: -1, y: -1}, 
        vertical: false
    }
}

struct Pawn {
    location: Point,
    target: Row
}

struct Board {
    width: i32,
    height: i32,
    pawns: [Pawn; 2],
    walls: [Wall; 20]
}

impl Board {
    fn pawn_here(&self, point: Point) -> bool {
        for p in self.pawns.iter() {
            if p.location == point {
                return true;
            }
        }
        return false;
    }
    fn wall_here(&self, point: Point, direction:Direction) -> bool {
        let point1: Point;
        let point2: Point;
        let vertical: bool;
        match direction {
            Direction::Up => {
                point1 = point;
                point2 = point.shift(-1, 0);
                vertical = false;
            },
            Direction::Left => {
                point1 = point.shift(-1, 0);
                point2 = point.shift(-1, -1);
                vertical = true;
            },
            Direction::Down => {
                point1 = point.shift(-1, -1);
                point2 = point.shift(0, -1);
                vertical = false;
            },
            Direction::Right => {
                point1 = point;
                point2 = point.shift(0, -1);
                vertical = true;
            }
        }
        for wall in self.walls.iter() {
            if wall.vertical != vertical {
                continue;
            }
            if wall.location == point1 ||
                wall.location == point2 {
                    return true;
                }
        }
        return false;
    }

}

fn default_board() -> Board {
    return Board{
        width: 9,
        height: 9,
        pawns: [
            Pawn {
                location: Point{x: 4, y: 0},
                target: 8
            },
            Pawn {
                location: Point{x: 4, y: 8},
                target: 0
            }
        ],
        walls: [default_wall(); 20]
    };
}

impl ToString for Board {
    fn to_string(&self)  -> String {
        let mut result=  String::from("");
        for y in 0..self.width {
            let mut line: String = String::from("");
            for x in 0..self.height {
                let here = Point{x:x, y:y};
                line += if self.pawn_here(here) {PAWN_TEXT} else {GAP_TEXT};
                line += if self.wall_here(here, Direction::Right) {WALL_TEXT} else {NO_WALL_TEXT};
            };
            line += "\n";
            result = line + &result;

            let mut line2 = String::from("");
            for x in 0..self.height {
                let here = Point{x:x, y:y};
                line2 += if self.wall_here(here, Direction::Up) {WALL_TEXT} else {NO_WALL_TEXT};
                line2 += NO_WALL_TEXT;                
            }
            line2.push('\n');
            result = line2 + &result;
        };
        return result;
    }
}

struct Player {
    name: String,
    pawn: Pawn
}

fn main() {
    println!("Hello, world!");
    let mut board = default_board();
    board.walls[0] = Wall {
        location: Point {
            x: 3,
            y: 3
        },
        vertical: true
    };

    board.walls[1] = Wall {
        location: Point {
            x: 5,
            y: 5
        },
        vertical: false
    };

    board.walls[2] = Wall {
        location: Point {
            x: 0,
            y: 5
        },
        vertical: true
    };

    board.walls[3] = Wall {
        location: Point {
            x: 0,
            y: 0
        },
        vertical: false
    };


    println!("{}", board.to_string());

    }

