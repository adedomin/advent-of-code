use aoc_shared::{fold_decimal, read_input, RecordGrouper, Token};
use num::traits::AsPrimitive;
use std::io;

type Output = Vec<MonkeyScript>;

const WORRY_DIV: i64 = 3;
const PART1_ROUNDS: i64 = 20;
const PART2_ROUNDS: i64 = 10000;

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul(Option<i64>),
    Add(Option<i64>),
    Div(Option<i64>, i64, i64),
}

impl From<(u8, Option<i64>)> for Op {
    fn from(value: (u8, Option<i64>)) -> Self {
        let (op, num) = value;
        match op {
            b'*' => Self::Mul(num),
            b'+' => Self::Add(num),
            _ => panic!("Invalid op: {}", op),
        }
    }
}

impl Op {
    pub fn do_op(&self, lhs: i64) -> i64 {
        match self {
            Op::Mul(rhs) => rhs.map_or_else(|| lhs * lhs, |rhs| lhs * rhs),
            Op::Add(rhs) => rhs.map_or_else(|| lhs + lhs, |rhs| lhs + rhs),
            Op::Div(rhs, monkey_t, monkey_f) => {
                if lhs % rhs.unwrap() == 0 {
                    *monkey_t
                } else {
                    *monkey_f
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct MonkeyScript {
    items: Vec<i64>,
    operation: Op,
    div_and_branch: Op,
}

#[derive(PartialEq, Eq, Default)]
enum Step {
    #[default]
    Uknk,
    Item,
    Oper,
    Test,
    TestCond,
    TestCondT,
    TestCondF,
}

#[derive(Default, Debug)]
struct MonkeyPartial {
    items: Vec<i64>,
    oper: (Option<u8>, Option<i64>),
    test: (Option<i64>, Option<i64>, Option<i64>),
}

impl From<MonkeyPartial> for MonkeyScript {
    fn from(value: MonkeyPartial) -> Self {
        let items = value.items;
        let oper = value.oper;
        let divver = value.test;
        let operation: Op = if let (Some(op), val) = oper {
            (op, val)
        } else {
            panic!("Incomplete monkey! missing an operator. {oper:?}");
        }
        .into();
        let div_and_branch = if let (Some(num), Some(monkey1), Some(monkey2)) = divver {
            Op::Div(Some(num), monkey1, monkey2)
        } else {
            panic!("Incomplete monkey! Missing a divisible number, monkey_true or monkey_false. {divver:?}");
        };

        MonkeyScript {
            items,
            operation,
            div_and_branch,
        }
    }
}

fn is_num_and_parse<T>(input: &[u8]) -> Option<T>
where
    T: Copy + 'static,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<Output = T>,
    u8: num::traits::AsPrimitive<T>,
{
    match input[0] {
        b'0'..=b'9' => Some(input.iter().fold(0.as_(), fold_decimal)),
        _ => None,
    }
}

fn parse_input(input: Vec<u8>) -> Output {
    let tokenizer = RecordGrouper::new(&input);
    tokenizer
        .map(|token_grp| {
            token_grp
                .iter()
                .fold(
                    (MonkeyPartial::default(), Step::Uknk),
                    |(mut monkey, step), token| match token {
                        Token::Something(word) => match step {
                            Step::Item => {
                                let num = word.iter().fold(0i64, fold_decimal);
                                monkey.items.push(num);
                                (monkey, step)
                            }
                            Step::Oper => {
                                monkey.oper.1 = is_num_and_parse(word);
                                (monkey, step)
                            }
                            Step::Test => {
                                monkey.test.0 = is_num_and_parse(word);
                                (monkey, step)
                            }
                            Step::TestCond => match *word {
                                b"true" => (monkey, Step::TestCondT),
                                b"false" => (monkey, Step::TestCondF),
                                _ => (monkey, step),
                            },
                            Step::TestCondT => {
                                monkey.test.1 = is_num_and_parse(word);
                                (monkey, step)
                            }
                            Step::TestCondF => {
                                monkey.test.2 = is_num_and_parse(word);
                                (monkey, step)
                            }
                            Step::Uknk => match *word {
                                b"items" => (monkey, Step::Item),
                                b"Operation" => (monkey, Step::Oper),
                                b"Test" => (monkey, Step::Test),
                                b"If" => (monkey, Step::TestCond),
                                _ => (monkey, step),
                            },
                        },
                        Token::Delimiter(delim) if step == Step::Oper => match *delim {
                            b'*' | b'+' => {
                                monkey.oper.0 = Some(*delim);
                                (monkey, step)
                            }
                            _ => (monkey, step),
                        },
                        Token::Newline => (monkey, Step::Uknk),
                        _ => (monkey, step),
                    },
                )
                .0
                .into()
        })
        .collect::<Vec<MonkeyScript>>()
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn inverse_mod(a: i64, b: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, b);
    if g == 1 {
        Some((x % b + b) % b)
    } else {
        None
    }
}

fn chinese_rem(num: i64, moduli: &[i64]) -> Option<i64> {
    let prod = moduli.iter().product::<i64>();
    moduli
        .iter()
        .try_fold(0i64, |acc, &m| {
            let prod = prod / m;
            let rem = num % m;
            Some(acc + (rem * inverse_mod(prod, m)? * prod))
        })
        .map(|sum| sum % prod)
}

fn solve_p1(mut monkeys: Vec<MonkeyScript>) -> i64 {
    let monkey_cnt = monkeys.len();
    let mut monkey_insp_cnt = vec![0i64; monkey_cnt];

    for _round in 0..PART1_ROUNDS {
        for i in 0..monkey_cnt {
            monkey_insp_cnt[i] += monkeys[i].items.len() as i64;
            let operator = monkeys[i].operation;
            let test = monkeys[i].div_and_branch;
            let mut send = monkeys[i]
                .items
                .drain(..)
                .map(|item| {
                    let new_item = operator.do_op(item) / WORRY_DIV;
                    (test.do_op(new_item) as usize, new_item)
                })
                .collect::<Vec<(usize, i64)>>();

            send.drain(..).for_each(|(midx, item)| {
                monkeys[midx].items.push(item);
            });
        }
    }

    monkey_insp_cnt.sort_unstable();
    monkey_insp_cnt[monkey_cnt - 2..monkey_cnt]
        .iter()
        .product::<i64>()
}

fn solve_p2(mut monkeys: Vec<MonkeyScript>) -> i64 {
    let monkey_cnt = monkeys.len();
    let mut monkey_insp_cnt = vec![0i64; monkey_cnt];

    let mods = monkeys
        .iter()
        .map(|monkey| match monkey.div_and_branch {
            Op::Div(Some(div), _, _) => div,
            _ => 1,
        })
        .collect::<Vec<i64>>();

    for _round in 0..PART2_ROUNDS {
        for i in 0..monkey_cnt {
            monkey_insp_cnt[i] += monkeys[i].items.len() as i64;

            let operator = monkeys[i].operation;
            let test = monkeys[i].div_and_branch;
            let mut send = monkeys[i]
                .items
                .drain(..)
                .map(|item| {
                    let new_item = operator.do_op(item);
                    (
                        test.do_op(new_item) as usize,
                        chinese_rem(new_item, &mods).expect("monkey divisors are not coprime."),
                    )
                })
                .collect::<Vec<(usize, i64)>>();

            send.drain(..).for_each(|(midx, item)| {
                monkeys[midx].items.push(item);
            });
        }
    }

    monkey_insp_cnt.sort_unstable();
    monkey_insp_cnt[monkey_cnt - 2..monkey_cnt]
        .iter()
        .product::<i64>()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(input);
    let copy_of = parsed_input.clone();
    let part1 = solve_p1(parsed_input);
    let part2 = solve_p2(copy_of);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
