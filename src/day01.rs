use std::fs;

fn part1(name: &str) -> usize {
    return fs::read_to_string(name)
    .expect("Couldn't open file")
    .split("\r\n")
    .count();
}

fn part2(name: &str) -> usize {
    return fs::read_to_string(name)
    .expect("Couldn't open file")
    .split("\r\n")
    .count();
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
        assert_eq!(part1("data/Day01_test.txt"), 0);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("data/Day01_test.txt"), 0);
    }
}
