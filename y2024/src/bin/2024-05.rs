use aoc_shared::{fold_decimal_from, read_input_to_string};
use std::{
    collections::{BinaryHeap, HashMap},
    io,
};

type Output = Vec<Vec<Job>>;
type Solved = usize;
type Int = u8;

#[derive(PartialEq, Eq, Debug)]
struct Job {
    value: Int,
    insert: usize,
    prio: Vec<Int>,
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let is_greater = self.prio.iter().any(|&v| other.value == v);
        let is_lesser = other.prio.iter().any(|&v| self.value == v);
        if is_greater {
            std::cmp::Ordering::Greater
        } else if is_lesser {
            std::cmp::Ordering::Less
        } else {
            other.insert.cmp(&self.insert)
        }
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Output {
    let mut prio: HashMap<Int, Vec<Int>> = HashMap::new();
    let (top, bot) = input.split_once("\n\n").expect("Data missing print jobs.");
    top.split_ascii_whitespace().for_each(|priojob| {
        let (gt, lt) = priojob
            .split_once('|')
            .expect("Priority item not delimited properly");
        let gt = fold_decimal_from(gt.as_bytes());
        let lt = fold_decimal_from(lt.as_bytes());
        prio.entry(gt)
            .and_modify(|p| p.push(lt))
            .or_insert(vec![lt]);
    });

    bot.split_ascii_whitespace()
        .map(|line| {
            line.split(',')
                .map(|num| fold_decimal_from::<Int>(num.as_bytes()))
                .enumerate()
                .map(|(insert, value)| {
                    let prio = prio.get(&value).cloned().unwrap_or_default();
                    Job {
                        value,
                        insert,
                        prio,
                    }
                })
                .collect::<Vec<Job>>()
        })
        .collect::<Output>()
}

fn solve(input: &Output) -> (Solved, Solved) {
    input.iter().fold((0, 0), |(p1, p2), jobs| {
        let midpoint = jobs.len() / 2;
        let jmid = jobs[midpoint].value as Solved;

        let mut pline = jobs.iter().map(|j| j.value);
        let mut heap = BinaryHeap::from_iter(jobs.iter());
        let mut out_of_order = false;
        let mut i = 0;
        let mut heapmid = 0;
        // BinaryHeap iterators are NOT in order, even when consumed with into_iter()
        while let Some(Job { value, .. }) = heap.pop() {
            let orderval = pline.next().expect("binheap and pline should be same size");
            if *value != orderval {
                out_of_order = true;
            }
            if i == midpoint {
                heapmid = *value as Solved;
            }
            i += 1;
        }
        if out_of_order {
            (p1, p2 + heapmid)
        } else {
            (p1 + jmid, p2)
        }
    })
}

// fn part2_sol(input: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let (part1, part2) = solve(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
