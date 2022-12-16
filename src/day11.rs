use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of, space0},
    combinator::{map, map_res, opt, recognize},
    multi::{many1, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

const INPUT: &str = include_str!("../data/Day11.txt");

#[derive(Debug, PartialEq, Eq)]
enum Operand {
    OLD,
    NUM(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    PLUS,
    MUL,
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Plus(Operand, Operand),
    Mul(Operand, Operand),
}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    idx: usize,
    starting_items: Vec<usize>,
    op: Operation,
    modulo: usize,
    ontrue: usize,
    onfalse: usize,
}

fn integerp(input: &str) -> IResult<&str, usize> {
    map_res(recognize(many1(one_of("0123456789"))), |s: &str| {
        s.parse::<usize>()
    })(input)
}

fn operand(input: &str) -> IResult<&str, Operand> {
    preceded(
        space0,
        alt((
            map(tag("old"), |_| Operand::OLD),
            map(integerp, |n| Operand::NUM(n)),
        )),
    )(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
    preceded(
        space0,
        alt((
            map(tag("+"), |_| Operator::PLUS),
            map(tag("*"), |_| Operator::MUL),
        )),
    )(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    map(
        tuple((operand, operator, operand)),
        |(left, op, right)| match op {
            Operator::PLUS => Operation::Plus(left, right),
            Operator::MUL => Operation::Mul(left, right),
        },
    )(input)
}

fn monkey_line(input: &str) -> IResult<&str, usize> {
    terminated(
        preceded(tag("Monkey "), integerp),
        pair(tag(":"), line_ending),
    )(input)
}

fn starting_items_line(input: &str) -> IResult<&str, Vec<usize>> {
    terminated(
        preceded(
            tag("  Starting items: "),
            separated_list1(tag(", "), integerp),
        ),
        line_ending,
    )(input)
}

fn operation_line(input: &str) -> IResult<&str, Operation> {
    terminated(preceded(tag("  Operation: new = "), operation), line_ending)(input)
}

fn test_line(input: &str) -> IResult<&str, usize> {
    terminated(
        preceded(tag("  Test: divisible by "), integerp),
        line_ending,
    )(input)
}

fn ontrue_line(input: &str) -> IResult<&str, usize> {
    terminated(
        preceded(tag("    If true: throw to monkey "), integerp),
        line_ending,
    )(input)
}

fn onfalse_line(input: &str) -> IResult<&str, usize> {
    terminated(
        preceded(tag("    If false: throw to monkey "), integerp),
        opt(line_ending),
    )(input)
}

//Monkey 0:
//  Starting items: 79, 98
//  Operation: new = old * 19
//  Test: divisible by 23
//    If true: throw to monkey 2
//    If false: throw to monkey 3
//
// Horrific assumptions about fixed structure of the text here :-(
fn monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            monkey_line,
            starting_items_line,
            operation_line,
            test_line,
            ontrue_line,
            onfalse_line,
        )),
        |(idx, starting_items, op, modulo, ontrue, onfalse)| Monkey {
            idx,
            starting_items,
            op,
            modulo,
            ontrue,
            onfalse,
        },
    )(input)
}

fn part1(input: &str) -> usize {
    let monkeys = separated_list1(line_ending, monkey)(input).unwrap().1;

    let mut monkey_items = monkeys
        .iter()
        .map(|m| m.starting_items.clone())
        .collect::<Vec<_>>();
    let mut monkey_counts: Vec<usize> = monkeys.iter().map(|_| 0).collect();

    for _round in 0..20 {
        for idx in 0..monkeys.len() {
            let items = monkey_items[idx].clone();
            // this is how many times I will inspect an item
            monkey_counts[idx] += items.len();
            monkey_items[idx].clear();
            // work out the disposition of each item
            for w in items {
                let v = match monkeys[idx].op {
                    Operation::Plus(Operand::OLD, Operand::NUM(x)) => w + x,
                    Operation::Plus(Operand::OLD, Operand::OLD) => w + w,
                    Operation::Mul(Operand::OLD, Operand::NUM(x)) => w * x,
                    Operation::Mul(Operand::OLD, Operand::OLD) => w * w,
                    _ => panic!("Should Never Happen"),
                };
                let v = v / 3;
                let pass_to = if v % monkeys[idx].modulo == 0 {
                    monkeys[idx].ontrue
                } else {
                    monkeys[idx].onfalse
                };
                monkey_items[pass_to].push(v);
            }
        }
        println!("Monkey Counts({}) = {:?}", _round, monkey_counts);
        for idx in 0..monkeys.len() {
            println!(">> Monkey {} Has: {:?}", idx, monkey_items[idx]);
        }
    }
    monkey_counts
        .into_iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

fn part2(input: &str) -> usize {
    let monkeys = separated_list1(line_ending, monkey)(input).unwrap().1;

    let mut monkey_items = monkeys
        .iter()
        .map(|m| m.starting_items.clone())
        .collect::<Vec<_>>();
    let mut monkey_counts: Vec<usize> = monkeys.iter().map(|_| 0).collect();

    let modulo = monkeys.iter().map(|s| s.modulo).product::<usize>();

    for _round in 0..10000 {
        for idx in 0..monkeys.len() {
            let items = monkey_items[idx].clone();
            // this is how many times I will inspect an item
            monkey_counts[idx] += items.len();
            monkey_items[idx].clear();
            // work out the disposition of each item
            for w in items {
                let v = match monkeys[idx].op {
                    Operation::Plus(Operand::OLD, Operand::NUM(x)) => w + x,
                    Operation::Plus(Operand::OLD, Operand::OLD) => w + w,
                    Operation::Mul(Operand::OLD, Operand::NUM(x)) => w * x,
                    Operation::Mul(Operand::OLD, Operand::OLD) => w * w,
                    _ => panic!("Should Never Happen"),
                };
                let pass_to = if v % monkeys[idx].modulo == 0 {
                    monkeys[idx].ontrue
                } else {
                    monkeys[idx].onfalse
                };
                monkey_items[pass_to].push(v % modulo);
            }
        }
    }
    monkey_counts
        .into_iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../data/Day11_test.txt");

    #[test]
    fn monkey_line_works() {
        assert_eq!(monkey_line("Monkey 0:\r\n").unwrap(), ("", 0))
    }

    #[test]
    fn starting_items_works() {
        assert_eq!(
            starting_items_line("  Starting items: 79, 98\r\n").unwrap(),
            ("", vec![79, 98])
        )
    }

    #[test]
    fn operand_works() {
        assert_eq!(operand(" old\r\n").unwrap(), ("\r\n", Operand::OLD));
        assert_eq!(operand("19\r\n").unwrap(), ("\r\n", Operand::NUM(19)));
    }

    #[test]
    fn operation_works() {
        assert_eq!(
            operation(" old + 19\r\n").unwrap(),
            ("\r\n", Operation::Plus(Operand::OLD, Operand::NUM(19)))
        );
    }

    #[test]
    fn operation_line_add_works() {
        assert_eq!(
            operation_line("  Operation: new = old + 19\r\n").unwrap(),
            ("", Operation::Plus(Operand::OLD, Operand::NUM(19)))
        );
    }

    #[test]
    fn operation_line_mul_works() {
        assert_eq!(
            operation_line("  Operation: new = 19 * old\r\n").unwrap(),
            ("", Operation::Mul(Operand::NUM(19), Operand::OLD))
        );
    }

    #[test]
    fn test_line_works() {
        assert_eq!(test_line("  Test: divisible by 23\r\n").unwrap(), ("", 23))
    }

    #[test]
    fn ontrue_line_works() {
        assert_eq!(
            ontrue_line("    If true: throw to monkey 2\r\n").unwrap(),
            ("", 2)
        )
    }

    #[test]
    fn onfalse_line_works() {
        assert_eq!(
            onfalse_line("    If false: throw to monkey 3\r\n").unwrap(),
            ("", 3)
        )
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 10605);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 2713310158);
    }
}
