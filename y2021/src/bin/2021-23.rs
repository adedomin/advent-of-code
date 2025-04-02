use aoc_shared::{read_input_to_string, Dijkstra, HeapState};
use itertools::{Either, Itertools};
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
    Part1(u8, Option<Amphipod>, Option<Amphipod>),
    Part2(
        u8,
        Option<Amphipod>,
        Option<Amphipod>,
        Option<Amphipod>,
        Option<Amphipod>,
    ),
}

const PART2_ROOM_EXTENSIONS: [Amphipod; 8] = [
    // room1
    Amphipod::Desert,
    Amphipod::Desert,
    // room2
    Amphipod::Copper,
    Amphipod::Bronze,
    // room3
    Amphipod::Bronze,
    Amphipod::Amber,
    // room4
    Amphipod::Amber,
    Amphipod::Copper,
];

impl Room {
    fn convert_to_part2(self) -> Self {
        match self {
            Room::Hallway(x) => Room::Hallway(x),
            Room::Part1(x, y, z) => {
                let one = (x - 2) as usize;
                let two = (x - 1) as usize;
                Room::Part2(
                    x,
                    y,
                    Some(PART2_ROOM_EXTENSIONS[one]),
                    Some(PART2_ROOM_EXTENSIONS[two]),
                    z,
                )
            }
            Room::Part2(_, _, _, _, _) => self,
        }
    }

    fn is_solved(&self) -> bool {
        match self {
            Room::Part1(room_num, Some(am1), Some(am2)) => am1 == am2 && am1.room() == *room_num,
            Room::Part2(room_num, Some(am1), Some(am2), Some(am3), Some(am4)) => {
                [am1, am2, am3, am4].into_iter().all_equal() && am1.room() == *room_num
            }
            _ => false,
        }
    }

    fn try_pop(&self) -> Option<(Self, Amphipod, u32)> {
        if self.is_solved() {
            return None;
        }
        match self {
            Room::Part1(_, None, None) => None,
            Room::Part1(room_num, Some(am1), Some(am2)) => {
                Some((Room::Part1(*room_num, None, Some(*am2)), *am1, 1u32))
            }
            // we only pop am1.room() == room_num, when the bottom most amphipod is in the wrong room.
            Room::Part1(room_num, None, Some(am2)) if am2.room() != *room_num => {
                Some((Room::Part1(*room_num, None, None), *am2, 2u32))
            }
            Room::Part2(room_num, Some(am1), Some(am2), Some(am3), Some(am4)) => Some((
                Room::Part2(*room_num, None, Some(*am2), Some(*am3), Some(*am4)),
                *am1,
                1u32,
            )),
            Room::Part2(room_num, None, Some(am2), Some(am3), Some(am4))
                if [am2, am3, am4].into_iter().any(|am| am.room() != *room_num) =>
            {
                Some((
                    Room::Part2(*room_num, None, None, Some(*am3), Some(*am4)),
                    *am2,
                    2u32,
                ))
            }
            Room::Part2(room_num, None, None, Some(am3), Some(am4))
                if am3.room() != *room_num || am4.room() != *room_num =>
            {
                Some((
                    Room::Part2(*room_num, None, None, None, Some(*am4)),
                    *am3,
                    3u32,
                ))
            }
            Room::Part2(room_num, None, None, None, Some(am4)) if am4.room() != *room_num => {
                Some((Room::Part2(*room_num, None, None, None, None), *am4, 4u32))
            }
            _ => None,
        }
    }

    fn try_insert(&self, am1: Amphipod) -> Option<(Self, u32)> {
        if self.is_solved() {
            return None;
        }
        match self {
            Room::Part1(room_num, None, Some(am2)) if am1 == *am2 => {
                Some((Room::Part1(*room_num, Some(am1), Some(*am2)), 2))
            }
            Room::Part1(room_num, None, None) => Some((Room::Part1(*room_num, None, Some(am1)), 3)),
            Room::Part2(room_num, None, Some(am2), Some(am3), Some(am4))
                if [am2, am3, am4].into_iter().all(|&am| am == am1) =>
            {
                Some((
                    Room::Part2(*room_num, Some(am1), Some(*am2), Some(*am3), Some(*am4)),
                    2,
                ))
            }
            Room::Part2(room_num, None, None, Some(am3), Some(am4))
                if [am3, am4].into_iter().all(|&am| am == am1) =>
            {
                Some((
                    Room::Part2(*room_num, None, Some(am1), Some(*am3), Some(*am4)),
                    3,
                ))
            }
            Room::Part2(room_num, None, None, None, Some(am4)) if *am4 == am1 => {
                Some((Room::Part2(*room_num, None, None, Some(am1), Some(*am4)), 4))
            }
            Room::Part2(room_num, None, None, None, None) => {
                Some((Room::Part2(*room_num, None, None, None, Some(am1)), 5))
            }
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
        .map(|(room_num, (top, bot))| Room::Part1(room_num as u8 * 2 + 2, Some(top), Some(bot)))
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
    [Either::Left((0..ki).rev()), Either::Right((ki + 1)..11)]
        .into_iter()
        .for_each(|e| {
            e.take_while(|&ri| !matches!(key.0[ri], Room::Hallway(Some(_))))
                .filter(|&ri| matches!(key.0[ri], Room::Hallway(None)))
                .for_each(|ri| {
                    let mut nkey = key.0;
                    nkey[ri] = Room::Hallway(Some(am));
                    nkey[ki] = replace_with;
                    let moves = ki.abs_diff(ri) as u32;
                    heap.push(Key(nkey), cost + ((moves + imoves) * am.cost()))
                });
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

fn part1_sol(input: Output) -> Solved {
    let mut heap = Dij::new();
    heap.push(input, 0);

    while let Some(HeapState { key, cost }) = heap.pop() {
        if key.is_solved() {
            return cost;
        }

        key.0.as_ref().iter().enumerate().for_each(|(i, r)| {
            match r {
                Room::Hallway(None) => (),
                Room::Hallway(Some(am)) => {
                    move_from_hallway_or_room(
                        &key,
                        i,
                        &mut heap,
                        cost,
                        0,
                        *am,
                        Room::Hallway(None),
                    );
                }
                _ => {
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

fn part2_sol(input: Output) -> Solved {
    let Key(inner) = input;
    part1_sol(Key(inner
        .into_iter()
        .map(|x| x.convert_to_part2())
        .collect::<Vec<Room>>()
        .try_into()
        .unwrap()))
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(parsed_input.clone());
    print!("Part1: {part1}, ");
    let part2 = part2_sol(parsed_input);
    print!("Part2: {part2}");
    println!();
    Ok(())
}
