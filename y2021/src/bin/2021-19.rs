use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

use aoc_shared::{fold_decimal, read_input, AoCTokenizer, Token};
use itertools::Itertools;

type Scanner = (i32, i32, i32);

fn parse(input: Vec<u8>) -> Vec<Vec<Scanner>> {
    let mut ret = vec![];
    let mut curr_scanner = vec![];
    let mut neg = false;
    let mut xyz = [0i32; 3];
    let mut xyz_p = 0usize;
    let mut is_scan = false;

    for token in AoCTokenizer::new(&input) {
        match token {
            Token::Something(scan_delim) if scan_delim == b"scanner" => {
                if !curr_scanner.is_empty() {
                    ret.push(curr_scanner);
                    curr_scanner = vec![];
                }
                is_scan = true;
            }
            Token::Something(num) if !is_scan => {
                let num = num.iter().fold(0i32, fold_decimal);
                let num = if neg { -num } else { num };
                xyz[xyz_p] = num;
                xyz_p += 1;
                neg = false;
            }
            Token::Delimiter(neg_del) => neg = neg_del == b'-',
            Token::Newline if xyz_p == 3 => {
                curr_scanner.push((xyz[0], xyz[1], xyz[2]));
                neg = false;
                xyz_p = 0usize;
            }
            Token::Newline => {
                neg = false;
                xyz_p = 0usize;
                is_scan = false;
            }
            Token::End => {
                if !curr_scanner.is_empty() {
                    ret.push(curr_scanner);
                    curr_scanner = vec![];
                }
            }
            _ => (),
        }
    }
    ret
}

// There is only 24 possible. For the sake of writing simplicity
// one should iterate the whole point list, then select
// 1 through 24 possible permutations of each point.
fn gen_rot_vector((a, b, c): Scanner) -> [Scanner; 24] {
    [
        (a, b, c),
        (a, -b, -c),
        (-a, b, -c),
        (-a, -b, c),
        (b, a, -c),
        (b, -a, c),
        (-b, a, c),
        (-b, -a, -c),
        (c, b, -a),
        (c, -b, a),
        (-c, b, a),
        (-c, -b, -a),
        (b, c, a),
        (b, -c, -a),
        (-b, c, -a),
        (-b, -c, a),
        (c, a, b),
        (c, -a, -b),
        (-c, a, -b),
        (-c, -a, b),
        (a, c, -b),
        (a, -c, b),
        (-a, c, b),
        (-a, -c, -b),
    ]
}

#[derive(Eq, PartialEq, Debug)]
struct MatchKind {
    count: u32,
    rot_idx: usize,
}

impl MatchKind {
    fn incr(&mut self) {
        self.count += 1;
    }
}

impl Ord for MatchKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for MatchKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_slope(lhs: Scanner, rhs: Scanner) -> Scanner {
    (lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
}

fn manhattan_slope_off_fix(lhs: Scanner, rhs: Scanner) -> Scanner {
    (lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
}

// We assume a matching overlapping point(s) will all share the most common offsets between the map and the rotations.
fn find_match(lhs: &[Scanner], rhs: &[Scanner]) -> Option<(Scanner, Vec<Scanner>)> {
    let mut slope_maps = HashMap::<Scanner, MatchKind>::new();
    for &lbeacon in lhs {
        for &rbeacon in rhs {
            for (rot_idx, &rot) in gen_rot_vector(rbeacon).iter().enumerate() {
                let slope = manhattan_slope(lbeacon, rot);
                match slope_maps.get_mut(&slope) {
                    Some(slope) => slope.incr(),
                    None => {
                        slope_maps.insert(slope, MatchKind { count: 1, rot_idx });
                    }
                }
            }
        }
    }

    if let Some((&slope_off, max)) = slope_maps
        .iter()
        .max_by(|(_, lhs_mk), (_, rhs_mk)| lhs_mk.cmp(rhs_mk))
    {
        // The prompt says we need *at least* 12 overlapping point
        if max.count < 12 {
            None
        } else {
            let mut return_set = HashSet::<Scanner>::from_iter(lhs.iter().cloned());
            for &point in rhs {
                let sel_rot = gen_rot_vector(point)[max.rot_idx];
                return_set.insert(manhattan_slope_off_fix(sel_rot, slope_off));
            }
            Some((
                slope_off,
                return_set.iter().cloned().collect::<Vec<Scanner>>(),
            ))
        }
    } else {
        None
    }
}

fn solve(sensor_readings: Vec<Vec<Scanner>>) -> (i32, u32) {
    let mut next_round = VecDeque::from_iter(sensor_readings);
    let mut scanners = vec![(0, 0, 0)];
    let mut start = next_round.pop_front().unwrap();
    while !next_round.is_empty() {
        while let Some(cmp) = next_round.pop_front() {
            if let Some((scanner, found)) = find_match(&start, &cmp) {
                scanners.push(scanner);
                start = found;
                break;
            } else {
                next_round.push_back(cmp);
            }
        }
    }
    let p2 = scanners
        .iter()
        .permutations(2)
        .map(|a| {
            let &(la, lb, lc) = a[0];
            let &(ra, rb, rc) = a[1];
            la.abs_diff(ra) + lb.abs_diff(rb) + lc.abs_diff(rc)
        })
        .max()
        .unwrap();

    (start.len() as i32, p2)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let sensor_readings = parse(input);
    let (p1, p2) = solve(sensor_readings);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
