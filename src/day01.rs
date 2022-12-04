use core::str::Split;
use itertools::rev;
use itertools::sorted;
use itertools::Itertools;
use std::fs;

fn sum_group(iter: &mut Split<&str>) -> Option<i32> {
    match iter.next() {
        None => None,
        Some("") => Some(0),
        Some(s) => Some(
            s.parse::<i32>().unwrap()
                + match sum_group(iter) {
                    None => 0,
                    Some(x) => x,
                },
        ),
    }
}

fn part1(name: &str) -> i32 {
    return fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .batching(sum_group)
        .max()
        .unwrap_or(0);
}

fn part2(name: &str) -> i32 {
    return rev(sorted(
        fs::read_to_string(name)
            .expect("Couldn't open file")
            .split("\r\n")
            .batching(sum_group),
    ))
    .take(3)
    .sum();
}

fn main() {
    println!("Part 1: {}", part1("data/Day01.txt"));
    println!("Part 2: {}", part2("data/Day01.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("data/Day01_test.txt"), 24000);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("data/Day01_test.txt"), 45000);
    }
}
