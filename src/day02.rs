use std::fs;

use regex::Regex;

#[derive(Clone, Copy)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS
}

#[derive(Clone, Copy)]
enum WantResult {
    WIN,
    LOSE,
    DRAW
}

trait Score {
    fn score(self: &Self) -> i32;
    fn win_lose(self: &Self, opponent: &Move) -> i32;
}

impl Move {
    
    fn score(self: &Self) -> i32 { match self {
        Move::ROCK => 1,
        Move::PAPER => 2,
        Move::SCISSORS => 3
    } }

    fn win_lose(self: &Self,opponent: &Move) -> i32 {
        match (self, opponent) {
            (Move::ROCK, Move::ROCK) => 3,
            (Move::ROCK, Move::PAPER) => 0,
            (Move::ROCK, Move::SCISSORS) => 6,
            (Move::PAPER, Move::ROCK) => 6,
            (Move::PAPER, Move::PAPER) => 3,
            (Move::PAPER, Move::SCISSORS) => 0,
            (Move::SCISSORS, Move::ROCK) => 0,
            (Move::SCISSORS, Move::PAPER) => 6,
            (Move::SCISSORS, Move::SCISSORS) => 3
        }
    }
}

struct Play {
    opponent : Move,
    mine : Move
}

impl Play {
    fn score(self: &Self) -> i32 {
        return self.mine.score() + self.mine.win_lose(&self.opponent)
    }
}

#[derive(Debug)]
struct ParsePlayError {o: String}

impl std::str::FromStr for Play {
    type Err = ParsePlayError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("([ABC]) ([XYZ])").unwrap();
        let cap = re.captures(s).unwrap();
        let opponent = match &cap[1] {
            "A" => Some(Move::ROCK),
            "B" => Some(Move::PAPER),
            "C" => Some(Move::SCISSORS),
            _ => None,
        };
        let mine = match &cap[2] {
            "X" => Some(Move::ROCK),
            "Y" => Some(Move::PAPER),
            "Z" => Some(Move::SCISSORS),
            _ => None,
        };

        if let (Some(opponent), Some(mine)) = (opponent, mine) {
            Ok(Play{opponent: opponent, mine: mine})
        } else {
            Err(ParsePlayError {o: String::from(s)})
        }
    }
}

struct MatchStrategy {
    opponent : Move,
    want: WantResult
}

impl Move {
    fn move_for_result(self: &Self, result : WantResult) -> Move {
        match (self, result) {
            (x, WantResult::DRAW) => *x,
            (Move::ROCK, WantResult::LOSE) => Move::SCISSORS,
            (Move::ROCK, WantResult::WIN) => Move::PAPER,
            (Move::PAPER, WantResult::LOSE) => Move::ROCK,
            (Move::PAPER, WantResult::WIN) => Move::SCISSORS,
            (Move::SCISSORS, WantResult::LOSE) => Move::PAPER,
            (Move::SCISSORS, WantResult::WIN) => Move::ROCK,
        }
    }
}

impl std::str::FromStr for MatchStrategy {
    type Err = ParsePlayError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("([ABC]) ([XYZ])").unwrap();
        let cap = re.captures(s).unwrap();
        let opponent = match &cap[1] {
            "A" => Some(Move::ROCK),
            "B" => Some(Move::PAPER),
            "C" => Some(Move::SCISSORS),
            _ => None,
        };
        let want = match &cap[2] {
            "X" => Some(WantResult::LOSE),
            "Y" => Some(WantResult::DRAW),
            "Z" => Some(WantResult::WIN),
            _ => None,
        };

        if let (Some(opponent), Some(want)) = (opponent, want) {
            Ok(MatchStrategy{ opponent:opponent, want: want })
        } else {
            Err(ParsePlayError {o: String::from(s)})
        }
    }
}

impl WantResult {
    fn score(self: &Self) -> i32 {
        match self {
            WantResult::LOSE => 0,
            WantResult::DRAW => 3,
            WantResult::WIN => 6
        }
    }
}

impl MatchStrategy {
    fn score(self: &Self) -> i32 {
        self.want.score() + self.opponent.move_for_result(self.want).score()
    }
}

fn part1(name: &str) -> i32 {
    fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<Play>().unwrap().score())
        .sum()
}

fn part2(name: &str) -> i32 {
    fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<MatchStrategy>().unwrap().score())
        .sum()
}

fn main() {
    println!("Part 1: {}", part1("data/Day02.txt"));
    println!("Part 2: {}", part2("data/Day02.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("data/Day02_test.txt"), 15);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("data/Day02_test.txt"), 12);
    }
}
