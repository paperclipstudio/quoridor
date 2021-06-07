#![allow(dead_code)]
use crate::board::{Direction::*, *};
extern crate priority_queue;

fn shortest_distance(board: Board, from: &Point, to: &Point) -> i32 {
    return shortest_distance_reccsive(board, from, to);
}

fn distance_if_no_walls((a_x, a_y): &Point, (b_x, b_y): &Point) -> i32 {
    return (a_x - b_x).abs() + (a_y - b_y).abs();
}

fn shortest_distance_reccsive(board: Board, from: &Point, to: &Point) -> i32 {
    use priority_queue::PriorityQueue;
    let directions = [Up, Right, Left, Down];
    let mut to_search: PriorityQueue<Point, i32> = PriorityQueue::new();
    to_search.push(*from, distance_if_no_walls(from, to));
    let mut min_to_node: HashMap<Point, i32> = HashMap::new();
    min_to_node.insert(*from, 0);

    while !to_search.is_empty() {
        let temp = to_search.pop().unwrap();
        let current_point = &temp.0;
        let current_distance = temp.1;
        for direction in directions.iter() {
            if !board.can_move_from(*current_point, *direction) {
                continue;
            }

            let one_step = shift(&current_point, *direction);
            let estimate_distance = current_distance + distance_if_no_walls(&one_step, to);
            if min_to_node.get(&one_step).unwrap_or(&i32::MAX) < &estimate_distance {
                continue;
            }
            min_to_node.insert(one_step, *min_to_node.get(current_point).unwrap() + 1);
            //dbg!(estimate_distance);
            let (x, y) = one_step;
            let (x1, y1) = *to;
            if x == x1 && y == y1 {
                return *min_to_node.get(to).unwrap();
            }
            if one_step == *to {
                return estimate_distance;
            }

            to_search.push(one_step, 999 - estimate_distance);
        }
    }
    return -1;
}

pub fn is_path<F>(board: Board, from: &Point, target: F) -> bool
where
    F: Fn(Point) -> bool,
{
    let mut to_search: Vec<Point> = Vec::from([*from]);
    let mut searched: Vec<Point> = Vec::new();
    let mut result = false;
    while !to_search.is_empty() && !result {
        let current = to_search.pop().unwrap();
        
        for not_y in 0..10 {
            let y = 9 - not_y;
            for x in 0..10 {
                if (x, y) == current {
                    print!("@")
                } else if (x, y) == (0, 0) {
                    print!("O")
                } else if searched.contains(&(x, y)) {
                    print!("x")
                } else if to_search.contains(&(x,y)) {
                    print!("*")
                }else {
                    print!("-")
                }
            }
            println!("")
        }
        println!("\n\n");
        
        //println!("({}, {})", current.0, current.1);
        searched.push(current);
        for direction in [Up, Right, Left, Down].iter() {
            let one_step = shift(&current, *direction);
            if board.can_move_from(current, *direction) {
                if !searched.contains(&one_step) {
                    to_search.insert(0, one_step);
                }
                if target(one_step) {
                    result = true;
                    break;
                }
            }
        }
    }

    for not_y in 0..10 {
        let y = 9 - not_y;
        for x in 0..10 {
            let message = if searched.contains(&(x, y)) { "x" } else { "-" };
            print!("{}", message);
        }
        println!("")
    }

    return result;
}

use std::collections::HashMap;
fn print_shortest_path(hash: HashMap<Point, i32>, width: i32, height: i32) {
    for not_y in 0..height {
        for x in 0..width {
            let y = 9 - not_y;
            let value = hash.get(&(x, y)).unwrap_or(&-1);
            print!("|{:02}|", value);
        }
        println!("");
    }
}

#[cfg(test)]
mod test {
    use crate::board::Board;
    use crate::path_finder::*;
    #[test]
    fn no_walls_0() {
        let board = Board::create_default();
        assert_eq!(11, shortest_distance(board, &(4, 7), &(0, 0)));
    }

    #[test]
    fn no_walls_1() {
        let board = Board::create_default();
        assert_eq!(4, shortest_distance(board, &(3, 4), &(7, 4)));
    }

    #[test]
    fn no_walls_2() {
        let board = Board::create_default();
        assert_eq!(16, shortest_distance(board, &(0, 0), &(8, 8)));
    }

    #[test]
    fn one_wall() {
        let board = Board::create_default()
            .place_wall((4, 4), Orientation::Horizontal)
            .place_wall((3, 3), Orientation::Vertical);
        assert_eq!(10, shortest_distance(board, &(4, 0), &(4, 8)))
    }

    #[test]
    fn is_path_no_walls() {
        let board = Board::create_default();
        assert!(is_path(board, &(0, 0), |(_, y)| return y == 8));
    }

    #[test]
    fn is_path_no_walls_2() {
        let board = Board::create_default();
        assert!(is_path(board, &(7, 7), |(_, y)| y == 0));
    }

    #[test]
    fn no_path_when_blocked() {
        let board = Board::create_default()
            .place_wall((0, 2), Orientation::Horizontal)
            .place_wall((2, 2), Orientation::Horizontal)
            .place_wall((4, 2), Orientation::Horizontal)
            .place_wall((6, 2), Orientation::Horizontal)
            .place_wall((7, 2), Orientation::Vertical)
            .place_wall((7, 1), Orientation::Horizontal);
        println!("\n\n\n\n\n\n\n No really \n\n\n\n\n");
        assert!(!is_path(board, &(0, 0), |(_, y)| y == 7));
    }
}
