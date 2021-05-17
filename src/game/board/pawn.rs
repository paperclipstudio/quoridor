use super::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pawn {
    pub location: Point
}

impl Pawn {
    pub fn move_to(mut self, direction: Direction) -> Pawn {
        self.location = match direction {
            Up => self.location.shift(0, 1),
            Right => self.location.shift(1, 0),
            Down => self.location.shift(0, -1),
            Left => self.location.shift(-1, 0),
        };
        return self;
    }

    pub fn create() -> Pawn {
        return Pawn {
            location: Point::origin()
        };
    }

    pub fn set_location(mut self, x: i32, y: i32) -> Pawn{
        self.location = point::create(x, y);
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_test() {
        let pawn: Pawn = Pawn::create().set_location(5, 5);
        assert_eq!(point::create(5, 5), pawn.location);
        // Basic movement
        assert_eq!(point::create(6, 5), pawn.move_to(Right).location);
        assert_eq!(point::create(4, 5), pawn.move_to(Left).location);
        assert_eq!(point::create(5, 6), pawn.move_to(Up).location);
        assert_eq!(point::create(5, 4), pawn.move_to(Down).location);
    }
}
