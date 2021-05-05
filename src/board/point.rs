
type Row = i32;
type Col = i32;
#[derive(PartialEq, Clone, Copy)]
pub struct Point {
    pub x: Col,
    pub y: Row,
}

pub fn new_point(x:i32, y:i32) -> Point{
    return Point{x:x, y:y};
}

impl Point {
    pub fn shift(&self, change_x: i32, change_y: i32) -> Point {
        return Point {
            x: self.x + change_x,
            y: self.y + change_y,
        };
    }
}
