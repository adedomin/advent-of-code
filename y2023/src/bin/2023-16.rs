use aoc_shared::{pad_to_flat2d, read_input, FlatVec2D};
use std::io;

#[derive(Copy, Clone, Default)]
enum Mirror {
    #[default]
    Edge,
    Empty,
    HoriSplit,
    VertSplit,
    BackwardR,
    ForwardR,
}

impl std::fmt::Debug for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mirror::Edge => Ok(()),
            Mirror::Empty => f.write_str("."),
            Mirror::HoriSplit => f.write_str("-"),
            Mirror::VertSplit => f.write_str("|"),
            Mirror::BackwardR => f.write_str("\\"),
            Mirror::ForwardR => f.write_str("/"),
        }
    }
}

impl From<u8> for Mirror {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'-' => Self::HoriSplit,
            b'|' => Self::VertSplit,
            b'/' => Self::ForwardR,
            b'\\' => Self::BackwardR,
            _ => Self::Edge,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Cardinal {
    N,
    E,
    S,
    W,
}

impl From<Cardinal> for u8 {
    fn from(value: Cardinal) -> Self {
        match value {
            Cardinal::N => 0b0001,
            Cardinal::E => 0b0010,
            Cardinal::S => 0b0100,
            Cardinal::W => 0b1000,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Move(usize, usize, Cardinal);

enum Deflect {
    Nil,
    One(Move),
    Two(Move, Move),
}

#[derive(Default, Clone)]
struct Energized((), (), u8);

impl Energized {
    pub fn energized(&self) -> bool {
        self.2 != 0
    }

    pub fn add_cardinal(&mut self, card: Cardinal) -> bool {
        let card = u8::from(card);
        if self.2 & card != 0 {
            false
        } else {
            self.2 |= card;
            true
        }
    }
}

impl std::fmt::Debug for Energized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.energized() {
            f.write_str("#")
        } else {
            f.write_str(".")
        }
    }
}

impl Mirror {
    fn deflect(&self, Move(x, y, to): Move) -> Deflect {
        use Cardinal::*;
        use Deflect::*;
        match (self, to) {
            (Self::Edge, _) => Nil,
            (Self::Empty, N) => One(Move(x, y - 1, N)),
            (Self::Empty, E) => One(Move(x + 1, y, E)),
            (Self::Empty, S) => One(Move(x, y + 1, S)),
            (Self::Empty, W) => One(Move(x - 1, y, W)),
            (Self::HoriSplit, N) => Two(Move(x - 1, y, W), Move(x + 1, y, E)),
            (Self::HoriSplit, S) => Two(Move(x - 1, y, W), Move(x + 1, y, E)),
            (Self::HoriSplit, E) => One(Move(x + 1, y, E)),
            (Self::HoriSplit, W) => One(Move(x - 1, y, W)),
            (Self::VertSplit, N) => One(Move(x, y - 1, N)),
            (Self::VertSplit, S) => One(Move(x, y + 1, S)),
            (Self::VertSplit, E) => Two(Move(x, y - 1, N), Move(x, y + 1, S)),
            (Self::VertSplit, W) => Two(Move(x, y - 1, N), Move(x, y + 1, S)),
            (Self::ForwardR, N) => One(Move(x + 1, y, E)),
            (Self::ForwardR, E) => One(Move(x, y - 1, N)),
            (Self::ForwardR, S) => One(Move(x - 1, y, W)),
            (Self::ForwardR, W) => One(Move(x, y + 1, S)),
            (Self::BackwardR, N) => One(Move(x - 1, y, W)),
            (Self::BackwardR, E) => One(Move(x, y + 1, S)),
            (Self::BackwardR, S) => One(Move(x + 1, y, E)),
            (Self::BackwardR, W) => One(Move(x, y - 1, N)),
        }
    }
}

fn solve(grid: &FlatVec2D<Mirror>, start: Move) -> usize {
    let mut ret = FlatVec2D::<Energized>::new(grid.1, grid.2);
    let mut stack = vec![start];
    ret[(start.0, start.1)].add_cardinal(start.2);
    while let Some(m) = stack.pop() {
        let mirror = grid[(m.0, m.1)];
        match mirror.deflect(m) {
            Deflect::Nil => (),
            Deflect::One(m1) => {
                if ret[(m1.0, m1.1)].add_cardinal(m1.2) {
                    stack.push(m1);
                }
            }
            Deflect::Two(m1, m2) => {
                if ret[(m1.0, m1.1)].add_cardinal(m1.2) {
                    stack.push(m1);
                }
                if ret[(m2.0, m2.1)].add_cardinal(m2.2) {
                    stack.push(m2);
                }
            }
        }
    }

    let mut r = 0;
    for y in ret.pad_yrange() {
        for x in ret.pad_xrange() {
            if ret[(x, y)].energized() {
                r += 1;
            }
        }
    }
    r
}

fn main() -> io::Result<()> {
    use Cardinal::*;
    let input = read_input()?;
    let parsed_input = pad_to_flat2d(&input, Mirror::Edge);

    let part1 = solve(&parsed_input, Move(1, 1, E));
    print!("Part1: {part1}, ");
    let xmax = parsed_input.1 - 2;
    let ymax = parsed_input.2 - 2;
    let part2 = parsed_input
        .pad_xrange()
        .flat_map(|x| [Move(x, 1, S), Move(x, ymax, N)])
        .chain(
            parsed_input
                .pad_yrange()
                .flat_map(|y| [Move(1, y, E), Move(xmax, y, W)]),
        )
        .map(|m| solve(&parsed_input, m))
        .max()
        .unwrap();
    print!("Part2: {part2}");
    println!();
    Ok(())
}
