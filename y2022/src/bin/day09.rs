use std::io;
use y2022::{fold_decimal, read_input, AoCTokenizer, Token};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    dir: Dir,
    count: usize,
}

const GRID_DIM: usize = 2048usize;
const MID: usize = GRID_DIM / 2;

fn parse_input(input: Vec<u8>) -> Vec<Instruction> {
    let tokenizer = AoCTokenizer::new(&input);
    tokenizer
        .fold((Vec::new(), None), |(mut acc, dir), token| match token {
            Token::Something(x) if dir.is_none() => match x[0] {
                b'U' => (acc, Some(Dir::Up)),
                b'D' => (acc, Some(Dir::Down)),
                b'L' => (acc, Some(Dir::Left)),
                b'R' => (acc, Some(Dir::Right)),
                _ => {
                    let x = x.escape_ascii();
                    panic!("Invalid token: {x}")
                }
            },
            Token::Something(x) => {
                let count = x.iter().fold(0usize, fold_decimal);
                let dir = dir.unwrap();
                let res = Instruction { dir, count };
                acc.push(res);
                (acc, None)
            }
            _ => (acc, dir),
        })
        .0
}

fn manhattan_slope(lhs: (usize, usize), rhs: (usize, usize)) -> (isize, isize) {
    (
        lhs.0 as isize - rhs.0 as isize,
        lhs.1 as isize - rhs.1 as isize,
    )
}

// fn debug_print(board: &[Vec<bool>]) {
//     board.iter().for_each(|row| {
//         row.iter().for_each(|col| {
//             if *col {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         });
//         println!();
//     })
// }

fn array_windows_mut_each<T, const N: usize>(arr: &mut [T], mut f: impl FnMut(&mut [T; N])) {
    let mut start = 0;
    let mut end = N;
    while end <= arr.len() {
        f((&mut arr[start..end]).try_into().unwrap());
        start += 1;
        end += 1;
    }
}

fn solution(input: &[Instruction], rope: &mut [(usize, usize)]) -> usize {
    let mut board = vec![vec![false; GRID_DIM]; GRID_DIM];
    board[MID][MID] = true;
    let rope_pairs = rope.len() - 1;

    input.iter().for_each(|Instruction { dir, count }| {
        for _i in 0..*count {
            match dir {
                Dir::Up => rope[0].0 += 1,
                Dir::Down => rope[0].0 -= 1,
                Dir::Left => rope[0].1 -= 1,
                Dir::Right => rope[0].1 += 1,
            }
            let mut iter = 1;
            array_windows_mut_each(rope, |[(heady, headx), (taily, tailx)]| {
                match manhattan_slope((*heady, *headx), (*taily, *tailx)) {
                    // need to move down
                    (-2, 0) => *taily -= 1,
                    // need to move up
                    (2, 0) => *taily += 1,
                    // need to move left
                    (0, -2) => *tailx -= 1,
                    // need to move right
                    (0, 2) => *tailx += 1,

                    // need to move down+left
                    (-2, -1) => {
                        *taily -= 1;
                        *tailx -= 1;
                    }
                    // need to move up+right
                    (2, 1) => {
                        *taily += 1;
                        *tailx += 1;
                    }
                    // need to move down+right
                    (-2, 1) => {
                        *taily -= 1;
                        *tailx += 1;
                    }
                    // need to move up+left
                    (2, -1) => {
                        *taily += 1;
                        *tailx -= 1;
                    }

                    // need to move down+left
                    (-1, -2) => {
                        *taily -= 1;
                        *tailx -= 1;
                    }
                    // need to move up+right
                    (1, 2) => {
                        *taily += 1;
                        *tailx += 1;
                    }
                    // need to move down+right
                    (-1, 2) => {
                        *taily -= 1;
                        *tailx += 1;
                    }
                    // need to move up+left
                    (1, -2) => {
                        *taily += 1;
                        *tailx -= 1;
                    }

                    // longer rope situations only
                    // up+right
                    (2, 2) if rope_pairs > 1 => {
                        *taily += 1;
                        *tailx += 1;
                    }
                    // down+right
                    (-2, 2) if rope_pairs > 1 => {
                        *taily -= 1;
                        *tailx += 1;
                    }
                    // up+left
                    (2, -2) if rope_pairs > 1 => {
                        *taily += 1;
                        *tailx -= 1;
                    }
                    // down+left
                    (-2, -2) if rope_pairs > 1 => {
                        *taily -= 1;
                        *tailx -= 1;
                    }

                    (0, 0)
                    | (1, 0)
                    | (0, 1)
                    | (1, 1)
                    | (-1, 0)
                    | (0, -1)
                    | (-1, 1)
                    | (1, -1)
                    | (-1, -1) => (),
                    _ => panic!("We are not in sync with head!"),
                }
                if iter == rope_pairs {
                    board[*taily][*tailx] = true;
                }
                iter += 1;
            });
        }
    });
    // debug_print(&board);
    board.iter().flatten().filter(|&&v| v).count()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(input);
    let mut part1_rope = [(MID, MID); 2];
    let part1 = solution(&parsed_input, &mut part1_rope);
    let mut part2_rope = [(MID, MID); 10];
    let part2 = solution(&parsed_input, &mut part2_rope);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
