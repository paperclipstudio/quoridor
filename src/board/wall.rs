use super::point;

#[derive(Clone, Copy)]
pub struct Wall {
    pub location: point::Point,
    pub vertical: bool
}

pub fn default_wall() -> Wall {
    return Wall {
        location: point::create(-1 , -1),
        vertical: false,
    };
}
