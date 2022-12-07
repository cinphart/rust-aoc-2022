use std::{collections::HashSet, fs};

fn not_unique(sl: &[char]) -> bool {
    let mut c: HashSet<char> = HashSet::new();
    for &ch in sl {
        c.insert(ch);
    }
    c.len() != sl.len()
}

fn part1(name: &str) -> usize {
    let chars = fs::read_to_string(name)
        .expect("Couldn't open file")
        .chars()
        .collect::<Vec<char>>();
    chars
        .windows(4)
        .into_iter()
        .take_while(|s| not_unique(&s))
        .count()
        + 4
}

fn part2(name: &str) -> usize {
    let chars = fs::read_to_string(name)
        .expect("Couldn't open file")
        .chars()
        .collect::<Vec<char>>();
    chars
        .windows(14)
        .into_iter()
        .take_while(|s| not_unique(&s))
        .count()
        + 14
}

fn main() {
    println!("Part 1: {}", part1("data/Day06.txt"));
    println!("Part 2: {}", part2("data/Day06.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("data/Day06_test.txt"), 7);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("data/Day06_test.txt"), 19);
    }
}
