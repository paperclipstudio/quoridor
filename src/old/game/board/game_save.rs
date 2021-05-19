
/*

use crate::game::board::Direction;
use crate::game::point::Point;
#[derive(Clone, Debug)]
pub struct GameSave {
    moves: Vec<Turn>
}

#[derive(Clone, Copy, Debug)]
enum Turn {
    Wall(Point, bool),
    Move(Direction)
}

pub fn new() -> GameSave {
    return GameSave {
        moves: Vec::new()
    }
}

impl GameSave {
    fn _new() -> GameSave {
        return GameSave {
            moves: Vec::new()
        };
    }

    pub fn _move_pawn(&mut self, direction: Direction) {
        self.moves.push(Turn::Move(direction));
    }

    pub fn _place_wall(&mut self, location: Point, is_vertical: bool) {
        self.moves.push(Turn::Wall(location, is_vertical));
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for turn in &self.moves {
            result += (match turn {
                Turn::Move(direction) => format!("{:?}\n", direction),
                Turn::Wall(location, is_vertical) => format!("{},{}#{}\n", location.x, location.y, is_vertical)
            }).as_str();
        } 
        return result;
    }

}
*/