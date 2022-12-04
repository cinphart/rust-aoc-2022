use regex::Regex;
use std::fs;

fn parse_ranges(s: &str) -> ((u32, u32), (u32, u32)) {
    let re = Regex::new("([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
    let cap = re.captures(s).unwrap();
    let a1 = cap[1].parse::<u32>().unwrap();
    let a2 = cap[2].parse::<u32>().unwrap();
    let a3 = cap[3].parse::<u32>().unwrap();
    let a4 = cap[4].parse::<u32>().unwrap();
    ((a1, a2), (a3, a4))
}

fn contained(((a1, a2), (b1, b2)): &((u32, u32), (u32, u32))) -> bool {
    (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2)
}

fn overlaps(((a1, a2), (b1, b2)): &((u32, u32), (u32, u32))) -> bool {
    (a1 <= b1 && b1 <= a2) || (b1 <= a1 && a1 <= b2)
}

fn part1(name: &str) -> u32 {
    fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .map(parse_ranges)
        .filter(contained)
        .count() as u32
}

fn part2(name: &str) -> u32 {
    fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .map(parse_ranges)
        .filter(overlaps)
        .count() as u32
}

fn main() {
    println!("Part 1: {}", part1("data/Day04.txt"));
    println!("Part 2: {}", part2("data/Day04.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("data/Day04_test.txt"), 2);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("data/Day04_test.txt"), 4);
    }
}
