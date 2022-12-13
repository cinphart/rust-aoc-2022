use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

const INPUT: &str = include_str!("../data/Day10.txt");

#[derive(Debug)]
struct ParseError {
    _o: String,
}

enum Instruction {
    NoOp,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").take(2).collect::<Vec<_>>();
        if parts[0] == "addx" {
            let size = parts[1]
                .parse::<i32>()
                .or(Err(ParseError { _o: s.to_string() }))?;
            Ok(Instruction::AddX(size))
        } else {
            Ok(Instruction::NoOp)
        }
    }
}

impl Instruction {
    fn steps(self: &Self) -> Vec<(i32, String)> {
        match self {
            Instruction::NoOp => vec![(0, "noop".to_string())],
            Instruction::AddX(x) => vec![(0, "addx1".to_string()), (*x, "addx2".to_string())],
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut acc: Vec<(usize, i32, i32, String)> = Vec::new();
    let mut curr: i32 = 1;
    let eol = Regex::new("\r\n|\r|\n").unwrap();
    for (idx, (change, label)) in eol
        .split(input)
        .map(|s| s.parse::<Instruction>().unwrap())
        .flat_map(|s| s.steps().into_iter())
        .enumerate()
    {
        let s = idx + 1;
        acc.push((s, change, curr, label));
        curr += change;
    }
    acc.push((acc.len() + 1, 0, curr, "endstop".to_string()));

    let steps = acc
        .into_iter()
        .take(221)
        .filter(|(idx, _, _, _)| idx % 40 == 20)
        .collect::<Vec<_>>();

    steps.into_iter().map(|s| (s.0 as i32) * s.2).sum::<i32>()
}

fn part2(input: &str) -> String {
    let mut acc: Vec<(usize, i32, i32, String)> = Vec::new();
    let mut curr: i32 = 1;
    let eol = Regex::new("\r\n|\r|\n").unwrap();
    for (idx, (change, label)) in eol
        .split(input)
        .map(|s| s.parse::<Instruction>().unwrap())
        .flat_map(|s| s.steps().into_iter())
        .enumerate()
    {
        let s = idx + 1;
        acc.push((s, change, curr, label));
        curr += change;
    }
    acc.push((acc.len() + 1, 0, curr, "endstop".to_string()));

    let mut screen = [['.'; 40]; 6];

    for y in 0..6 {
        for x in 0..40 {
            let idx = y * 40 + x;
            let crsr_pos = acc[idx].2;
            if crsr_pos >= (x as i32) - 1 && crsr_pos <= (x as i32) + 1 {
                screen[y][x] = '#';
            }
        }
    }

    screen
        .iter()
        .map(|s| s.iter().collect::<String>())
        .join("\n")
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2:\r\n{}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../data/Day10_test.txt");

    const TEST_PART2_EXPECTED: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 13140);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), TEST_PART2_EXPECTED.to_string());
    }
}
