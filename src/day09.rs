use std::{collections::HashSet};

use regex::Regex;

const INPUT: &str = include_str!("../data/Day09.txt");


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point (i32, i32);

#[derive(PartialEq, Eq, Hash)]
struct Move (Point, usize);

fn parse(input: &str) -> Move {
    let re = Regex::new("([RLUD]) ([0-9]+)").unwrap();
    let cap = re.captures(input).unwrap();
    let size = &cap[2].parse::<usize>().unwrap();
    let dir = match &cap[1] {
        "R" => Point(-1,0),
        "L" => Point(1,0),
        "U" => Point(0,-1),
        _ => Point(0,1)
    };
    Move(dir, *size)
}

fn new_tail_pos(head: Point, tail: Point) -> Point {
    let xdiff = tail.0 - head.0;
    let ydiff = tail.1 - head.1;
    // don't move if touching
    if xdiff.abs() <= 1 && ydiff.abs() <= 1 {
        return tail
    }
    return Point(tail.0-1*xdiff.signum(), tail.1-1*ydiff.signum())
}

fn part1(input: &str) -> usize {
    let mut head = Point(0,0);
    let mut tail = Point(0,0);
    let mut visited: HashSet<Point> = HashSet::new();
    for m in input.split("\r\n").map(parse) {
        for _ in 0..m.1 {
            let newhead = Point(head.0+m.0.0, head.1+m.0.1);
            let newtail = new_tail_pos(newhead, tail);
            visited.insert(newtail);
            head = newhead;
            tail = newtail;
        }
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    let mut rope : Vec<Point> = (0..10).map(|_| Point(0,0)).collect();
    let mut visited: HashSet<Point> = HashSet::new();
    for Move(Point(x,y),s) in input.split("\r\n").map(parse) {
        for _ in 0..s {
            rope[0] = Point(rope[0].0+x, rope[0].1+y);
            for t in 1..rope.len() {
                rope[t] = new_tail_pos(rope[t-1], rope[t]);                
            }
            visited.insert(rope[9]);
        }
    }
    visited.len()
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1_INPUT: &str = include_str!("../data/Day09_test.txt");
    const TEST2_INPUT: &str = include_str!("../data/Day09_test2.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST1_INPUT), 13);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST2_INPUT), 36);
    }
}
