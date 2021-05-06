
type Row = i32;
type Col = i32;
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: Col,
    pub y: Row,
}

pub fn create(x:i32, y:i32) -> Point{
    return Point{x:x, y:y};
}

impl Point {
    pub fn shift(&self, change_x: i32, change_y: i32) -> Point {
        return Point {
            x: self.x + change_x,
            y: self.y + change_y,
        };
    }

    pub fn origin() -> Point {
        return create(0, 0);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return 
            self.x == other.x &&
            self.y == other.y
    }
}
