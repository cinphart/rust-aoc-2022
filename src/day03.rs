use std::{fs, str::Split};

use itertools::{Chunk, Itertools};

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        u32::from(c) - u32::from('a') + 1
    } else {
        u32::from(c) - u32::from('A') + 27
    }
}

fn item_priorities(line: &str) -> Vec<u32> {
    line.chars().map(priority).collect()
}

fn misplaced_item_priority(line: &str) -> u32 {
    let priorities = item_priorities(line);
    let l = line.len() / 2;
    let front: Vec<u32> = priorities.iter().take(l).map(|s| *s).collect();
    let back: Vec<u32> = priorities.into_iter().skip(l).collect();
    front
        .into_iter()
        .find(|s| back.iter().any(|f| *s == *f))
        .unwrap()
}

fn badge_for_group(g: Chunk<Split<&str>>) -> u32 {
    let rows: Vec<Vec<u32>> = g.into_iter().map(item_priorities).collect();
    let first: Vec<u32> = rows[0]
        .iter()
        .filter(|s| rows[1].iter().any(|f| **s == *f))
        .map(|s| *s)
        .collect();
    first
        .into_iter()
        .find(|s| rows[2].iter().any(|f| *s == *f))
        .unwrap()
}

fn part1(name: &str) -> u32 {
    fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .map(misplaced_item_priority)
        .sum()
}

fn part2(name: &str) -> u32 {
    fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .chunks(3)
        .into_iter()
        .map(badge_for_group)
        .sum()
}

fn main() {
    println!("Part 1: {}", part1("data/Day03.txt"));
    println!("Part 2: {}", part2("data/Day03.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("data/Day03_test.txt"), 157);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("data/Day03_test.txt"), 70);
    }
}
