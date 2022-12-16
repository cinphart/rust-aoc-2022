use std::cmp::Reverse;

use priority_queue::PriorityQueue;
use regex::Regex;

const INPUT: &str = include_str!("../data/Day12.txt");

fn altitude(ch: char) -> i32 {
    match ch {
        'S' => 0,
        'E' => 26,
        x => x as i32 - 'a' as i32,
    }
}

fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn part1(input: &str) -> i32 {
    let eol = Regex::new("\r\n|\r|\n").unwrap();
    let altitudes: Vec<Vec<char>> = eol.split(input).map(|s| s.chars().collect()).collect();
    let mut steps: Vec<Vec<i32>> = altitudes
        .iter()
        .map(|s| s.iter().map(|_| i32::MAX).collect())
        .collect();

    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    for y in 0..altitudes.len() {
        let row = &altitudes[y];
        for x in 0..row.len() {
            if row[x] == 'S' {
                start = (x as i32, y as i32);
                steps[y][x] = 0;
            }
            if row[x] == 'E' {
                end = (x as i32, y as i32);
            }
        }
    }

    let mut outstanding: PriorityQueue<(i32, i32), Reverse<i32>> = PriorityQueue::new();

    // Load start position, with priority based on manhattan distance
    outstanding.push(start, Reverse(manhattan_distance(start, end)));

    while outstanding.len() > 0 {
        if let Some(((x, y), _)) = outstanding.pop() {
            let my_steps = steps[y as usize][x as usize];
            if my_steps < steps[end.1 as usize][end.0 as usize] {
                let my_altitude = altitude(altitudes[y as usize][x as usize]);
                for point in vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                    if point.1 >= 0 && point.1 < (altitudes.len() as i32) {
                        let dy: usize = point.1 as usize;
                        let row = &altitudes[dy];
                        if point.0 >= 0 && point.0 < row.len() as i32 {
                            let dx: usize = point.0 as usize;
                            // Can I move to this spot, and is it worth it?
                            if my_altitude + 1 >= altitude(row[dx]) && steps[dy][dx] > my_steps + 1
                            {
                                steps[dy][dx] = my_steps + 1;
                                let priority =
                                    Reverse(my_steps + 1 + manhattan_distance(point, end));
                                outstanding.push(point, priority);
                            }
                        }
                    }
                }
            }
        }
    }
    steps[end.1 as usize][end.0 as usize]
}

fn part2(input: &str) -> i32 {
    let eol = Regex::new("\r\n|\r|\n").unwrap();
    let altitudes: Vec<Vec<char>> = eol.split(input).map(|s| s.chars().collect()).collect();
    let mut steps: Vec<Vec<i32>> = altitudes
        .iter()
        .map(|s| s.iter().map(|_| i32::MAX).collect())
        .collect();

    let mut starts: Vec<(i32, i32)> = Vec::new();
    let mut end: (i32, i32) = (0, 0);

    for y in 0..altitudes.len() {
        let row = &altitudes[y];
        for x in 0..row.len() {
            if row[x] == 'S' || row[x] == 'a' {
                starts.push((x as i32, y as i32));
                steps[y][x] = 0;
            }
            if row[x] == 'E' {
                end = (x as i32, y as i32);
            }
        }
    }

    let mut outstanding: PriorityQueue<(i32, i32), Reverse<i32>> = PriorityQueue::new();

    for start in starts {
        // Load start position, with priority based on manhattan distance
        outstanding.push(start, Reverse(manhattan_distance(start, end)));
    }

    while outstanding.len() > 0 {
        if let Some(((x, y), _)) = outstanding.pop() {
            let my_steps = steps[y as usize][x as usize];
            if my_steps < steps[end.1 as usize][end.0 as usize] {
                let my_altitude = altitude(altitudes[y as usize][x as usize]);
                for point in vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                    if point.1 >= 0 && point.1 < (altitudes.len() as i32) {
                        let dy: usize = point.1 as usize;
                        let row = &altitudes[dy];
                        if point.0 >= 0 && point.0 < row.len() as i32 {
                            let dx: usize = point.0 as usize;
                            // Can I move to this spot, and is it worth it?
                            if my_altitude + 1 >= altitude(row[dx]) && steps[dy][dx] > my_steps + 1
                            {
                                steps[dy][dx] = my_steps + 1;
                                let priority =
                                    Reverse(my_steps + 1 + manhattan_distance(point, end));
                                outstanding.push(point, priority);
                            }
                        }
                    }
                }
            }
        }
    }
    steps[end.1 as usize][end.0 as usize]
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../data/Day12_test.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 31);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 29);
    }
}
