use aoc_shared::{fold_decimal_from, read_input_to_string};
use std::{cmp::Ordering, collections::HashSet, io};

type Int = u32;
type Rules = HashSet<(Int, Int)>;
type Job = Vec<Vec<Int>>;
type Output = (Rules, Job);
type Solved = Int;

fn parse_input(input: &str) -> Output {
    let (top, bot) = input.split_once("\n\n").expect("Data missing print jobs.");
    let rules = top
        .split_ascii_whitespace()
        .map(|priojob| {
            let (gt, lt) = priojob
                .split_once('|')
                .expect("Priority item not delimited properly");
            let before = fold_decimal_from(gt.as_bytes());
            let after = fold_decimal_from(lt.as_bytes());
            (before, after)
        })
        .collect::<HashSet<(Int, Int)>>();

    (
        rules,
        bot.split_ascii_whitespace()
            .map(|line| {
                line.split(',')
                    .map(|num| fold_decimal_from::<Int>(num.as_bytes()))
                    .collect::<Vec<Int>>()
            })
            .collect::<Job>(),
    )
}

fn comparator(orderpairs: &HashSet<(Int, Int)>, l: Int, r: Int) -> Ordering {
    if orderpairs.contains(&(l, r)) {
        Ordering::Less
    } else if orderpairs.contains(&(r, l)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn solve((ordering, jobs): Output) -> (Solved, Solved) {
    jobs.into_iter().fold((0, 0), |(p1, p2), mut jobs| {
        let midpoint = jobs.len() / 2;

        if jobs.is_sorted_by(|&l, &r| {
            matches!(
                comparator(&ordering, l, r),
                Ordering::Less | Ordering::Equal
            )
        }) {
            (p1 + jobs[midpoint], p2)
        } else {
            let (_, smid, _) =
                jobs.select_nth_unstable_by(midpoint, |&l, &r| comparator(&ordering, l, r));
            (p1, p2 + *smid)
        }
    })
}

// fn part2_sol(input: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let (part1, part2) = solve(parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
