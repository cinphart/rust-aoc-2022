use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::{opt, recognize},
    multi::{many1, separated_list0, separated_list1},
    sequence::{preceded, terminated, separated_pair},
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum DirEntry {
    Dir { name: String },
    File { name: String, size: usize },
}

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Cd { name: String },
    Ls { entries: Vec<DirEntry> },
}

fn name(input: &str) -> IResult<&str, &str> {
    let (rest, m) = recognize(many1(one_of(
        "/abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.-",
    )))(input)?;
    Ok((rest, m))
}

fn integerp(input: &str) -> IResult<&str, usize> {
    let (rest, i) = recognize(many1(one_of("0123456789")))(input)?;
    Ok((rest, i.parse::<usize>().unwrap()))
}

fn cd(input: &str) -> IResult<&str, Command> {
    let (rest, _) = recognize(tag("$ cd"))(input)?;
    let (rest, _) = recognize(many1(tag(" ")))(rest)?;
    let (rest, name) = name(rest)?;
    Ok((
        rest,
        Command::Cd {
            name: name.to_string(),
        },
    ))
}

fn dirls(input: &str) -> IResult<&str, DirEntry> {
    let (rest, _) = recognize(tag("dir "))(input)?;
    let (rest, name) = name(rest)?;
    Ok((
        rest,
        DirEntry::Dir {
            name: name.to_string(),
        },
    ))
}

fn filels(input: &str) -> IResult<&str, DirEntry> {
    let (rest, (size, filename)) = separated_pair(
        integerp, tag(" "), name)(input)?;
    Ok((
        rest,
        DirEntry::File {
            name: filename.to_string(),
            size: size,
        },
    ))
}

fn dir_entry(input: &str) -> IResult<&str, DirEntry> {
    alt((dirls, filels))(input)
}

fn ls(input: &str) -> IResult<&str, Command> {
    let (rest, entries) = preceded(
        tag("$ ls"),
        opt(preceded(
            line_ending,
            separated_list1(line_ending, dir_entry),
        )),
    )(input)?;
    Ok((
        rest,
        Command::Ls {
            entries: entries.unwrap_or(Vec::new()),
        },
    ))
}

fn command(input: &str) -> IResult<&str, Command> {
    alt((cd, ls))(input)
}

fn parse_commands(input: &String) -> IResult<&str, Vec<Command>> {
    terminated(separated_list0(line_ending, command), opt(line_ending))(input.as_str())
}

fn part1(input: &String) -> usize {
    let (rest, result) = parse_commands(input).unwrap();
    println!("rest: {:?}", rest);
    println!("result: {:?}", result);
    result.len()
}

fn part2(input: &String) -> usize {
    input.len()
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
    fn cd_parse_works() {
        let (rest, result) = cd("$ cd wibble.q").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            result,
            Command::Cd {
                name: "wibble.q".to_string()
            }
        )
    }

    #[test]
    fn empty_ls_works() {
        let (rest, result) = ls("$ ls").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            result,
            Command::Ls {
                entries: Vec::new()
            }
        )
    }

    #[test]
    fn ls_with_dir_works() {
        let (rest, result) = ls("$ ls\r\ndir a.txt").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            result,
            Command::Ls {
                entries: vec![DirEntry::Dir{name: "a.txt".to_string()}]
            }
        )
    }

    #[test]
    fn ls_with_file_works() {
        let (rest, result) = ls("$ ls\r\n1234 file.w").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            result,
            Command::Ls {
                entries: vec![DirEntry::File{name: "file.w".to_string(), size: 1234}]
            }
        )
    }

    #[test]
    fn ls_with_both_works() {
        let (rest, result) = ls("$ ls\r\ndir a.txt\n1234 file.w").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            result,
            Command::Ls {
                entries: vec![DirEntry::Dir{name: "a.txt".to_string()}, DirEntry::File{name: "file.w".to_string(), size: 1234}]
            }
        )
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(&input()), 95437);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&input()), 19);
    }
}
