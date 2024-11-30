use aoc_shared::{read_input_to_string, Dijkstra, HeapState};
use std::io;

type Output = Key;
type Solved = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn cost(&self) -> u32 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn room(&self) -> u8 {
        match self {
            Amphipod::Amber => 2,
            Amphipod::Bronze => 4,
            Amphipod::Copper => 6,
            Amphipod::Desert => 8,
        }
    }
}

impl TryFrom<char> for Amphipod {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Amber),
            'B' => Ok(Self::Bronze),
            'C' => Ok(Self::Copper),
            'D' => Ok(Self::Desert),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Room {
    Hallway(Option<Amphipod>),
    Room(u8, Option<Amphipod>, Option<Amphipod>),
}

impl Room {
    fn is_solved(&self) -> bool {
        match self {
            Room::Room(room_num, Some(am1), Some(am2)) => am1 == am2 && am1.room() == *room_num,
            _ => false,
        }
    }

    fn try_pop(&self) -> Option<(Self, Amphipod, u32)> {
        if self.is_solved() {
            return None;
        }
        match self {
            Room::Room(_, None, None) => None,
            Room::Room(room_num, Some(am1), Some(am2)) => {
                Some((Room::Room(*room_num, None, Some(*am2)), *am1, 1u32))
            }
            // we only pop am1.room() == room_num, when the bottom most amphipod is in the wrong room.
            Room::Room(room_num, None, Some(am2)) if am2.room() != *room_num => {
                Some((Room::Room(*room_num, None, None), *am2, 2u32))
            }
            _ => None,
        }
    }

    fn try_insert(&self, am1: Amphipod) -> Option<(Self, u32)> {
        if self.is_solved() {
            return None;
        }
        match self {
            Room::Room(room_num, None, Some(am2)) if am1 == *am2 => {
                Some((Room::Room(*room_num, Some(am1), Some(*am2)), 2))
            }
            Room::Room(room_num, None, None) => Some((Room::Room(*room_num, None, Some(am1)), 3)),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Key([Room; 11]);

impl Key {
    fn is_solved(&self) -> bool {
        self.0[2].is_solved()
            && self.0[4].is_solved()
            && self.0[6].is_solved()
            && self.0[8].is_solved()
    }
}

fn parse_input(input: &str) -> Output {
    let mut amphipods: Vec<Vec<Amphipod>> = input
        .split('\n')
        .filter_map(|line| {
            let line = line
                .chars()
                .filter_map(|chr| chr.try_into().ok())
                .collect::<Vec<_>>();
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(
        amphipods.len(),
        2,
        "There should be rooms with 2 slots, got {}",
        amphipods.len()
    );
    let bot = amphipods.pop().unwrap();
    let top = amphipods.pop().unwrap();
    let rooms = top
        .into_iter()
        .zip(bot)
        .enumerate()
        .map(|(room_num, (top, bot))| Room::Room(room_num as u8 * 2 + 2, Some(top), Some(bot)))
        .collect::<Vec<Room>>();
    assert_eq!(
        rooms.len(),
        4,
        "There should only be 8 Amphipods and 4 rooms with 2 spaces.",
    );
    let mut ret = [Room::Hallway(None); 11];
    ret[2] = rooms[0];
    ret[4] = rooms[1];
    ret[6] = rooms[2];
    ret[8] = rooms[3];
    Key(ret)
}

type Dij = Dijkstra<Key, u32>;

// can't move directly into room, we were forced out because bottom is not in the right room.
fn move_into_hallway(
    key: &Key,
    ki: usize,
    heap: &mut Dij,
    cost: u32,
    replace_with: Room,
    am: Amphipod,
    imoves: u32,
) {
    let lrange = 0..ki;
    let rrange = ki + 1..11;
    lrange
        .rev()
        .take_while(|&li| !matches!(key.0[li], Room::Hallway(Some(_))))
        .filter(|&li| matches!(key.0[li], Room::Hallway(None)))
        .for_each(|li| {
            let mut nkey = key.0;
            nkey[li] = Room::Hallway(Some(am));
            nkey[ki] = replace_with;
            let moves = li.abs_diff(ki) as u32;
            heap.push(Key(nkey), cost + ((moves + imoves) * am.cost()))
        });
    rrange
        .take_while(|&ri| !matches!(key.0[ri], Room::Hallway(Some(_))))
        .filter(|&ri| matches!(key.0[ri], Room::Hallway(None)))
        .for_each(|ri| {
            let mut nkey = key.0;
            nkey[ri] = Room::Hallway(Some(am));
            nkey[ki] = replace_with;
            let moves = ki.abs_diff(ri) as u32;
            heap.push(Key(nkey), cost + ((moves + imoves) * am.cost()))
        });
}

fn move_from_hallway_or_room(
    key: &Key,
    ki: usize,
    heap: &mut Dij,
    cost: u32,
    extra_moves: u32,
    am: Amphipod,
    replace_with: Room,
) -> bool {
    let target = am.room() as usize;
    // hallway can never be on a target.
    let range = if ki < target {
        ki + 1..target
    } else {
        target + 1..ki
    };
    let moves = range.len() as u32;
    // we're right on edge of the room.
    if moves == 0 {
        if let Some((room, imoves)) = key.0[target].try_insert(am) {
            let mut nkey = key.0;
            nkey[target] = room;
            nkey[ki] = replace_with;
            heap.push(
                Key(nkey),
                cost + ((moves + imoves + extra_moves) * am.cost()),
            );
            true
        } else {
            false
        }
    } else {
        // try to move to edge of room.
        for probe in range {
            if let Room::Hallway(Some(_)) = key.0[probe] {
                return false;
            }
        }
        if let Some((room, imoves)) = key.0[target].try_insert(am) {
            let mut nkey = key.0;
            nkey[target] = room;
            nkey[ki] = replace_with;
            heap.push(
                Key(nkey),
                cost + ((moves + imoves + extra_moves) * am.cost()),
            );
            true
        } else {
            false
        }
    }
}

fn part1_sol(input: Output) -> Solved {
    let mut heap = Dij::new();
    heap.push(input, 0);

    while let Some(HeapState { key, cost }) = heap.pop() {
        if key.is_solved() {
            return cost;
        }

        key.clone().0.into_iter().enumerate().for_each(|(i, r)| {
            match r {
                Room::Hallway(None) => (),
                Room::Hallway(Some(am)) => {
                    move_from_hallway_or_room(&key, i, &mut heap, cost, 0, am, Room::Hallway(None));
                }
                Room::Room(_, _, _) => {
                    if let Some((room, am, imoves)) = r.try_pop() {
                        let target = am.room() as usize;
                        // can't move directly into room, we were forced out because bottom is not in the right room.
                        if target == i {
                            move_into_hallway(&key, i, &mut heap, cost, room, am, imoves);
                        } else {
                            // try to move directly to room
                            if !move_from_hallway_or_room(
                                &key, i, &mut heap, cost, imoves, am, room,
                            ) {
                                move_into_hallway(&key, i, &mut heap, cost, room, am, imoves);
                            }
                        }
                    }
                }
            }
        });
    }
    panic!("NO SOLUTION?!");
}

// fn part2_sol(input: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    println!("{parsed_input:?}");
    let part1 = part1_sol(parsed_input);
    // let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    // print!("Part2: {part2}");
    println!();
    Ok(())
}
