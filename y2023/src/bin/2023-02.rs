use aoc_shared::{read_input, try_atoi, Token, Tokenize};
use std::io;

type Output = Vec<Vec<Game>>;

#[derive(Default, Debug)]
struct Game {
    r: i32,
    g: i32,
    b: i32,
}

fn parse_input(input: &[u8]) -> Output {
    let mut ret = vec![];
    let mut cur_num = 0;
    for token in input.tokenize() {
        match token {
            Token::Something(b"Game") => {
                ret.push(vec![Game::default()]);
            }
            Token::Something(b"red") => ret.last_mut().unwrap().last_mut().unwrap().r = cur_num,
            Token::Something(b"green") => ret.last_mut().unwrap().last_mut().unwrap().g = cur_num,
            Token::Something(b"blue") => ret.last_mut().unwrap().last_mut().unwrap().b = cur_num,
            Token::Something(num) => {
                cur_num = try_atoi::<i32, 10>(num).unwrap_or(cur_num);
            }
            Token::Delimiter(b';') => ret.last_mut().unwrap().push(Game::default()),
            _ => (),
        }
    }

    ret
}

fn glecmp(lhs: &Game, rhs: &Game) -> bool {
    lhs.r <= rhs.r && lhs.g <= rhs.g && lhs.b <= rhs.b
}

const P1MAX_GAME: Game = Game {
    r: 12,
    g: 13,
    b: 14,
};

fn solve1(i: &[Vec<Game>]) -> i32 {
    i.iter()
        .enumerate()
        .map(|(pos, games)| {
            if games.iter().all(|game| glecmp(game, &P1MAX_GAME)) {
                (pos + 1) as i32
            } else {
                0
            }
        })
        .sum()
}

fn gmax(lhs: &Game, rhs: &Game) -> Game {
    Game {
        r: std::cmp::max(lhs.r, rhs.r),
        g: std::cmp::max(lhs.g, rhs.g),
        b: std::cmp::max(lhs.b, rhs.b),
    }
}

fn gpower(i: Game) -> i32 {
    i.r * i.g * i.b
}

fn solve2(i: &[Vec<Game>]) -> i32 {
    i.iter()
        .map(|games| {
            games
                .iter()
                .fold(Game::default(), |acc, game| gmax(&acc, game))
        })
        .map(gpower)
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = solve1(&parsed_input);
    let part2 = solve2(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
