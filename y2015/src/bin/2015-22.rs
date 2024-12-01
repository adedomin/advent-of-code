use aoc_shared::{read_input_to_string, try_atoi, Dijkstra, HeapState};
use std::io;

type Output = Character;
type Solved = u16;

const PLAYER_HP: i16 = 50;
const MANA_START: i16 = 500;

#[derive(Clone, Copy)]
struct Character {
    health: i16,
    attack: i16,
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
    fn cost(&self) -> u16 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn attack_pow(&self) -> i16 {
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
    shield: u8,
    poison: u8,
    recharge: u8,
    player_hp: i16,
    mana: i16,
    enemy_hp: i16,
    // path: Vec<Spell>,
}

fn attack_minus_armor(attack: i16, armor: i16) -> i16 {
    if attack - armor > 0 {
        attack - armor
    } else {
        1
    }
}

struct TurnEffects {
    shield_stat: i16,
    poison_dmg: i16,
    mana_regen: i16,
}

fn run_effects(shield: u8, poison: u8, recharge: u8) -> (TurnEffects, u8, u8, u8) {
    let mut shield_stat = 0;
    let shield = shield
        .checked_sub(1)
        .inspect(|_| shield_stat = 7)
        .unwrap_or(0);
    let mut poison_dmg = 0;
    let poison = poison
        .checked_sub(1)
        .inspect(|_| poison_dmg = 3)
        .unwrap_or(0);
    let mut mana_regen = 0;
    let recharge = recharge
        .checked_sub(1)
        .inspect(|_| mana_regen = 101)
        .unwrap_or(0);

    (
        TurnEffects {
            shield_stat,
            poison_dmg,
            mana_regen,
        },
        shield,
        poison,
        recharge,
    )
}

fn part1_sol(enemy: Output, hard_mode: bool) -> Solved {
    let mut heap = Dijkstra::<Key, u16>::new();
    for spell in SPELLS {
        let key = Key {
            spell,
            shield: 0,
            poison: 0,
            recharge: 0,
            player_hp: PLAYER_HP - if hard_mode { 1 } else { 0 },
            mana: MANA_START,
            enemy_hp: enemy.health,
            // path: vec![spell],
        };
        heap.push(key, spell.cost());
    }

    while let Some(HeapState {
        key:
            Key {
                spell,
                shield,
                poison,
                recharge,
                player_hp,
                mana,
                enemy_hp,
            },
        cost,
    }) = heap.pop()
    {
        if player_hp < 1 {
            continue;
        }

        let (turn, mut shield, mut poison, mut recharge) = run_effects(shield, poison, recharge);
        let mut hp_regen = 0;
        match spell {
            Spell::MagicMissile => (),
            Spell::Drain => hp_regen = 2,
            Spell::Shield if shield < 1 => shield = 6,
            Spell::Shield => continue,
            Spell::Poison if poison < 1 => poison = 6,
            Spell::Poison => continue,
            Spell::Recharge if recharge < 1 => recharge = 5,
            Spell::Recharge => continue,
        }

        // player turn
        let player_hp = player_hp + hp_regen;
        let enemy_hp = enemy_hp - spell.attack_pow() - turn.poison_dmg;
        let mana = mana - spell.cost() as i16 + turn.mana_regen;
        if mana < 1 {
            continue;
        }

        // enemy turn
        let (turn, shield, poison, recharge) = run_effects(shield, poison, recharge);
        let player_hp = player_hp - attack_minus_armor(enemy.attack, turn.shield_stat);
        let enemy_hp = enemy_hp - turn.poison_dmg;
        if enemy_hp < 1 {
            return cost;
        }
        let mana = mana + turn.mana_regen;

        for spell in SPELLS {
            // let mut npath = key.path.clone();
            // npath.push(spell);
            let nkey = Key {
                spell,
                shield,
                poison,
                recharge,
                mana,
                player_hp: player_hp - if hard_mode { 1 } else { 0 },
                enemy_hp,
                // path: npath,
            };
            heap.push(nkey, cost + spell.cost());
        }
    }
    panic!("No solution");
}

// fn part2_sol(enemy: Output) -> Solved {
// }

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(parsed_input, false);
    let part2 = part1_sol(parsed_input, true);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
