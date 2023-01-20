use std::str::FromStr;

use regex::Regex;

const INPUT: &str = include_str!("../data/Day16.txt");

#[derive(Debug)]
struct ValveInfo {
    name: String,
    flow: i32,
    tunnels_to: Vec<String>,
}

#[derive(Debug)]
struct ParseError {}

impl FromStr for ValveInfo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            "Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels lead to valves ([A-Z]{2}(?:, [A-Z]{2})*)",
        )
        .unwrap();
        let cap = re.captures(s).unwrap();
        Ok(ValveInfo {
            name: cap[1].to_string(),
            flow: cap[2].parse::<i32>().unwrap(),
            tunnels_to: cap[3].split(", ").map(|s| s.to_string()).collect(),
        })
    }
}

fn part1(input: &str) -> usize {
    let eol = Regex::new("\r\n|\r|\n").unwrap();
    eol.split(input)
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<ValveInfo>().unwrap())
        .count()
}

fn part2(input: &str) -> usize {
    input.len()
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../data/Day16_test.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 1651);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 8);
    }
}
