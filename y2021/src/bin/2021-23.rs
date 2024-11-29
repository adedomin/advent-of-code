use aoc_shared::read_input_to_string;
use std::{
    collections::{BinaryHeap, HashMap},
    io,
};

type Output = [Room; 4];
type Solved = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
    Vacant,
}

impl Amphipod {
    fn cost(&self) -> u64 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
            Amphipod::Vacant => 0,
        }
    }

    fn room(&self) -> usize {
        match self {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
            Amphipod::Vacant => 4,
        }
    }

    fn is_valid(&self) -> bool {
        !matches!(self, Amphipod::Vacant)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Room {
    top: Amphipod,
    bot: Amphipod,
}

impl Room {
    fn new() -> Self {
        Room {
            top: Amphipod::Vacant,
            bot: Amphipod::Vacant,
        }
    }

    fn is_solved(&self, idx: usize) -> bool {
        match self {
            Room {
                top: Amphipod::Amber,
                bot: Amphipod::Amber,
            } if idx == 0 => true,
            Room {
                top: Amphipod::Bronze,
                bot: Amphipod::Bronze,
            } if idx == 1 => true,
            Room {
                top: Amphipod::Copper,
                bot: Amphipod::Copper,
            } if idx == 2 => true,
            Room {
                top: Amphipod::Desert,
                bot: Amphipod::Desert,
            } if idx == 3 => true,
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        matches!(
            self,
            Room {
                top: Amphipod::Vacant,
                bot: Amphipod::Vacant
            }
        )
    }

    fn insert(&self, amph: &Amphipod) -> Option<(Room, u64)> {
        match self {
            Self {
                top: Amphipod::Vacant,
                bot: Amphipod::Vacant,
            } => Some((
                Room {
                    top: Amphipod::Vacant,
                    bot: *amph,
                },
                3,
            )),
            Self {
                top: Amphipod::Vacant,
                bot,
            } => {
                if *bot == *amph {
                    Some((
                        Self {
                            top: *amph,
                            bot: *bot,
                        },
                        2,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn pop(self) -> (Room, Amphipod, u64) {
        match self {
            Room {
                top: Amphipod::Vacant,
                bot: Amphipod::Vacant,
            } => (Self::new(), Amphipod::Vacant, 0),
            Room {
                top: Amphipod::Vacant,
                bot: amph,
            } => (Self::new(), amph, 2),
            Room {
                top: amph1,
                bot: amph2,
            } => (
                Room {
                    top: Amphipod::Vacant,
                    bot: amph2,
                },
                amph1,
                1,
            ),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Key {
    hallway: [Amphipod; 7],
    rooms: [Room; 4],
}

type HS = aoc_shared::HeapState<Key, u64>;

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
    top.into_iter()
        .zip(bot)
        .map(|(top, bot)| Room { top, bot })
        .collect::<Vec<Room>>()
        .try_into()
        .expect("There should only be 8 Amphipods and 4 rooms with 2 spaces.")
}

const HALLWAY_POS: [u64; 7] = [0, 1, 3, 5, 7, 9, 10];
const ROOM_POS: [u64; 4] = [2, 4, 6, 8];
const COUNT_FROM: [(usize, usize); 4] = [(1, 2), (2, 3), (3, 4), (4, 5)];

fn part1_sol(input: Output) -> Solved {
    let mut distmap: HashMap<Key, u64> = HashMap::new();
    let mut heap: BinaryHeap<HS> = BinaryHeap::new();

    input
        .into_iter()
        .enumerate()
        .filter(|(idx, room)| !room.is_solved(*idx))
        .for_each(|(idx, room)| {
            let (room, popped, moves) = room.pop();
            HALLWAY_POS.iter().enumerate().for_each(|(hidx, &pos)| {
                let mut hallway = [Amphipod::Vacant; 7];
                hallway[hidx] = popped;
                let mut rooms = input;
                rooms[idx] = room;
                let key = Key { hallway, rooms };
                let cost = (pos.abs_diff(ROOM_POS[idx]) + moves) * popped.cost();
                distmap.insert(key.clone(), cost);
                heap.push(HS { key, cost });
            });
        });

    while let Some(HS {
        key: Key {
            mut hallway,
            mut rooms,
        },
        cost,
    }) = heap.pop()
    {
        // try and shove amphipods into their rooms.
        let mut moved = true;
        let mut totcost = cost;
        while moved {
            moved = false;
            (0..hallway.len()).for_each(|hi| {
                if !hallway[hi].is_valid() {
                    return;
                }
                let amph = hallway[hi];
                let room = amph.room();
                let curr_pos = HALLWAY_POS[hi];
                let (target, dir) = if curr_pos < ROOM_POS[room] {
                    (ROOM_POS[room] - 1, 1)
                } else {
                    (ROOM_POS[room] + 1, -1)
                };

                let mut i = hi as isize;
                if let Some((nroom, icost)) = loop {
                    if HALLWAY_POS[i as usize] == target {
                        println!(
                            "trying to move {amph:?} into room[{room}] {:?}",
                            rooms[room]
                        );
                        break rooms[room].insert(&amph);
                    } else if hallway[i as usize] != Amphipod::Vacant {
                        break None;
                    }
                    i += dir;
                } {
                    println!("moved into room[{room}] {nroom:?}");
                    println!("rooms: {rooms:?}");
                    totcost += (curr_pos.abs_diff(ROOM_POS[room]) + icost) * amph.cost();
                    rooms[room] = nroom;
                    hallway[hi] = Amphipod::Vacant;
                    moved = true;
                }
            });
        }
        // did we solve it?
        if rooms
            .iter()
            .enumerate()
            .all(|(ridx, rooms)| rooms.is_solved(ridx))
        {
            println!("{rooms:?}, {hallway:?}");
            return totcost;
        }
        // now try and move out new amphs that need to move (one at a time)
        rooms
            .iter()
            .enumerate()
            .filter(|(_, room)| !room.is_empty()) // room is empty
            .filter(|(ridx, room)| !room.is_solved(*ridx)) // room is solved for
            .filter(|(ridx, room)| {
                let (_, amph, _) = room.pop();
                amph.room() != *ridx // this amph belongs here, and is at the bottom of the room.
            })
            .flat_map(|(ridx, room)| {
                let mut keys = Vec::with_capacity(hallway.len());
                let (room, popped, moves) = room.pop();
                let (left, right) = COUNT_FROM[ridx];

                // straight to room
                let dir_room = popped.room();
                if dir_room != ridx {
                    let curr_pos = ROOM_POS[ridx];
                    let (nidx, target, dir) = if curr_pos < ROOM_POS[dir_room] {
                        (right, ROOM_POS[dir_room] - 1, 1isize)
                    } else {
                        (left, ROOM_POS[dir_room] + 1, -1isize)
                    };
                    let mut i: isize = nidx as isize;
                    if let Some((nroom, icost)) = loop {
                        println!("{popped:?}, {ridx}: {i} {left}, {right} {curr_pos}");
                        if HALLWAY_POS[i as usize] == target {
                            break rooms[dir_room].insert(&popped);
                        } else if hallway[i as usize] != Amphipod::Vacant {
                            break None;
                        }
                        i += dir;
                    } {
                        let mut rooms = rooms;
                        rooms[ridx] = room;
                        rooms[dir_room] = nroom;
                        let key = Key { hallway, rooms };
                        let cost = (curr_pos.abs_diff(ROOM_POS[dir_room]) + icost) * popped.cost();
                        keys.push((key, cost + totcost));
                        return keys; // direct to room will always be faster, no?
                    }
                }

                // left
                let mut li = left;
                loop {
                    if hallway[li] != Amphipod::Vacant {
                        break;
                    } else {
                        let mut hallway = hallway;
                        hallway[li] = popped;
                        let mut rooms = rooms;
                        rooms[ridx] = room;
                        let key = Key { hallway, rooms };
                        let cost =
                            (HALLWAY_POS[li].abs_diff(ROOM_POS[ridx] + moves)) * popped.cost();
                        keys.push((key, cost + totcost));
                    }
                    if li == 0 {
                        break;
                    }
                    li -= 1;
                }
                // right
                let mut ri = right;
                loop {
                    if hallway[ri] != Amphipod::Vacant {
                        break;
                    } else {
                        let mut hallway = hallway;
                        hallway[ri] = popped;
                        let mut rooms = rooms;
                        rooms[ridx] = room;
                        let key = Key { hallway, rooms };
                        let cost =
                            (HALLWAY_POS[li].abs_diff(ROOM_POS[ridx] + moves)) * popped.cost();
                        keys.push((key, cost + totcost));
                    }
                    ri += 1;
                    if ri == hallway.len() {
                        break;
                    }
                }
                keys
            })
            .for_each(|(key, ncost)| {
                let dent = distmap.entry(key.clone()).or_insert(u64::MAX);
                if ncost < *dent {
                    *dent = ncost;
                    heap.push(HS { key, cost: ncost });
                }
            });
        // if no moves can be made, prune this result.
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
