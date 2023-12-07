use aoc_shared::{atoi, destructure_or_none, read_input, Token, Tokenize};
use itertools::Itertools;
use std::{fmt::Write, io, iter::zip};

type Output = Vec<(Hand, i32)>;

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct CamelCard(pub u8);

impl TryFrom<u8> for CamelCard {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'j' => Ok(CamelCard(0)), // wild joker
            b'2' => Ok(CamelCard(1)),
            b'3' => Ok(CamelCard(2)),
            b'4' => Ok(CamelCard(3)),
            b'5' => Ok(CamelCard(4)),
            b'6' => Ok(CamelCard(5)),
            b'7' => Ok(CamelCard(6)),
            b'8' => Ok(CamelCard(7)),
            b'9' => Ok(CamelCard(8)),
            b'T' => Ok(CamelCard(9)),
            b'J' => Ok(CamelCard(10)), // part1 joker
            b'Q' => Ok(CamelCard(11)),
            b'K' => Ok(CamelCard(12)),
            b'A' => Ok(CamelCard(13)),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for CamelCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chr = match self.0 {
            0 => 'j',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            8 => '9',
            9 => 'T',
            10 => 'J',
            11 => 'Q',
            12 => 'K',
            13 => 'A',
            _ => unreachable!(),
        };
        f.write_char(chr)
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Hand(pub [CamelCard; 5]);

const JOKER: CamelCard = CamelCard(10);
const JOKER_WILD: CamelCard = CamelCard(0);

impl Hand {
    fn histogram(&self) -> [u8; 5] {
        let mut hist = [0u8; 14];

        self.0.iter().for_each(|card| hist[card.0 as usize] += 1);

        let (jokers_wild, hist) = hist.split_at_mut(1);
        let jokers_wild = jokers_wild[0];

        hist.sort();
        let mut hist: [u8; 5] = hist[8..].try_into().unwrap();
        // add in wild cards to best match
        hist[4] += jokers_wild;
        hist
    }

    fn jokers_wild(self) -> Self {
        let Self(mut orig) = self;
        orig.iter_mut().for_each(|card| {
            if *card == JOKER {
                *card = JOKER_WILD;
            }
        });
        Self(orig)
    }
}

impl From<[CamelCard; 5]> for Hand {
    fn from(value: [CamelCard; 5]) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            std::fmt::Display::fmt(&c, f)?;
        }
        f.write_char(' ')?;
        let ht: HandType = self.into();
        std::fmt::Debug::fmt(&ht, f)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ht1: HandType = self.into();
        let ht2: HandType = other.into();
        let c = ht1.cmp(&ht2);
        if matches!(c, std::cmp::Ordering::Equal) {
            for (s, o) in zip(self.0, other.0) {
                let c2 = s.cmp(&o);
                if !matches!(c2, std::cmp::Ordering::Equal) {
                    return c2;
                }
            }
            c
        } else {
            c
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        match value.histogram() {
            [1, 1, 1, 1, 1] => Self::HighCard,
            [0, 1, 1, 1, 2] => Self::OnePair,
            [0, 0, 1, 2, 2] => Self::TwoPair,
            [0, 0, 1, 1, 3] => Self::ThreeKind,
            [0, 0, 0, 2, 3] => Self::FullHouse,
            [0, 0, 0, 1, 4] => Self::FourKind,
            [0, 0, 0, 0, 5] => Self::FiveKind,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &[u8]) -> Output {
    input
        .tokenize()
        .flat_map(|t| destructure_or_none!(Token::Something|word| = t))
        .tuples()
        .map(|(cards, bid)| {
            if cards.len() != 5 {
                panic!("Input card hand {cards:?} is not length 5.");
            }

            let mut c2 = [CamelCard::default(); 5];
            cards.iter().enumerate().for_each(|(pos, &card)| {
                c2[pos] = card.try_into().expect("Card, {card:?} is not in range.");
            });
            (c2.into(), atoi::<i32, 10>(bid))
        })
        .collect_vec()
}

fn solve1(mut input: Vec<(Hand, i32)>) -> i32 {
    input.sort_by(|(h, _), (h2, _)| h.cmp(h2));
    input
        .iter()
        .enumerate()
        .map(|(pos, (_, bid))| {
            let rank = (pos + 1) as i32;
            bid * rank
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    #[cfg(debug_assertions)]
    {
        for (hand, bid) in parsed_input.iter() {
            println!("{hand}, bid {bid}");
        }
    }
    let part1 = solve1(parsed_input.clone());

    let wild = parsed_input
        .into_iter()
        .map(|(hand, bid)| (hand.jokers_wild(), bid))
        .collect_vec();
    #[cfg(debug_assertions)]
    {
        println!("----- Jokers Wild");
        for (hand, bid) in wild.iter() {
            println!("{hand}, bid {bid}");
        }
    }
    let part2 = solve1(wild);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
