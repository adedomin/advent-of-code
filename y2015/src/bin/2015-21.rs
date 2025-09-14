use aoc_shared::{read_input_to_string, try_atoi};
use std::io;

type Output = Character;
type Solved = i16;

const PLAYER_HP: i16 = 100;

#[derive(Clone, Copy)]
struct Character {
    health: i16,
    attack: i16,
    armor: i16,
}

impl From<&Gears> for Character {
    fn from((_, weapon, armor, ring1, ring2): &Gears) -> Self {
        let attack = weapon.1 + ring1.1 + ring2.1;
        let armor = armor.2 + ring1.2 + ring2.2;
        Character {
            health: PLAYER_HP,
            attack,
            armor,
        }
    }
}

fn beats_enemy(player: Character, enemy: Character) -> bool {
    let damage_to_player = if enemy.attack - player.armor > 0 {
        enemy.attack - player.armor
    } else {
        1
    };
    let damage_to_enemy = if player.attack - enemy.armor > 0 {
        player.attack - enemy.armor
    } else {
        1
    };

    let rounds_needed_player = (player.health / damage_to_player)
        + (if player.health % damage_to_player > 0 {
            1
        } else {
            0
        });
    let rounds_needed_enemy = (enemy.health / damage_to_enemy)
        + (if enemy.health % damage_to_enemy > 0 {
            1
        } else {
            0
        });

    rounds_needed_player >= rounds_needed_enemy
}

#[derive(Clone, Copy)]
struct Gear(
    i16, /* cost */
    i16, /* damage */
    i16, /* defense */
);

// weapons
const DAGGER: Gear = Gear(8, 4, 0);
const SHORTSWORD: Gear = Gear(10, 5, 0);
const WARHAMMER: Gear = Gear(25, 6, 0);
const LONGSWORD: Gear = Gear(40, 7, 0);
const GREATAXE: Gear = Gear(74, 8, 0);
const WEAPONS: [Gear; 5] = [DAGGER, SHORTSWORD, WARHAMMER, LONGSWORD, GREATAXE];

// ARMOR
const NO_ARMOR: Gear = Gear(0, 0, 0);
const LEATHER: Gear = Gear(13, 0, 1);
const CHAINMAIL: Gear = Gear(31, 0, 2);
const SPLINTMAIL: Gear = Gear(53, 0, 3);
const BANDEDMAIL: Gear = Gear(75, 0, 4);
const PLATEMAIL: Gear = Gear(102, 0, 5);
const ARMOR: [Gear; 6] = [
    NO_ARMOR, LEATHER, CHAINMAIL, SPLINTMAIL, BANDEDMAIL, PLATEMAIL,
];

// RINGS
const NO_RING: Gear = Gear(0, 0, 0);
const DAMAGE_PLUS1: Gear = Gear(25, 1, 0);
const DAMAGE_PLUS2: Gear = Gear(50, 2, 0);
const DAMAGE_PLUS3: Gear = Gear(100, 3, 0);
const DEFENSE_PLUS1: Gear = Gear(20, 0, 1);
const DEFENSE_PLUS2: Gear = Gear(40, 0, 2);
const DEFENSE_PLUS3: Gear = Gear(80, 0, 3);
const RINGS: [Gear; 7] = [
    NO_RING,
    DAMAGE_PLUS1,
    DAMAGE_PLUS2,
    DAMAGE_PLUS3,
    DEFENSE_PLUS1,
    DEFENSE_PLUS2,
    DEFENSE_PLUS3,
];

type Gears = (i16, Gear, Gear, Gear, Gear);

static GEAR_COMBINATIONS: [Gears; WEAPONS.len() * ARMOR.len() * RINGS.len() * RINGS.len()] = {
    let mut arr = [(0, NO_RING, NO_RING, NO_RING, NO_RING);
        WEAPONS.len() * ARMOR.len() * RINGS.len() * RINGS.len()];
    let mut wep = 0usize;
    let mut arm = 0usize;
    let mut r1 = 0usize;
    let mut r2 = 0usize;
    let mut i = 0;
    // if it doesn't terminate or goes out of bounds, the algo is wrong anyway.
    loop {
        let cost = WEAPONS[wep].0 + ARMOR[arm].0 + RINGS[r1].0 + RINGS[r2].0;
        arr[i] = (cost, WEAPONS[wep], ARMOR[arm], RINGS[r1], RINGS[r2]);
        wep += 1;
        if wep == WEAPONS.len() {
            wep = 0;
            arm += 1;
            if arm == ARMOR.len() {
                arm = 0;
                r1 += 1;
                if r1 == RINGS.len() {
                    r1 = 0;
                    r2 += 1;
                    if r2 == RINGS.len() {
                        break arr;
                    }
                }
            }
        }
        i += 1;
    }
};

fn parse_input(input: &str) -> Output {
    let vals = input
        .split_ascii_whitespace()
        .filter_map(|str| try_atoi::<i16, 10>(str.as_bytes()))
        .collect::<Vec<i16>>();
    assert_eq!(vals.len(), 3);
    Character {
        health: vals[0],
        attack: vals[1],
        armor: vals[2],
    }
}

fn part1_sol(enemy: Output) -> Solved {
    GEAR_COMBINATIONS
        .iter()
        .filter_map(|gears| {
            let player: Character = gears.into();
            if beats_enemy(player, enemy) {
                Some(gears.0)
            } else {
                None
            }
        })
        .min()
        .expect("At least one gear pair to win")
}

fn part2_sol(enemy: Output) -> Solved {
    GEAR_COMBINATIONS
        .iter()
        .filter_map(|gears| {
            let player: Character = gears.into();
            if !beats_enemy(player, enemy) {
                Some(gears.0)
            } else {
                None
            }
        })
        .max()
        .expect("At least one gear pair to win")
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(parsed_input);
    let part2 = part2_sol(parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
