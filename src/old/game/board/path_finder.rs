//use PriorityQueue::*;

pub mod path_finder {
    use crate::game::board::Board;
    use crate::game::board::Direction;
    use crate::game::board::*;
    use crate::game::point::Point;
    use priority_queue::PriorityQueue;
    pub struct PathFinder {
        pub board: Board,
    }

    impl PathFinder {
        /**
         * Returns the shortest number of steps to reach goal
         * or None if no path
         */
        pub fn distance_to_goal(&self, from: Point, goal: i32) -> Option<i32> {
            let mut to_search: PriorityQueue<Point, i32> = PriorityQueue::new();
            let mut searched: Vec<(Point, i32)> = Vec::new();
            let directions: [Direction; 4] = [Up, Right, Down, Left];

            to_search.push(from, 0);
            loop {
                // If nope left to check then no path
                if to_search.is_empty() {
                    break;
                }

                let popped = to_search.pop().unwrap();
                let current = popped.0;
                let distance = popped.1;

                if current.y == goal {
                    return Some(distance);
                }

                
                // Check if it has already been searched.
                for pair in &searched {
                    if current == pair.0 {
                        continue;
                    }
                }

                for direction in directions.iter() {
                    let next = current.shift_direction(*direction);
                    if (self).board.can_move(current, *direction) {
                        to_search.push(next, distance + 1);
                    }
                }
                searched.push((current, distance));
            }

            return None;
        }

        #[allow(dead_code)]
        fn path_between(self, from: Point, to: Point) -> Option<Vec<Point>> {
            let mut to_search: PriorityQueue<Point, i32> = PriorityQueue::new();
            to_search.push(from, 0);
            let mut searched: Vec<(Point, i32)> = Vec::new();
            let mut current;
            let mut current_distance;
            loop {
                let pop = to_search.pop().unwrap();
                current = pop.0;
                current_distance = pop.1;
                searched.push((current, current_distance));
                let directions: [Direction; 4] = [Up, Right, Down, Left];

                for direction in directions.iter() {
                    let next = current.shift_direction(*direction);
                    // Check to see if next has already been searched
                    for pair in &searched {
                        if next == pair.0 {
                            continue;
                        }
                    }

                    if self.board.can_move(current, *direction) {
                        to_search.push(current.shift_direction(*direction), current_distance + 1);
                    }
                }

                if !to_search.is_empty() && current != to {
                    break;
                }
            }

            if current == to {
                let result: Vec<Point> = Vec::new();

                return Some(result);
            }

            return None;
        }
    }
#[allow(dead_code)]
    fn distance_between(a: Point, b: Point) -> i32 {
        let x_diff = (a.x - b.x).abs();
        let y_diff = (a.y - b.y).abs();
        return x_diff + y_diff;
    }

}
