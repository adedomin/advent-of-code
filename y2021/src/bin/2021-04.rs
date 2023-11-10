use std::{collections::HashMap, io};

use aoc_shared::read_input;
use itertools::Itertools;

struct Board {
    bitmap: i32,
    board_sum: i32,
    won: bool,
}

struct BoardPos {
    board_idx: usize,
    board_loc: i32,
}

fn fold_decimal(acc: i32, chr: &u8) -> i32 {
    acc * 10 + (chr - b'0') as i32
}

fn parse_numbers(numbers: &[u8]) -> Vec<i32> {
    numbers
        .iter()
        .group_by(|&&chr| chr != b',')
        .into_iter()
        .filter(|(t, _)| *t)
        .map(|(_, digits)| digits.fold(0, fold_decimal))
        .collect_vec()
}

fn parse_boards(boards: &[u8]) -> (Vec<Board>, HashMap<i32, Vec<BoardPos>>) {
    let (board, board_map, _, _) = boards
        .iter()
        .group_by(|&&chr| chr != b'\n' && chr != b' ')
        .into_iter()
        .filter(|(t, _)| *t)
        .fold(
            (
                Vec::<Board>::new(), // for some reason rust needed the types
                HashMap::<i32, Vec<BoardPos>>::new(),
                0i32,
                0i32,
            ),
            |(mut boards, mut board_map, count, sum), (_, digits)| {
                let num = digits.fold(0, fold_decimal);
                let bmap_data = BoardPos {
                    board_idx: boards.len(),
                    board_loc: count,
                };

                match board_map.get_mut(&num) {
                    Some(brd) => brd.push(bmap_data),
                    None => {
                        board_map.insert(num, vec![bmap_data]);
                    }
                }

                if count == 24 {
                    boards.push(Board {
                        bitmap: 0,
                        board_sum: sum + num,
                        won: false,
                    });
                    (boards, board_map, 0, 0)
                } else {
                    (boards, board_map, count + 1, sum + num)
                }
            },
        );
    (board, board_map)
}

fn board_won(board: i32) -> bool {
    for pos in 0..5 {
        let mult = 5 * pos;
        let lat = 0b11111 << mult;
        let lon = 1 << pos | 1 << (5 + pos) | 1 << (10 + pos) | 1 << (15 + pos) | 1 << (20 + pos);
        if (board & lat == lat) || (board & lon == lon) {
            return true;
        }
    }
    false
}

fn board_pos_marked(board: i32, loc: i32) -> bool {
    board & (1 << loc) != 0
}

fn solve(
    numbers: Vec<i32>,
    mut boards: Vec<Board>,
    board_map: HashMap<i32, Vec<BoardPos>>,
) -> (i32, i32) {
    let mut first_win = -1;
    let mut last_win = -1;
    for number in numbers {
        let boards_to_mark = match board_map.get(&number) {
            Some(x) => x,
            None => continue,
        };
        for BoardPos {
            board_idx,
            board_loc,
        } in boards_to_mark
        {
            let Board {
                bitmap,
                board_sum,
                won,
            } = boards[*board_idx];
            if won {
                continue;
            }
            if !board_pos_marked(bitmap, *board_loc) {
                let new_bitmap = bitmap | (1 << *board_loc);
                let new_sum = board_sum - number;
                let wins = board_won(new_bitmap);
                boards[*board_idx] = Board {
                    bitmap: new_bitmap,
                    board_sum: new_sum,
                    won: wins,
                };
                if wins && first_win == -1 {
                    first_win = new_sum * number;
                } else if wins {
                    last_win = new_sum * number;
                }
            }
        }
    }
    (first_win, last_win)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let (numbers, boards) =
        input.split_at(input.iter().find_position(|&&chr| chr == b'\n').unwrap().0);
    let numbers = parse_numbers(numbers);
    let (game_boards, board_map) = parse_boards(boards);
    let (p1, p2) = solve(numbers, game_boards, board_map);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
