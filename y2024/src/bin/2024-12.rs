use aoc_shared::{debug, pad_to_flat2d, read_input, FlatVec2D};
use rustc_hash::FxHashMap;

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
struct X(u8);

impl From<u8> for X {
    fn from(value: u8) -> Self {
        match value {
            b'A'..=b'Z' => Self(value - b'A'),
            _ => OUT_OF_BOUNDS,
        }
    }
}

type Output = FlatVec2D<X>;
#[rustfmt::skip]
const CARD: [(isize, isize); 4] = {
    let (sx, sy) = (0,-1);
    let mut card = [(sx,sy); 4];
    let mut i = 1;
    while i < card.len() {
        card[i] = rot90(card[i-1]);
        i += 1;
    }
    card
};

const fn rot90((x, y): (isize, isize)) -> (isize, isize) {
    (-y, x)
}

const OUT_OF_BOUNDS: X = X(b'Z' + 1);

type ShapeBoundsSides = (usize, FxHashMap<(usize, usize), usize>, usize);

fn get_shape(
    map: &Output,
    visit: &mut FlatVec2D<bool>,
    (x, y): (usize, usize),
) -> Option<ShapeBoundsSides> {
    if std::mem::replace(&mut visit[(x, y)], true) || map[(x, y)] == OUT_OF_BOUNDS {
        return None;
    }

    let mut area = 1usize;
    let mut frontier = FxHashMap::default();
    let mut corners = 0usize;
    let mut stack = vec![(x, y)];
    let c = map[(x, y)];
    while let Some((x, y)) = stack.pop() {
        for (dx, dy) in CARD {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let n = map[(nx, ny)];

            if n != c {
                *frontier.entry((nx, ny)).or_default() += 1;
                let (cdx, cdy) = rot90((dx, dy));
                let (cnx, cny) = ((x as isize + cdx) as usize, (y as isize + cdy) as usize);
                let cn = map[(cnx, cny)];
                let (anx, any) = ((nx as isize + cdx) as usize, (ny as isize + cdy) as usize);
                let an = map[(anx, any)];
                debug!(
                    "Shape {c:?}({x},{y}): n:{n:?}({nx},{ny}) cn:{cn:?}({cnx},{cny}), an:{an:?}({anx},{any}), cn != c || an == c {}",
                    cn != c || an == c
                );
                corners += usize::from(cn != c || an == c);
            } else if !std::mem::replace(&mut visit[(nx, ny)], true) {
                stack.push((nx, ny));
                area += 1;
            }
        }
    }
    debug!("Shape {c:?}: {area}, {corners}");
    Some((area, frontier, corners))
}

fn solve(map: &Output) -> (usize, usize) {
    let mut visit = FlatVec2D::<bool>::new(map.1, map.2);
    let mut p1 = 0;
    let mut p2 = 0;
    let mut stack = vec![(1, 1)];
    while let Some((x, y)) = stack.pop() {
        let Some((area, bounds, sides)) = get_shape(map, &mut visit, (x, y)) else {
            continue;
        };
        let perimeter = bounds.values().sum::<usize>();
        p1 += area * perimeter;
        p2 += area * sides;
        stack.extend(bounds.into_keys());
    }
    (p1, p2)
}

fn main() -> std::io::Result<()> {
    let input = read_input()?;
    let parsed_input = pad_to_flat2d(&input, OUT_OF_BOUNDS);
    let (part1, part2) = solve(&parsed_input);
    print!("Part1: {part1}, Part2: {part2}");
    println!();
    Ok(())
}
