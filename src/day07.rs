use std::{cmp::min, collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of, space1},
    combinator::{map, map_res, opt, recognize},
    multi::{many0, many1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
struct Dir {
    name: String,
    file_sizes: Vec<usize>,
    sub_dirs: Vec<Dir>,
}

fn name(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of(
        "/abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.",
    )))(input)
}

fn integerp(input: &str) -> IResult<&str, usize> {
    map_res(recognize(many1(one_of("0123456789"))), |s: &str| {
        s.parse::<usize>()
    })(input)
}

fn cd(input: &str) -> IResult<&str, &str> {
    terminated(preceded(pair(tag("$ cd"), space1), name), opt(line_ending))(input)
}

fn cdup(input: &str) -> IResult<&str, ()> {
    map(terminated(tag("$ cd .."), opt(line_ending)), |_| ())(input)
}

fn dirls(input: &str) -> IResult<&str, (&str, Option<usize>)> {
    map(
        terminated(preceded(tag("dir "), name), opt(line_ending)),
        |name| (name, None),
    )(input)
}

fn filels(input: &str) -> IResult<&str, (&str, Option<usize>)> {
    map(
        terminated(separated_pair(integerp, space1, name), opt(line_ending)),
        |(size, name)| (name, Some(size)),
    )(input)
}

fn dir_entry(input: &str) -> IResult<&str, (&str, Option<usize>)> {
    alt((dirls, filels))(input)
}

fn ls(input: &str) -> IResult<&str, HashMap<&str, usize>> {
    let (rest, entries) = preceded(
        terminated(tag("$ ls"), opt(line_ending)),
        opt(many0(dir_entry)),
    )(input)?;
    Ok((
        rest,
        match entries {
            None => HashMap::new(),
            Some(x) => x
                .into_iter()
                .filter_map(|(r, s)| match s {
                    None => None,
                    Some(x) => Some((r, x)),
                })
                .collect(),
        },
    ))
}

fn endit(input: &str) -> IResult<&str, ()> {
    if input.len() == 0 {
        Ok((input, ()))
    } else {
        map(cdup, |_| ())(input)
    }
}

fn filesystem(input: &str) -> IResult<&str, Dir> {
    map(
        tuple((cd, ls, terminated(many0(filesystem), endit))),
        |(name, entries, dirs)| Dir {
            name: name.to_string(),
            file_sizes: entries.values().map(|&s| s).collect(),
            sub_dirs: dirs,
        },
    )(input)
}

fn sizeof(prefix: &String, dir: &Dir) -> (usize, usize) {
    let ftotal = dir.file_sizes.iter().map(|&s| s).sum::<usize>();
    let name = prefix.to_owned() + "/" + dir.name.as_str();
    let (dirtotal, diracc) = dir
        .sub_dirs
        .iter()
        .map(|s| sizeof(&name, s))
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    let total = ftotal + dirtotal;
    let acc = diracc + if total < 100000 { total } else { 0 };
    (total, acc)
}

fn smallest(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    match (a, b) {
        (None, x) => x,
        (x, None) => x,
        (Some(x), Some(y)) => Some(min(x, y)),
    }
}

fn smallest_deletable(prefix: &String, needed: usize, dir: &Dir) -> (usize, Option<usize>) {
    let ftotal = dir.file_sizes.iter().map(|&s| s).sum::<usize>();
    let name = prefix.to_owned() + "/" + dir.name.as_str();
    let (dirtotal, diracc) = dir
        .sub_dirs
        .iter()
        .map(|s| smallest_deletable(&name, needed, s))
        .fold((0, None), |(a, b), (c, d)| (a + c, smallest(b, d)));
    let total = ftotal + dirtotal;
    let myacc = if total > needed { Some(total) } else { None };
    let acc = smallest(myacc, diracc);
    (total, acc)
}

fn part1(input: &String) -> usize {
    let (rest, result) = filesystem(input).unwrap();
    println!("rest: {:?}", rest);
    println!("result: {:?}", result);
    sizeof(&"".to_string(), &result).1
}

fn part2(input: &String) -> usize {
    let (rest, result) = filesystem(input).unwrap();
    println!("rest: {:?}", rest);
    println!("result: {:?}", result);
    let prefix = "".to_string();
    let needed = 30000000 - (70000000 - sizeof(&prefix, &result).0);
    smallest_deletable(&prefix, needed, &result).1.unwrap()
}

fn main() {
    let input = fs::read_to_string("data/Day07.txt").expect("Couldn't open file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        fs::read_to_string("data/Day07_test.txt").expect("Couldn't open file")
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(&input()), 95437);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&input()), 24933642);
    }
}
