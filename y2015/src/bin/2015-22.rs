use aoc_shared::{read_input_to_string, try_atoi};
use std::{
    collections::{BinaryHeap, HashMap},
    io,
};

type Output = Character;
type Solved = u32;

const PLAYER_HP: i32 = 50;
const MANA_START: i32 = 500;

#[derive(Clone, Copy)]
struct Character {
    health: i32,
    attack: i32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> u32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn duration(&self) -> u8 {
        match self {
            Spell::MagicMissile => 0,
            Spell::Drain => 0,
            Spell::Shield => 6,
            Spell::Poison => 6,
            Spell::Recharge => 5,
        }
    }

    fn effect_idx(&self) -> Option<usize> {
        match self {
            Spell::MagicMissile => None,
            Spell::Drain => None,
            Spell::Shield => Some(0),
            Spell::Poison => Some(1),
            Spell::Recharge => Some(2),
        }
    }

    fn attack_pow(&self) -> i32 {
        match self {
            Spell::MagicMissile => 4,
            Spell::Drain => 2,
            Spell::Shield => 0,
            Spell::Poison => 0,
            Spell::Recharge => 0,
        }
    }
}

const SPELLS: [Spell; 5] = [
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

fn parse_input(input: &str) -> Output {
    let vals = input
        .split_ascii_whitespace()
        .filter_map(|str| try_atoi::<_, 10>(str.as_bytes()))
        .collect::<Vec<_>>();
    assert_eq!(vals.len(), 2);
    Character {
        health: vals[0],
        attack: vals[1],
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    spell: Spell,
    effects: [u8; 3],
    mana: i32,
    player_hp: i32,
    enemy_hp: i32,
    // path: Vec<Spell>,
}

#[derive(Clone, PartialEq, Eq)]
struct HeapState {
    key: Key,
    cost: u32,
}

impl Ord for HeapState {
    /// Reverse of the Ord impl for min heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn attack_minus_armor(attack: i32, has_armor: bool) -> i32 {
    if has_armor {
        if attack - 7 > 0 {
            attack - 7
        } else {
            1
        }
    } else {
        attack
    }
}

fn part1_sol(enemy: Output) -> Solved {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    for spell in SPELLS {
        let key = Key {
            spell,
            effects: [0, 0, 0],
            mana: MANA_START,
            player_hp: PLAYER_HP,
            enemy_hp: enemy.health,
            // path: vec![spell],
        };
        dist.insert(key.clone(), spell.cost());
        heap.push(HeapState {
            key,
            cost: spell.cost(),
        });
    }

    while let Some(HeapState { mut key, cost }) = heap.pop() {
        if key.enemy_hp < 1 {
            // println!("{:?}", key.path);
            return cost - key.spell.cost();
        } else if key.player_hp < 1 || key.mana < 1 {
            continue;
        }

        // have to deincrement at start of turn, noop if the effect isn't set.
        key.effects
            .iter_mut()
            .for_each(|e| *e = e.saturating_sub(1));

        if let Some(idx) = key.spell.effect_idx() {
            if key.effects[idx] > 0 {
                continue;
            }
        }

        match key.spell {
            Spell::Shield | Spell::Poison | Spell::Recharge => {
                key.effects[key.spell.effect_idx().unwrap()] = key.spell.duration()
            }
            _ => (),
        }

        let enemy_hp = key.enemy_hp
            - (key.spell.attack_pow()
                + if key.effects[Spell::Poison.effect_idx().unwrap()] == Spell::Poison.duration() {
                    3
                } else if key.effects[Spell::Poison.effect_idx().unwrap()] > 0 {
                    3 * 2 // does damage on player/enemy turn but only if not just cast.
                } else {
                    0
                });

        let player_hp = key.player_hp
            - attack_minus_armor(
                enemy.attack,
                key.effects[Spell::Shield.effect_idx().unwrap()] > 0,
            )
            + if key.spell == Spell::Drain { 2 } else { 0 };

        let mana = key.mana - key.spell.cost() as i32
            + if key.effects[Spell::Recharge.effect_idx().unwrap()] == Spell::Recharge.duration() {
                101
            } else if key.effects[Spell::Recharge.effect_idx().unwrap()] > 0 {
                101 * 2
            } else {
                0
            };

        key.effects
            .iter_mut()
            .for_each(|e| *e = e.saturating_sub(1));

        for spell in SPELLS {
            // let mut npath = key.path.clone();
            // npath.push(spell);
            let nkey = Key {
                spell,
                effects: key.effects,
                mana,
                player_hp,
                enemy_hp,
                // path: npath,
            };
            let ncost = cost + spell.cost();
            let dent = dist.entry(nkey.clone()).or_insert(u32::MAX);
            if ncost < *dent {
                *dent = ncost;
                heap.push(HeapState {
                    key: nkey,
                    cost: ncost,
                });
            }
        }
    }
    panic!("No solution");
}

// fn part2_sol(enemy: Output) -> Solved {
// }

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(parsed_input);
    // let part2 = part2_sol(parsed_input);
    print!("Part1: {part1}, ");
    // print!("Part2: {part2}");
    println!();
    Ok(())
}
