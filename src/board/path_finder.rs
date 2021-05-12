//use PriorityQueue::*;

pub mod path_finder {
    use super::*;
    use crate::board::Board;
    use crate::board::Direction;
    use crate::board::*;
    use crate::point::Point;
    use priority_queue::PriorityQueue;
    pub struct PathFinder {
        board: Board,
    }

    impl PathFinder {
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
                let mut result: Vec<Point> = Vec::new();
                

                return Some(result);
            }

            return None;
        }
    }

    fn distance_between(a: Point, b: Point) -> i32 {
        let xDiff = (a.x - b.x).abs();
        let yDiff = (a.y - b.y).abs();
        return xDiff + yDiff;
    }

    struct Path {
        path: Vec<Point>,
    }

    impl Path {
        fn length(self) -> i32 {
            return self.path.len() as i32;
        }
    }
}
