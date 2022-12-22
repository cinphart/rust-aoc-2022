use regex::Regex;
use std::{collections::HashSet, str::FromStr};

const INPUT: &str = include_str!("../data/Day15.txt");

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Debug)]
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

impl SensorInfo {
    fn covered_at(&self, line: i32) -> Option<(i32, i32)> {
        let covered_manhattan =
            (self.loc.0 - self.closest_beacon.0).abs() + (self.loc.1 - self.closest_beacon.1).abs();
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
            (self.loc.0 - self.closest_beacon.0).abs() + (self.loc.1 - self.closest_beacon.1).abs();

        let sensor_x = self.loc.0;
        let sensor_y = self.loc.1;

        (0..covered_manhattan)
            .map(|d| (d, covered_manhattan - d))
            .flat_map(|(dx, dy)| {
                vec![
                    Point(sensor_x + dx, sensor_x + dy),
                    Point(sensor_x + dx, sensor_y - dy),
                    Point(sensor_x - dx, sensor_y - dy),
                    Point(sensor_x - dx, sensor_y - dy),
                ]
                .into_iter()
            })
            .collect()
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

fn part2(input: &str) -> usize {
    let eol = Regex::new("\r\n|\r|\n").unwrap();
    let sensors = eol
        .split(input)
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<SensorInfo>().unwrap())
        .collect::<Vec<_>>();

    sensors.len()
}

fn main() {
    println!("Part 1: {}", part1(INPUT, 2000000));
    println!("Part 2: {}", part2(INPUT));
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
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT, 10), 26);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 56000011);
    }
}
