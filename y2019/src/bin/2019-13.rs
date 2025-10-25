use std::{
    fmt::Write,
    io::{self},
};
#[cfg(feature = "term")]
use std::{
    io::Write as WriteT,
    os::fd::AsFd,
    sync::mpsc::{self, Receiver},
    thread,
};

#[cfg(all(feature = "term", debug_assertions))]
use std::{thread::sleep, time::Duration};

#[cfg(feature = "term")]
use termion::{
    event::Key,
    get_tty,
    input::TermRead,
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
            TileId::Ball => f.write_char('●'),
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
fn get_terminal() -> (RawTerminal<std::fs::File>, Receiver<()>) {
    let mut tty = get_tty()
        .ok()
        .and_then(|tty| tty.into_raw_mode().ok())
        .expect("Need a raw tty.");
    write!(tty, "{}", termion::clear::All).unwrap();
    let quit = {
        let (send, recv) = mpsc::sync_channel(1);
        thread::spawn(move || {
            for k in get_tty().expect("Should be able to get input.").keys() {
                if matches!(
                    k.expect("to decode key"),
                    Key::Ctrl('c') | Key::Ctrl('d') | Key::Ctrl('z') | Key::Esc
                ) {
                    _ = send.send(());
                    break;
                }
            }
        });
        recv
    };
    (tty, quit)
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

const QUARTERS: i64 = 2;

fn run_program(mut program: Vec<i64>) -> (i64, i64) {
    let mut tiles: Vec<Vec<TileId>> = vec![];
    let mut intcode = IntCode::default();
    let mut input = None;
    let mut outstate = OutState::Init;

    let mut starting_blocks = 0;
    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    program[0] = QUARTERS;

    #[cfg(feature = "term")]
    let (mut tty, quit) = get_terminal();
    loop {
        #[cfg(feature = "term")]
        if let Ok(()) = quit.try_recv() {
            break;
        }
        match intcode.execute_til(&mut program, &mut input) {
            Ok(output) => {
                outstate = outstate.next(output).expect("Valid Output.");
                match outstate {
                    OutState::XYTile(x, y, tile) => {
                        set_tile(&mut tiles, x, y, tile);
                        match tile {
                            TileId::Paddle => paddle_x = x,
                            TileId::Ball => ball_x = x,
                            TileId::Block => starting_blocks += 1,
                            _ => (),
                        }
                    }
                    OutState::Score(s) => score = s,
                    _ => (),
                }
            }
            Err(IntCodeErr::NeedInput) => {
                #[cfg(feature = "term")]
                {
                    write_game_state(&mut tty, &tiles, score);
                    #[cfg(debug_assertions)]
                    sleep(Duration::from_millis(10));
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

    (starting_blocks, score)
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let (part1, part2) = run_program(program);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
