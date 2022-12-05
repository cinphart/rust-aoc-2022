use itertools::Itertools;
use regex::Regex;
use std::{collections::VecDeque, fs};

fn parse_stack(s: &str, stacks: &mut Vec<VecDeque<char>>) {
    let re = Regex::new("\\[([A-Z])\\]").unwrap();
    let mut idx = 0;
    for stack_top in s.chars().into_iter().chunks(4).into_iter() {
        match re.captures(stack_top.collect::<String>().as_str()) {
            Some(s) => stacks[idx].push_back(s[1].chars().nth(0).unwrap()),
            None => (),
        }
        idx = idx + 1;
    }
}

fn parse_move(s: &str) -> Option<(usize, usize, usize)> {
    let re = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    if let Some(cap) = re.captures(s) {
        let a1 = cap[1].parse::<usize>().unwrap();
        let a2 = cap[2].parse::<usize>().unwrap() - 1;
        let a3 = cap[3].parse::<usize>().unwrap() - 1;
        Some((a1, a2, a3))
    } else {
        None
    }
}

fn part1(name: &str) -> String {
    let as_string = fs::read_to_string(name).expect("Couldn't open file");
    let contents = as_string.split("\r\n").collect::<Vec<_>>();

    let len = (contents[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _i in 0..len {
        stacks.push(VecDeque::new())
    }
    for line in contents.iter().take_while(|s| s.len() > 1) {
        println!("{}", line);
        if (&line).len() > 1 {
            parse_stack(&line, &mut stacks)
        }
    }

    for (number, from, to) in contents
        .into_iter()
        .skip_while(|s| (*s).len() > 1)
        .filter_map(parse_move)
    {
        for _i in 0..number {
            let piece = stacks[from].pop_front().unwrap();
            stacks[to].push_front(piece)
        }
    }
    stacks
        .iter_mut()
        .map(|s| s.pop_front().unwrap())
        .collect::<String>()
}

fn part2(name: &str) -> String {
    let as_string = fs::read_to_string(name).expect("Couldn't open file");
    let contents = as_string.split("\r\n").collect::<Vec<_>>();

    let len = (contents[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _i in 0..len {
        stacks.push(VecDeque::new())
    }
    for line in contents.iter().take_while(|s| s.len() > 1) {
        println!("{}", line);
        if (&line).len() > 1 {
            parse_stack(&line, &mut stacks)
        }
    }

    for (number, from, to) in contents
        .into_iter()
        .skip_while(|s| (*s).len() > 1)
        .filter_map(parse_move)
    {
        let mut tmp: VecDeque<char> = VecDeque::new();
        for _i in 0..number {
            let piece = stacks[from].pop_front().unwrap();
            tmp.push_front(piece);
        }
        while let Some(c) = tmp.pop_front() {
            stacks[to].push_front(c)
        }
    }
    stacks
        .iter_mut()
        .map(|s| s.pop_front().unwrap())
        .collect::<String>()
}

fn main() {
    println!("Part 1: {}", part1("data/Day05.txt"));
    println!("Part 2: {}", part2("data/Day05.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("data/Day05_test.txt"), "CMZ");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("data/Day05_test.txt"), "MCD");
    }
}
