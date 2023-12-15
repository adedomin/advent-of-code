use aoc_shared::{atoi, read_input, Tokenize};
use std::{cmp, io};

struct Ingredient {
    cap: i64,
    dur: i64,
    flv: i64,
    txr: i64,
    cal: i64,
}

impl Ingredient {
    fn scores(&self, grams: i64) -> (i64, i64, i64, i64, i64) {
        (
            self.cap * grams,
            self.dur * grams,
            self.flv * grams,
            self.txr * grams,
            self.cal * grams,
        )
    }
}

enum Tokens {
    Name,
    Cap,
    Dur,
    Flv,
    Txr,
    Cal,
    Uknk,
}

#[derive(Default)]
struct Parse<'a>(
    Option<&'a [u8]>,
    Option<i64>,
    Option<i64>,
    Option<i64>,
    Option<i64>,
    Option<i64>,
);

impl<'a> TryFrom<Parse<'a>> for Ingredient {
    type Error = ();

    fn try_from(value: Parse<'a>) -> Result<Self, Self::Error> {
        match value {
            Parse(Some(_name), Some(cap), Some(dur), Some(flv), Some(txr), Some(cal)) => Ok(Self {
                cap,
                dur,
                flv,
                txr,
                cal,
            }),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &[u8]) -> Vec<Ingredient> {
    use aoc_shared::Token::*;
    use Tokens::*;
    input
        .tokenize()
        .fold(
            (Vec::new(), Name, 1, Parse::default()),
            |(mut acc, mut tok, sign, mut vals), lex| match lex {
                Something(word) => {
                    match tok {
                        Name => vals.0 = Some(word),
                        Cap => vals.1 = Some(sign * atoi::<i64, 10>(word)),
                        Dur => vals.2 = Some(sign * atoi::<i64, 10>(word)),
                        Flv => vals.3 = Some(sign * atoi::<i64, 10>(word)),
                        Txr => vals.4 = Some(sign * atoi::<i64, 10>(word)),
                        Cal => vals.5 = Some(sign * atoi::<i64, 10>(word)),
                        Uknk => match word {
                            b"capacity" => tok = Cap,
                            b"durability" => tok = Dur,
                            b"flavor" => tok = Flv,
                            b"texture" => tok = Txr,
                            b"calories" => tok = Cal,
                            _ => (),
                        },
                    }
                    (acc, tok, 1, vals)
                }
                Delimiter(b'-') => (acc, tok, -1, vals),
                Delimiter(_) => (acc, Uknk, 1, vals),
                Newline | End => {
                    if let Ok(igd) = vals.try_into() {
                        acc.push(igd);
                    }
                    (acc, Name, 1, Parse::default())
                }
                _ => (acc, tok, 1, vals),
            },
        )
        .0
}

enum Recur {
    Base(usize, u8),
    Loop(usize, u8, u8),
}

fn solve_p1(grams: u8, calorie_target: i64, ingredients: &[Ingredient]) -> (i64, i64) {
    let igd_cnt = ingredients.len();
    let mut loop_scores = vec![(0, 0, 0, 0, 0); igd_cnt];
    let mut max = i64::MIN;
    let mut max_cal_restrict = i64::MIN;

    // impl of stars and bars solver: K ingredients of N grams.
    let mut loop_ctrs = Vec::with_capacity(igd_cnt);
    loop_ctrs.push(Recur::Loop(0usize, 0u8, grams));
    while let Some(recur) = loop_ctrs.pop() {
        match recur {
            // last bar position is based on others, thus k - 1 loops.
            Recur::Base(pos, target) => {
                loop_scores[pos] = ingredients[pos].scores(target as i64);

                let (c, d, f, t, cl) = loop_scores.iter().fold(
                    (0, 0, 0, 0, 0),
                    |(ca, da, fa, ta, cla), (cb, db, fb, tb, clb)| {
                        (ca + cb, da + db, fa + fb, ta + tb, cla + clb)
                    },
                );
                let score = cmp::max(c, 0) * cmp::max(d, 0) * cmp::max(f, 0) * cmp::max(t, 0);

                if max < score {
                    max = score;
                }

                if cl == calorie_target && max_cal_restrict < score {
                    max_cal_restrict = score;
                }
            }
            Recur::Loop(pos, current, target) => {
                loop_scores[pos] = ingredients[pos].scores(current as i64);

                if current < target {
                    loop_ctrs.push(Recur::Loop(pos, current + 1, target));
                }

                if pos + 1 < igd_cnt - 1 {
                    loop_ctrs.push(Recur::Loop(pos + 1, 0, target - current));
                } else {
                    loop_ctrs.push(Recur::Base(pos + 1, target - current));
                }
            }
        }
    }
    (max, max_cal_restrict)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);

    let (part1, part2) = solve_p1(100, 500, &parsed_input);

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
