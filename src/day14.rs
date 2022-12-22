use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::{map, map_res, recognize},
    multi::{many1, separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};

const INPUT: &str = include_str!("../data/Day14.txt");

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(i32, i32);

#[derive(Clone, Copy)]
enum Line {
    HLine(i32, i32, i32),
    VLine(i32, i32, i32),
}

use Line::*;

impl Line {
    fn points(&self) -> Vec<Point> {
        match self {
            HLine(x, y1, y2) => (*y1..(*y2) + 1).map(|y| Point(*x, y)).collect(),
            VLine(x1, x2, y) => (*x1..(*x2) + 1).map(|x| Point(x, *y)).collect(),
        }
    }
}

fn integerp(input: &str) -> IResult<&str, i32> {
    map_res(recognize(many1(one_of("0123456789"))), |s: &str| {
        s.parse::<i32>()
    })(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(separated_pair(integerp, tag(","), integerp), |(a, b)| {
        Point(a, b)
    })(input)
}

fn points_to_lines(points: &Vec<Point>) -> Vec<Line> {
    points
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(Point(x1, y1), Point(x2, y2))| {
            if *x1 == *x2 {
                HLine(*x1, min(*y1, *y2), max(*y1, *y2))
            } else {
                VLine(min(*x1, *x2), max(*x1, *x2), *y1)
            }
        })
        .collect()
}

fn lines(input: &str) -> IResult<&str, Vec<Line>> {
    map(separated_list1(tag(" -> "), point), |s| points_to_lines(&s))(input)
}

fn part1(input: &str) -> usize {
    let lines = separated_list0(line_ending, lines)(input).unwrap().1;

    let mut populated_points: HashSet<Point> = lines
        .iter()
        .flat_map(|s| s.iter().flat_map(|p| p.points().into_iter()))
        .collect();

    let abyss_y = populated_points.iter().map(|Point(_, y)| y).max().unwrap() + 1;

    let start = Point(500, 0);

    let mut sand: usize = 0;

    let mut done = false;

    while !done {
        let mut sand_x = start.0;
        let mut sand_y = start.1;
        let mut stopped = false;
        while !stopped && sand_y < abyss_y {
            let state = (
                populated_points.contains(&Point(sand_x - 1, sand_y + 1)),
                populated_points.contains(&Point(sand_x, sand_y + 1)),
                populated_points.contains(&Point(sand_x + 1, sand_y + 1)),
            );
            (sand_x, sand_y, stopped) = match state {
                (_, false, _) => (sand_x, sand_y + 1, false),
                (false, true, _) => (sand_x - 1, sand_y + 1, false),
                (true, true, false) => (sand_x + 1, sand_y + 1, false),
                (true, true, true) => (sand_x, sand_y, true),
            }
        }
        done = sand_y == abyss_y;
        if !done {
            sand += 1;
            populated_points.insert(Point(sand_x, sand_y));
        }
    }
    sand
}

fn part2(input: &str) -> usize {
    let lines = separated_list0(line_ending, lines)(input).unwrap().1;

    let mut populated_points: HashSet<Point> = lines
        .iter()
        .flat_map(|s| s.iter().flat_map(|p| p.points().into_iter()))
        .collect();

    let lowest_y = populated_points.iter().map(|Point(_, y)| y).max().unwrap() + 1;

    let start = Point(500, 0);

    let mut sand: usize = 0;

    let mut done = false;

    while !done {
        let mut sand_x = start.0;
        let mut sand_y = start.1;
        let mut stopped = false;
        while !stopped && sand_y < lowest_y {
            let state = (
                populated_points.contains(&Point(sand_x - 1, sand_y + 1)),
                populated_points.contains(&Point(sand_x, sand_y + 1)),
                populated_points.contains(&Point(sand_x + 1, sand_y + 1)),
            );
            (sand_x, sand_y, stopped) = match state {
                (_, false, _) => (sand_x, sand_y + 1, false),
                (false, true, _) => (sand_x - 1, sand_y + 1, false),
                (true, true, false) => (sand_x + 1, sand_y + 1, false),
                (true, true, true) => (sand_x, sand_y, true),
            }
        }
        done = populated_points.contains(&start);
        if !done {
            sand += 1;
            populated_points.insert(Point(sand_x, sand_y));
        }
    }
    sand
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../data/Day14_test.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 24);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 93);
    }
}
