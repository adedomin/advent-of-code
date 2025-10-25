use std::{
    fmt::Write,
    io::{self},
};
#[cfg(feature = "term")]
use std::{io::Write as WriteT, os::fd::AsFd};

#[cfg(feature = "term")]
use termion::{
    get_tty,
    raw::{IntoRawMode, RawTerminal},
};

use y2019::intcode::{brk, read_intcode, IntCode, IntCodeErr};

#[derive(Copy, Clone)]
enum TileId {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl TryFrom<i64> for TileId {
    type Error = OutError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => return Err(OutError::InvalidTileId),
        })
    }
}

impl std::fmt::Display for TileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileId::Empty => f.write_char('.'),
            TileId::Wall => f.write_char('▞'),
            TileId::Block => f.write_char('▓'),
            TileId::Paddle => f.write_char('▀'),
            TileId::Ball => f.write_char('⬤'),
        }
    }
}

enum OutState {
    Init,
    X(i64),
    Y(i64, i64),
    XYTile(usize, usize, TileId),
    Score(i64),
}

#[derive(Debug)]
enum OutError {
    NegativeIndex,
    InvalidTileId,
}

fn to_idx(i: i64) -> Result<usize, OutError> {
    usize::try_from(i).or(Err(OutError::NegativeIndex))
}

impl OutState {
    fn next(self, out: i64) -> Result<Self, OutError> {
        Ok(match self {
            OutState::Init => OutState::X(out),
            OutState::X(o1) => OutState::Y(o1, out),
            OutState::Y(-1, 0) => OutState::Score(out),
            OutState::Score(_) => OutState::X(out),
            OutState::Y(o1, o2) => OutState::XYTile(to_idx(o1)?, to_idx(o2)?, out.try_into()?),
            OutState::XYTile(_, _, _) => OutState::X(out),
        })
    }
}

#[cfg(feature = "term")]
fn write_game_state<W: WriteT + AsFd>(tty: &mut RawTerminal<W>, tiles: &[Vec<TileId>], score: i64) {
    write!(tty, "{}", termion::cursor::Goto(1, 1),).unwrap();
    for line in tiles {
        for tile in line {
            write!(tty, "{tile}").unwrap();
        }
        write!(tty, "\r\n",).unwrap();
    }
    write!(tty, "Score: {score}\r\n",).unwrap();
}

fn set_tile(tiles: &mut Vec<Vec<TileId>>, x: usize, y: usize, new_tile: TileId) {
    let line = match tiles.get_mut(y) {
        Some(line) => line,
        None => {
            tiles.resize_with(y + 1, Vec::new);
            &mut tiles[y]
        }
    };

    let tile = match line.get_mut(x) {
        Some(tile) => tile,
        None => {
            line.resize_with(x + 1, || TileId::Empty);
            &mut line[x]
        }
    };
    *tile = new_tile
}

fn run_program(mut program: Vec<i64>, quarters: Option<i64>) -> i64 {
    let mut tiles: Vec<Vec<TileId>> = vec![];
    let mut intcode = IntCode::default();
    let mut input = None;
    let mut outstate = OutState::Init;
    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    #[cfg(feature = "term")]
    let mut tty = None;
    if let Some(quarters) = quarters {
        program[0] = quarters;
        #[cfg(feature = "term")]
        {
            let mut out = get_tty()
                .ok()
                .and_then(|tty| tty.into_raw_mode().ok())
                .expect("Need a raw tty.");
            write!(out, "{}", termion::clear::All).unwrap();
            tty = Some(out);
        }
    }
    loop {
        match intcode.execute_til(&mut program, &mut input) {
            Ok(output) => {
                outstate = outstate.next(output).expect("Valid Output.");
                match outstate {
                    OutState::XYTile(x, y, tile) => {
                        set_tile(&mut tiles, x, y, tile);
                        match tile {
                            TileId::Paddle => paddle_x = x,
                            TileId::Ball => ball_x = x,
                            _ => (),
                        }
                    }
                    OutState::Score(s) => score = s,
                    _ => (),
                }
            }
            Err(IntCodeErr::NeedInput) => {
                #[cfg(feature = "term")]
                if let Some(out) = tty.as_mut() {
                    #[cfg(debug_assertions)]
                    use std::{thread::sleep, time::Duration};

                    write_game_state(out, &tiles, score);
                    #[cfg(debug_assertions)]
                    sleep(Duration::from_millis(10));
                } else {
                    panic!("Must have a TTY to interact with the game!");
                }
                match ball_x.cmp(&paddle_x) {
                    std::cmp::Ordering::Less => input = Some(-1),
                    std::cmp::Ordering::Equal => input = Some(0),
                    std::cmp::Ordering::Greater => input = Some(1),
                }
            }
            Err(IntCodeErr::OutOfBounds(fault)) => {
                brk(fault, &mut program).expect("Resize program")
            }
            Err(IntCodeErr::End) => break,
            Err(e) => panic!("{e}"),
        }
    }
    if quarters.is_none() {
        tiles
            .into_iter()
            .flatten()
            .filter(|t| matches!(t, TileId::Block))
            .count() as i64
    } else {
        score
    }
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let part1 = run_program(program.clone(), None);
    let part2 = run_program(program.clone(), Some(2));
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
