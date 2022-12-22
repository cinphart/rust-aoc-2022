use std::{cmp::Ordering, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::{map, map_res, opt, recognize},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, pair, terminated},
    IResult,
};

const INPUT: &str = include_str!("../data/Day13.txt");

fn integerp(input: &str) -> IResult<&str, i32> {
    map_res(recognize(many1(one_of("0123456789"))), |s: &str| {
        s.parse::<i32>()
    })(input)
}

#[derive(PartialEq, Eq, Debug)]
enum Item {
    Num(i32),
    List(Vec<Item>),
}

use Item::*;

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num(x) => write!(f, "{}", x)?,
            List(v) => {
                write!(f, "[")?;
                for i in v {
                    write!(f, "{},", i)?;
                }
                write!(f, "]")?;
            }
        };
        Ok(())
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Num(a), Num(b)) => Some(a.cmp(b)),
            (List(a), List(b)) => a.partial_cmp(b),
            (Num(a), List(_)) => List(vec![Num(*a)]).partial_cmp(other),
            (List(_), Num(b)) => self.partial_cmp(&List(vec![Num(*b)])),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Num(a), Num(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Num(a), List(_)) => List(vec![Num(*a)]).cmp(other),
            (List(_), Num(b)) => self.cmp(&List(vec![Num(*b)])),
        }
    }
}

fn item(input: &str) -> IResult<&str, Item> {
    alt((
        map(integerp, |i| Num(i)),
        map(
            delimited(tag("["), separated_list0(tag(","), item), tag("]")),
            |l| List(l),
        ),
    ))(input)
}

fn itempair(input: &str) -> IResult<&str, (Item, Item)> {
    pair(
        terminated(item, line_ending),
        terminated(item, opt(line_ending)),
    )(input)
}

fn part1(input: &str) -> usize {
    let all_items = separated_list1(line_ending, itempair)(input).unwrap().1;
    let items = all_items
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a.cmp(b) != Ordering::Greater)
        .collect::<Vec<_>>();
    items.iter().map(|(s, _)| s + 1).sum()
}

fn part2(input: &str) -> usize {
    let all_pairs = separated_list1(line_ending, itempair)(input).unwrap().1;
    let mut all_items = all_pairs
        .iter()
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<_>>();
    let div1 = List(vec![List(vec![Num(2)])]);
    let div2 = List(vec![List(vec![Num(6)])]);
    all_items.push(&div1);
    all_items.push(&div2);
    all_items.sort();

    for (i, item) in all_items.iter().enumerate() {
        println!("{}: {}", i, item);
    }

    all_items
        .into_iter()
        .enumerate()
        .filter(|(_, s)| **s == div1 || **s == div2)
        .map(|(s, _)| s + 1)
        .product()
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../data/Day13_test.txt");

    #[test]
    fn cmp_tests() {
        assert_eq!(Num(1).cmp(&Num(1)), Ordering::Equal);
        assert_eq!(List(vec![Num(1)]).cmp(&List(vec![Num(1)])), Ordering::Equal);
        assert_eq!(List(vec![List(vec![Num(1)])]).cmp(&Num(1)), Ordering::Equal);
        assert_eq!(List(vec![List(vec![])]).cmp(&Num(1)), Ordering::Less);
        assert_eq!(List(vec![List(vec![Num(0)])]).cmp(&Num(1)), Ordering::Less);
        assert_eq!(Num(0).cmp(&Num(1)), Ordering::Less);
        assert_eq!(Num(2).cmp(&Num(1)), Ordering::Greater);
        assert_eq!(Num(1).cmp(&List(vec![List(vec![])])), Ordering::Greater);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 140);
    }
}
