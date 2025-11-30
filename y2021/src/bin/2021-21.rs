use aoc_shared::{read_input_to_string, try_atoi};
use itertools::Itertools;
use std::{collections::HashMap, io};

type Output = (u16, u16);
type Solved = u64;

fn parse_input(input: &str) -> Output {
    let (_, p1_start, _, p2_start) = input
        .split_ascii_whitespace()
        .filter_map(|word| {
            if word.is_empty() {
                None
            } else {
                try_atoi::<_, 10>(word.as_bytes())
            }
        })
        .collect_tuple()
        .expect("Input must have 4 numbers");
    (p1_start, p2_start)
}

const DIRAC_WIN_SCORE: u64 = 1000;
const DIRAC_BOARD_SIZE: u16 = 10;

// 99 sided die that always rolls sequentially.
fn deterministic_dice_rolls() -> impl Iterator<Item = u16> {
    (1..101).cycle().tuples().map(|(r1, r2, r3)| r1 + r2 + r3)
}

fn part1_sol((p1, p2): Output) -> Solved {
    let mut pos = [p1 - 1, p2 - 1];
    let mut score = [0u64; 2];
    for (rolls, roll) in deterministic_dice_rolls().enumerate() {
        let pturn = rolls & 1;
        let pturn_next = (rolls + 1) & 1;

        pos[pturn] = (pos[pturn] + roll) % DIRAC_BOARD_SIZE;
        score[pturn] += pos[pturn] as u64 + 1;
        if score[pturn] >= DIRAC_WIN_SCORE {
            return score[pturn_next] * ((rolls as u64 + 1) * 3); // rolls is technically 3 per roll.
        }
    }
    unreachable!();
}

const DIRAC_WIN_SCORE_P2: u8 = 21;

macro_rules! permut_all_quantums {
    ($($el:ident),* $(,)?) => {
        permut_all_quantums!(@inner $($el)*;)
    };
    (@inner $head:ident $($tail:ident)+;) => {
        $head += 1;
        if $head == 4 {
            $head = 1;
            permut_all_quantums!(@inner $($tail)+;);
        }
    };
    (@inner $tail:ident;) => {
        $tail += 1;
        if $tail == 4 {
            break;
        }
    };
}

const QUANTUM_ROLLS: [u8; 3 * 3 * 3] = {
    let mut ret = [0u8; 3 * 3 * 3];
    let mut i = 0usize;
    let mut dice1 = 1;
    let mut dice2 = 1;
    let mut dice3 = 1;
    loop {
        ret[i] = dice1 + dice2 + dice3;
        permut_all_quantums!(dice1, dice2, dice3);
        i += 1;
    }
    ret
};

// I am too lazy to calculate this trivial freqeuncy.
// Might as well make the compiler do it.
const QUANTUM_HIST: [(u8, u8); 7] = {
    let mut ret = [(3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0)];
    let mut i = 0;
    while i < QUANTUM_ROLLS.len() {
        let roll = QUANTUM_ROLLS[i];
        let mut j = 0;
        'inner: while j < ret.len() {
            let hist = ret[j].0;
            if hist == roll {
                ret[j].1 += 1;
                break 'inner;
            }
            j += 1;
        }
        i += 1;
    }
    ret
};

fn part2_sol((p1, p2): Output) -> Solved {
    let mut memo: HashMap<(u8, u8, u8, u8), (u64, u64)> = HashMap::new();
    // at this point, given how massive this problem is, using the stack will prevent me from writing a bad solution.
    fn rec(
        memo: &mut HashMap<(u8, u8, u8, u8), (u64, u64)>,
        pos1: u8,
        score1: u8,
        pos2: u8,
        score2: u8,
    ) -> (u64, u64) {
        // See recursive call note.
        if score2 >= DIRAC_WIN_SCORE_P2 {
            return (0, 1);
        }

        // We've already seen this.
        if let Some(ret) = memo.get(&(pos1, score1, pos2, score2)) {
            return *ret;
        }

        let ret = QUANTUM_HIST
            .iter()
            .fold((0, 0), |(aw1, aw2), &(roll, freq)| {
                // flip arguments to simulate turns.
                let pnew = (pos1 + roll) % DIRAC_BOARD_SIZE as u8;
                let (win2, win1) = rec(memo, pos2, score2, pnew, score1 + pnew + 1);
                (aw1 + (freq as u64 * win1), aw2 + (freq as u64 * win2))
            });
        memo.insert((pos1, score1, pos2, score2), ret);
        ret
    }
    let (w1, w2) = rec(&mut memo, p1 as u8 - 1, 0, p2 as u8 - 1, 0);
    w1.max(w2)
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(parsed_input);
    let part2 = part2_sol(parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
