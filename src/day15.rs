use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const INPUT: &str = include_str!("../data/Day15.txt");

#[derive(Debug, PartialEq, Eq, Hash,Clone,Copy)]
struct Point(i32, i32);

#[derive(Debug, Clone, Copy)]
struct SensorInfo {
    loc: Point,
    closest_beacon: Point,
}

#[derive(Debug)]
struct ParseError {}

impl FromStr for SensorInfo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            "Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
        )
        .unwrap();
        let cap = re.captures(s).unwrap();
        Ok(SensorInfo {
            loc: Point(
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            ),
            closest_beacon: Point(
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap(),
            ),
        })
    }
}

impl Point {
    fn manhattan_distance(&self, p: &Point) -> i32 {
        (self.0 - p.0).abs() + (self.1 - p.1).abs()
    }
}

impl SensorInfo {
    fn covered_at(&self, line: i32) -> Option<(i32, i32)> {
        let covered_manhattan = self.loc.manhattan_distance(&self.closest_beacon);
        let sensor_x = self.loc.0;
        let sensor_y = self.loc.1;
        let sensor_perpendicular_distance = (line - sensor_y).abs();

        if sensor_perpendicular_distance <= covered_manhattan {
            let covered_distance = covered_manhattan - sensor_perpendicular_distance;
            Some((sensor_x - covered_distance, sensor_x + covered_distance + 1))
        } else {
            None
        }
    }

    fn just_not_covered(&self) -> HashSet<Point> {
        let covered_manhattan =
            (self.loc.0 - self.closest_beacon.0).abs() + (self.loc.1 - self.closest_beacon.1).abs()+1;

        let sensor_x = self.loc.0;
        let sensor_y = self.loc.1;

        (0..covered_manhattan+1)
            .map(|d| (d, covered_manhattan - d))
            .flat_map(|(dx, dy)| {
                vec![
                    Point(sensor_x + dx, sensor_x + dy),
                    Point(sensor_x + dx, sensor_y - dy),
                    Point(sensor_x - dx, sensor_y - dy),
                    Point(sensor_x - dx, sensor_y + dy),
                ]
                .into_iter()
            })
            .collect()
    }

    fn hides(&self, p : &Point) -> bool {
        return self.loc.manhattan_distance(p) <= self.loc.manhattan_distance(&self.closest_beacon)
    }
}

fn part1(input: &str, line: i32) -> usize {
    let eol = Regex::new("\r\n|\r|\n").unwrap();
    let sensors = eol
        .split(input)
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<SensorInfo>().unwrap())
        .collect::<Vec<_>>();
    let beacons = sensors
        .iter()
        .filter(|s| s.closest_beacon.1 == line)
        .map(|s| s.closest_beacon.0)
        .collect::<HashSet<_>>();
    let covered = sensors
        .iter()
        .filter_map(|s| s.covered_at(line))
        .flat_map(|(x, y)| x..y)
        .collect::<HashSet<_>>();
    covered.len() - beacons.len()
}

fn part2(input: &str, max: i32) -> usize {
    let eol = Regex::new("\r\n|\r|\n").unwrap();
    let sensors = eol
        .split(input)
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<SensorInfo>().unwrap())
        .collect::<Vec<_>>();

    let possibly_not_covered = sensors
        .iter()
        .flat_map(|s| s.just_not_covered().into_iter())
        .filter(|p| p.0 >= 0 && p.0 <= max && p.1 >= 0 && p.1 <= max)
        .fold(HashMap::new(), |mut m, s| {
            *(m.entry(s).or_insert(0)) += 1;
            m
        });

    // If there's only one location, *at least* two areas have to border on this one - might be more,
    // but we can automatically exclude any point that is only in one boundary
    let need_checking = possibly_not_covered
        .iter()
        .filter_map(|(k, v)| match v {
            1 => None,
            _ => Some(k),
        })
        .copied()
        .collect::<Vec<_>>();

    println!("Needs Checking: {:?}", need_checking);

    // iterator through things that need checking, seeing if they are hidden by any of the sensors
    let result = need_checking.into_iter().filter(|p| sensors.iter().all(|s| !s.hides(p))).collect::<Vec<_>>();
    println!("Result: {:?}", result);
    (result[0].0 as usize) * 4_000_000 + (result[0].1 as usize)
}

fn main() {
    println!("Part 1: {}", part1(INPUT, 2_000_000));
    println!("Part 2: {}", part2(INPUT, 4_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../data/Day15_test.txt");

    #[test]
    fn covered_at_works() {
        let s = SensorInfo {
            loc: Point(8, 7),
            closest_beacon: Point(2, 10),
        };
        assert_eq!(s.covered_at(20), None);
        assert_eq!(s.covered_at(17), None);
        assert_eq!(s.covered_at(16), Some((8, 9)));
        assert_eq!(s.covered_at(15), Some((7, 10)));
    }

    #[test]
    fn just_not_covered_works() {
        let s = SensorInfo {
            loc: Point(0, 0),
            closest_beacon: Point(0, 1),
        };
        let mut expected: HashSet<Point> = HashSet::new();
        expected.insert(Point(2,0));
        expected.insert(Point(-2,0));
        expected.insert(Point(0,2));
        expected.insert(Point(0,-2));
        expected.insert(Point(1,1));
        expected.insert(Point(-1,1));
        expected.insert(Point(1,-1));
        expected.insert(Point(-1,-1));
        assert_eq!(s.just_not_covered(), expected)
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT, 10), 26);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT, 20), 56000011);
    }
}
