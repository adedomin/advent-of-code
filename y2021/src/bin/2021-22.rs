use aoc_shared::{fold_decimal_from, read_input, GroupTokenize, Token};
use std::io;

type Output = Vec<Cuboid>;
type Solved = i64;

#[derive(Clone)]
struct Cuboid {
    toggle: bool,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Cuboid {
    // this is the intersection between two cuboids.
    // if the other is a negative, we make a positive slice out of self.
    // if positive, we negate the intersection from other.
    fn intersect(&self, other: &Self) -> Option<Cuboid> {
        let toggle = !other.toggle;
        let x0 = std::cmp::max(self.x.0, other.x.0);
        let x1 = std::cmp::min(self.x.1, other.x.1);
        let y0 = std::cmp::max(self.y.0, other.y.0);
        let y1 = std::cmp::min(self.y.1, other.y.1);
        let z0 = std::cmp::max(self.z.0, other.z.0);
        let z1 = std::cmp::min(self.z.1, other.z.1);

        if x0 > x1 || y0 > y1 || z0 > z1 {
            None
        } else {
            Some(Cuboid {
                toggle,
                x: (x0, x1),
                y: (y0, y1),
                z: (z0, z1),
            })
        }
    }
}

fn parse_input(input: &[u8]) -> Output {
    input
        .group_tokens(Token::Newline)
        .map(|toks| {
            let mut toks = toks.into_iter();
            let toggle = toks.next().expect("Line to have content");
            let toggle = matches!(toggle, Token::Something(b"on"));
            let mut vals: [(Option<i64>, Option<i64>); 3] =
                [(None, None), (None, None), (None, None)];
            let mut vi = 4usize;
            let mut sign = 1;
            toks.for_each(|tok| match tok {
                Token::Something(w) if w[0].is_ascii_alphabetic() => vi = (w[0] - b'x') as usize,
                Token::Something(w) => {
                    if vals[vi].0.is_none() {
                        vals[vi].0 = Some(sign * fold_decimal_from::<i64>(w));
                        sign = 1;
                    } else {
                        vals[vi].1 = Some(sign * fold_decimal_from::<i64>(w));
                        sign = 1;
                    }
                }
                Token::Delimiter(b'-') => sign = -1,
                _ => (),
            });
            Cuboid {
                toggle,
                x: (vals[0].0.unwrap(), vals[0].1.unwrap()),
                y: (vals[1].0.unwrap(), vals[1].1.unwrap()),
                z: (vals[2].0.unwrap(), vals[2].1.unwrap()),
            }
        })
        .collect::<Vec<_>>()
}

fn p1_filter(n: &i64) -> bool {
    (-50..51).contains(n)
}

fn part1_sol(input: &Output) -> Solved {
    let filtered = input
        .iter()
        .flat_map(|c| {
            if [c.x.0, c.x.1, c.y.0, c.y.1, c.z.0, c.z.1]
                .iter()
                .all(p1_filter)
            {
                Some(c.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    part2_sol(filtered)
}

fn part2_sol(input: Output) -> Solved {
    // note that the order is important.
    let mut divcubes = vec![];
    input.into_iter().for_each(|cuboid| {
        let intersections = divcubes
            .iter()
            .flat_map(|cube| cuboid.intersect(cube))
            .collect::<Vec<_>>();
        if cuboid.toggle {
            divcubes.push(cuboid.clone());
        }
        divcubes.extend(intersections);
    });

    divcubes
        .into_iter()
        .map(|Cuboid { toggle, x, y, z }| {
            let sign = if toggle { 1 } else { -1 };
            sign * (x.1 - x.0 + 1) * (y.1 - y.0 + 1) * (z.1 - z.0 + 1)
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
