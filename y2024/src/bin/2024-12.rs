use aoc_shared::{debug, pad_to_flat2d, read_input, FlatVec2D};

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

const OUT_OF_BOUNDS: X = X(u8::MAX);

type Output = FlatVec2D<X>;

const fn rot90((x, y): (isize, isize)) -> (isize, isize) {
    (-y, x)
}

const CARD: [(isize, isize); 4] = {
    let (sx, sy) = (0, -1);
    let mut card = [(sx, sy); 4];
    let mut i = 1;
    while i < card.len() {
        card[i] = rot90(card[i - 1]);
        i += 1;
    }
    card
};

type ShapeBoundsSides = (usize, usize, usize, Vec<(usize, usize)>);

/// Get right as in clockwise 90deg from given vector dx, dy.
fn get_right(x: usize, y: usize, dx: isize, dy: isize) -> (usize, usize) {
    let (cdx, cdy) = rot90((dx, dy));
    ((x as isize + cdx) as usize, (y as isize + cdy) as usize)
}

fn get_shape(
    map: &Output,
    visit: &mut FlatVec2D<bool>,
    (x, y): (usize, usize),
) -> Option<ShapeBoundsSides> {
    if std::mem::replace(&mut visit[(x, y)], true) || map[(x, y)] == OUT_OF_BOUNDS {
        return None;
    }

    let mut area = 1usize;
    let mut perim = 0usize;
    let mut corners = 0usize;
    let mut frontier = vec![];
    let mut stack = vec![(x, y)];
    let c = map[(x, y)];

    while let Some((x, y)) = stack.pop() {
        for (dx, dy) in CARD {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let n = map[(nx, ny)];

            if n != c {
                perim += 1;

                // Check the immediate cw 90deg rot from the frontier node.
                // |         * <----- Frontier (nx, ny)
                // |___________
                // (x, y) -> * | * <- What we're looking for here.
                let cn = map[get_right(x, y, dx, dy)];
                let is_right_edge = cn != c;

                // We now check to see if we're a concave corner
                //  Frontier (nx, ny) --> * | * <--- What we're looking for here.
                //  ________________________|   <--- Same shape (color)
                // |           (x, y) --> *     <-/
                let an = map[get_right(nx, ny, dx, dy)];
                let is_concave = an == c;

                if is_right_edge || is_concave {
                    corners += 1;
                }

                frontier.push((nx, ny));
            } else if !std::mem::replace(&mut visit[(nx, ny)], true) {
                area += 1;

                stack.push((nx, ny));
            }
        }
    }
    debug!("Shape {c:?}: area:{area}, perimeter:{perim}, sides:{corners}");
    Some((area, perim, corners, frontier))
}

fn solve(map: &Output) -> (usize, usize) {
    let mut visit = FlatVec2D::<bool>::new(map.1, map.2);
    let mut p1 = 0;
    let mut p2 = 0;
    let mut stack = vec![(1, 1)];
    while let Some((x, y)) = stack.pop() {
        let Some((area, perimeter, sides, frontier)) = get_shape(map, &mut visit, (x, y)) else {
            continue;
        };
        p1 += area * perimeter;
        p2 += area * sides;
        stack.extend(frontier);
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
