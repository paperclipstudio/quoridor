use super::point;

#[derive(Clone, Copy, PartialEq, Debug)]
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

impl Wall {
    // Check to see if this wall would clash with the given wall
    pub fn clashes(&self, other: Wall) -> bool {
        // If the same center they clash
        if self.location == other.location {
            return true;
        }
        
        // If they are not the same location and different rotations
        // then they can't clash
        if self.vertical != other.vertical {
            return false;
        }

        if self.location.y == other.location.y {
            // One wall is too the right of the other
            if self.vertical { 
                return false; 
            }
            let x_distance = self.location.x - other.location.x;
           if x_distance == 1 || x_distance == -1 {
               return true;
           } 
        }

        if self.location.x == other.location.x {
            // One wall is above other
            if !self.vertical {
                return false;
            }

            let y_distance = self.location.y - other.location.y;
            if y_distance == 1 || y_distance == -1 {
                return true; } }

        return false;
    }


}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wall_clashes() {
        let mut wall_a = default_wall();
        wall_a.location = point::create(4,4);
        wall_a.vertical = true;

        let mut wall_b = default_wall();
        wall_b.location = point::create(4,4);
        wall_b.vertical = true;

        assert!(wall_a.clashes(wall_b));
        assert!(wall_b.clashes(wall_a));

        wall_b.location = point::create(4,5);
        assert!(wall_a.clashes(wall_b));
        assert!(wall_b.clashes(wall_a));

        wall_b.location = point::create(5,4);
        assert!(!wall_a.clashes(wall_b));
        assert!(!wall_b.clashes(wall_a));

        wall_b.vertical = false;
        wall_a.vertical = false;

        assert!(wall_a.clashes(wall_b));
        assert!(wall_b.clashes(wall_a));

        wall_b.location = point::create(6, 4);

        assert!(!wall_a.clashes(wall_b));
        assert!(!wall_b.clashes(wall_a));

    }
}


