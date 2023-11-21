use aoc_shared::{read_input, try_atoi, Tokenize};
use std::io;

type Output = Vec<AuntFacts>;

#[derive(Default)]
struct AuntFacts {
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

impl AuntFacts {
    fn is_undefined(&self) -> bool {
        self.children.is_none()
            && self.cats.is_none()
            && self.samoyeds.is_none()
            && self.pomeranians.is_none()
            && self.akitas.is_none()
            && self.vizslas.is_none()
            && self.goldfish.is_none()
            && self.trees.is_none()
            && self.cars.is_none()
            && self.perfumes.is_none()
    }

    fn cmp_aunt_matching(&self, rhs: &Self) -> u8 {
        (self.children == rhs.children) as u8
            + (self.cats == rhs.cats) as u8
            + (self.samoyeds == rhs.samoyeds) as u8
            + (self.pomeranians == rhs.pomeranians) as u8
            + (self.akitas == rhs.akitas) as u8
            + (self.vizslas == rhs.vizslas) as u8
            + (self.goldfish == rhs.goldfish) as u8
            + (self.trees == rhs.trees) as u8
            + (self.cars == rhs.cars) as u8
            + (self.perfumes == rhs.perfumes) as u8
    }

    fn cmp_retroencabulated(&self, rhs: &Self) -> u8 {
        (self.children == rhs.children) as u8
            + (self.cats < rhs.cats) as u8
            + (self.samoyeds == rhs.samoyeds) as u8
            + (self.pomeranians > rhs.pomeranians) as u8
            + (self.akitas == rhs.akitas) as u8
            + (self.vizslas == rhs.vizslas) as u8
            + (self.goldfish > rhs.goldfish) as u8
            + (self.trees < rhs.trees) as u8
            + (self.cars == rhs.cars) as u8
            + (self.perfumes == rhs.perfumes) as u8
    }
}

const AUNT_X: AuntFacts = AuntFacts {
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

enum Tok {
    Uknk,
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

fn parse_input(input: &[u8]) -> Output {
    use aoc_shared::Token::*;
    use Tok::*;
    input
        .tokenize()
        .fold(
            (Vec::with_capacity(500), AuntFacts::default(), Uknk),
            |(mut acc, mut auntfact, wtype), tok| match tok {
                Something(word) => {
                    let mut next = Uknk;
                    match wtype {
                        Uknk => match word {
                            b"children" => next = Children,
                            b"cats" => next = Cats,
                            b"samoyeds" => next = Samoyeds,
                            b"pomeranians" => next = Pomeranians,
                            b"akitas" => next = Akitas,
                            b"vizslas" => next = Vizslas,
                            b"goldfish" => next = Goldfish,
                            b"trees" => next = Trees,
                            b"cars" => next = Cars,
                            b"perfumes" => next = Perfumes,
                            _ => (),
                        },
                        Children => {
                            auntfact.children = try_atoi::<i32, 10>(word);
                        }
                        Cats => {
                            auntfact.cats = try_atoi::<i32, 10>(word);
                        }
                        Samoyeds => {
                            auntfact.samoyeds = try_atoi::<i32, 10>(word);
                        }
                        Pomeranians => {
                            auntfact.pomeranians = try_atoi::<i32, 10>(word);
                        }
                        Akitas => {
                            auntfact.akitas = try_atoi::<i32, 10>(word);
                        }
                        Vizslas => {
                            auntfact.akitas = try_atoi::<i32, 10>(word);
                        }
                        Goldfish => {
                            auntfact.goldfish = try_atoi::<i32, 10>(word);
                        }
                        Trees => {
                            auntfact.trees = try_atoi::<i32, 10>(word);
                        }
                        Cars => {
                            auntfact.cars = try_atoi::<i32, 10>(word);
                        }
                        Perfumes => {
                            auntfact.perfumes = try_atoi::<i32, 10>(word);
                        }
                    };
                    (acc, auntfact, next)
                }
                Delimiter(b',') => (acc, auntfact, Uknk),
                Newline | DoubleNewline | End => {
                    if !auntfact.is_undefined() {
                        acc.push(auntfact)
                    }
                    (acc, AuntFacts::default(), Uknk)
                }
                _ => (acc, auntfact, wtype),
            },
        )
        .0
}

fn solve1(aunts: &[AuntFacts]) -> usize {
    aunts
        .iter()
        .enumerate()
        .map(|(pos, aunt)| (pos, AUNT_X.cmp_aunt_matching(aunt)))
        .max_by_key(|(_, amatch)| *amatch)
        .expect("expected one aunt to match.")
        .0
}

fn solve2(aunts: &[AuntFacts]) -> usize {
    aunts
        .iter()
        .enumerate()
        .map(|(pos, aunt)| (pos, AUNT_X.cmp_retroencabulated(aunt)))
        .max_by_key(|(_, amatch)| *amatch)
        .expect("expected one aunt to match.")
        .0
}
fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    // one based index.
    let part1 = solve1(&parsed_input) + 1;
    let part2 = solve2(&parsed_input) + 1;

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
