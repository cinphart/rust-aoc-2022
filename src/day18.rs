const INPUT: &str = include_str!("../data/Day18.txt");

fn part1(input: &str) -> usize {
    input.len()
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

    const TEST_INPUT: &str = include_str!("../data/Day18_test.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 64);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 8);
    }
}
