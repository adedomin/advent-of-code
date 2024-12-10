use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D};
use std::io;

#[derive(Default, Copy, Clone)]
enum Cucumber {
    #[default]
    Nil,
    East,
    South,
}

impl From<u8> for Cucumber {
    fn from(value: u8) -> Self {
        match value {
            b'>' => Self::East,
            b'v' => Self::South,
            _ => Self::Nil,
        }
    }
}

type Int = usize;
type Output = FlatVec2D<Cucumber>;

const EAST: usize = 0;
const SOUTH: usize = 1;

fn get_herds(map: &Output) -> [Vec<(usize, usize)>; 2] {
    let mut herds: [Vec<(usize, usize)>; 2] = [vec![], vec![]];
    map.xyrange().for_each(|xy| match map[xy] {
        Cucumber::Nil => (),
        Cucumber::East => herds[EAST].push(xy),
        Cucumber::South => herds[SOUTH].push(xy),
    });
    herds
}

fn part1_sol(mut map: Output) -> Int {
    let mut move_mat = vec![];
    for i in 1.. {
        let mut moved = false;
        let herds = get_herds(&map);
        for j in EAST..SOUTH + 1 {
            herds[j].iter().for_each(|&(x, y)| {
                let (nx, ny) = if j == EAST {
                    ((x + 1) % map.1, y)
                } else {
                    (x, (y + 1) % map.2)
                };
                if matches!(map[(nx, ny)], Cucumber::Nil) {
                    moved = true;
                    move_mat.push(((x, y), (nx, ny)));
                }
            });
            move_mat.drain(..).for_each(|(xy, nxy)| map.swap(xy, nxy));
        }

        if !moved {
            return i;
        }
    }
    unreachable!();
}

// fn part2_sol(map: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let part1 = part1_sol(parsed_input);
    // let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    // print!("Part2: {part2}");
    println!();
    Ok(())
}
