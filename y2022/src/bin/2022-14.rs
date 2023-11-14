use aoc_shared::{array_windows, fold_decimal, read_input, RecordGrouper, Sentinel, Token};
use std::{collections::HashSet, io};

fn plot_line((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    let (is_left, range) = match x1.cmp(&x2) {
        std::cmp::Ordering::Less => (true, x1..=x2),
        std::cmp::Ordering::Greater => (true, x2..=x1),
        std::cmp::Ordering::Equal => (
            false,
            match y1.cmp(&y2) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => y1..=y2,
                std::cmp::Ordering::Greater => y2..=y1,
            },
        ),
    };
    range.map(move |val| if is_left { (val, y1) } else { (x1, val) })
}

struct ParticleMap {
    pub chasm_start: i64,
    pub particles: HashSet<(i64, i64)>,
}

impl ParticleMap {
    pub fn new(particles: HashSet<(i64, i64)>) -> Self {
        let chasm_start = particles
            .iter()
            .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
            .expect("At least one particle to exist in particle map")
            .1;
        ParticleMap {
            chasm_start,
            particles,
        }
    }

    pub fn contains(&self, xy: (i64, i64)) -> bool {
        self.particles.contains(&xy)
    }

    pub fn add(&mut self, xy: (i64, i64)) -> bool {
        self.particles.insert(xy)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum SandDir {
    #[default]
    Down,
    Left,
    Right,
    Rest,
}

struct Sand {
    pub max_depth: i64,
    pub position: (i64, i64),
    direction: SandDir,
    prev_direction: SandDir,
}

impl Sand {
    pub fn new(max_depth: i64) -> Self {
        Sand {
            max_depth,
            position: (500, 0),
            direction: SandDir::Down,
            prev_direction: SandDir::Down,
        }
    }

    pub fn reset_one(&mut self) {
        (self.position, self.direction) = match self.prev_direction {
            SandDir::Down => ((self.position.0, self.position.1 - 1), SandDir::Left),
            SandDir::Left => ((self.position.0 + 1, self.position.1 - 1), SandDir::Right),
            SandDir::Right => ((self.position.0 - 1, self.position.1 - 1), SandDir::Rest),
            SandDir::Rest => {
                return;
            }
        };
    }

    pub fn is_freefalling(&self) -> bool {
        self.direction != SandDir::Rest
    }

    pub fn cannot_generate(&self) -> bool {
        self.position == (500, 0)
    }
}

impl Iterator for Sand {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.1 - 1 == self.max_depth {
            return None;
        }

        self.prev_direction = self.direction;
        self.direction = match self.direction {
            SandDir::Down => {
                self.position.1 += 1;
                self.direction
            }
            SandDir::Left => {
                self.position = (self.position.0 - 1, self.position.1 + 1);
                SandDir::Down
            }
            SandDir::Right => {
                self.position = (self.position.0 + 1, self.position.1 + 1);
                SandDir::Down
            }
            SandDir::Rest => return None,
        };

        Some(self.position)
    }
}

fn parse_input(input: &[u8]) -> ParticleMap {
    let tokenizer = RecordGrouper::new_with_rs(input, Token::Newline);
    let particles = tokenizer
        .map(|token_grp| {
            token_grp
                .iter()
                .fold(
                    (Vec::new(), Sentinel::Unset(0)),
                    |(mut acc, xcoord), token| match token {
                        Token::Something(num) if xcoord.is_unset() => {
                            (acc, Sentinel::Value(num.iter().fold(0i64, fold_decimal)))
                        }
                        Token::Something(num) => {
                            xcoord.map_mv(|xcoord| {
                                acc.push((xcoord, num.iter().fold(0i64, fold_decimal)));
                            });
                            (acc, Sentinel::Unset(0))
                        }
                        _ => (acc, xcoord),
                    },
                )
                .0
        })
        .flat_map(|coords| {
            array_windows(&coords)
                .flat_map(|&[start, end]| plot_line(start, end))
                .collect::<Vec<(i64, i64)>>()
        })
        .collect::<HashSet<(i64, i64)>>();

    ParticleMap::new(particles)
}

fn solve(mut particles: ParticleMap) -> (i64, i64) {
    let mut solv = 0;
    loop {
        let mut sand = Sand::new(particles.chasm_start);
        while let Some((sandx, sandy)) = sand.next() {
            if particles.contains((sandx, sandy)) {
                sand.reset_one();
            }
        }
        if sand.is_freefalling() {
            break;
        }

        particles.add(sand.position);
        solv += 1;
    }

    // part2
    particles.particles.extend(plot_line(
        (-2000, particles.chasm_start + 2),
        (3000, particles.chasm_start + 2),
    ));
    particles.chasm_start += 2;
    let mut solvp2 = solv;
    loop {
        let mut sand = Sand::new(particles.chasm_start);
        while let Some((sandx, sandy)) = sand.next() {
            if particles.contains((sandx, sandy)) {
                sand.reset_one();
            }
        }
        if sand.is_freefalling() {
            panic!("Should be landing below the base line.")
        }
        particles.add(sand.position);
        solvp2 += 1;
        if sand.cannot_generate() {
            break;
        }
    }

    (solv, solvp2)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let particles = parse_input(&input);
    let (part1, part2) = solve(particles);
    println!("Part1 {part1}, Part2 {part2}");
    Ok(())
}
