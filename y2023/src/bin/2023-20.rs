use aoc_shared::read_input;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::is_alphabetic,
    combinator::{map_res, opt},
    multi::separated_list1,
    IResult,
};
use std::{
    collections::{HashMap, VecDeque},
    convert::Infallible,
    io,
};

type Label = [u8; 2];

#[derive(Eq, PartialEq, Clone)]
enum CType {
    Broadcast(bool),
    FlipFlop(bool),
    Conjunction(Vec<(Label, Energy)>),
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Energy {
    High,
    Low,
}

impl From<u8> for CType {
    fn from(value: u8) -> Self {
        match value {
            b'%' => Self::FlipFlop(false),
            b'&' => Self::Conjunction(vec![]),
            _ => Self::Broadcast(false),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Circuit(pub CType, pub Label);

impl Circuit {
    fn interact(&mut self, activator: Label, e: Energy) -> Option<Energy> {
        match &mut self.0 {
            CType::Broadcast(s) => {
                if *s {
                    Some(Energy::High)
                } else {
                    Some(Energy::Low)
                }
            }
            CType::FlipFlop(_) if matches!(e, Energy::High) => None,
            CType::FlipFlop(s) => {
                if *s {
                    *s = false;
                    Some(Energy::Low)
                } else {
                    *s = true;
                    Some(Energy::High)
                }
            }
            CType::Conjunction(mem) => {
                if let Some(m) = mem.iter_mut().find(|(c, _)| *c == activator) {
                    m.1 = e;
                    if mem
                        .iter()
                        .all(|(_, activated)| matches!(activated, Energy::High))
                    {
                        Some(Energy::Low)
                    } else {
                        Some(Energy::High)
                    }
                } else {
                    panic!("We shouldn't be here.") // how?
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct DesertMachine {
    pub state: HashMap<Label, Circuit>,
    pub edges: HashMap<Label, Vec<Label>>,
}

type InverseMap = HashMap<Label, Vec<Label>>;

fn label_to_real(i: &[u8]) -> Label {
    [i[0], i[1]]
}

fn parse_input(input: &[u8]) -> (DesertMachine, InverseMap) {
    fn parse_circuits(i: &[u8]) -> IResult<&[u8], (Circuit, Vec<Label>)> {
        let (i, ty) = opt(alt((tag(b"%"), tag(b"&"))))(i)?;
        let ty: CType = if let Some(ty) = ty {
            ty[0].into()
        } else {
            CType::Broadcast(false)
        };

        let (i, label) = match ty {
            CType::Broadcast(_) => {
                let (i, _) = tag(b"broadcaster")(i)?;
                (i, [b'Z', b'Z'])
            }
            CType::FlipFlop(_) | CType::Conjunction(_) => {
                let (i, label) = take_while1(is_alphabetic)(i)?;
                (i, label_to_real(label))
            }
        };
        // remove noise.
        let (i, _) = take_till(is_alphabetic)(i)?;

        let (i, circuits) = separated_list1(
            tag(b", "),
            map_res(
                take_while1(is_alphabetic),
                |n| -> Result<Label, Infallible> { Ok(label_to_real(n)) },
            ),
        )(i)?;
        Ok((i, (Circuit(ty, label), circuits)))
    }
    let (i, circuits) = separated_list1(tag(b"\n"), parse_circuits)(input).unwrap();

    if i != b"\n" {
        panic!("unparsed residue: {:?}", std::str::from_utf8(i).unwrap());
    }

    let mut state = circuits.iter().fold(HashMap::new(), |mut acc, (c, _)| {
        acc.insert(c.1, Circuit(c.0.clone(), c.1));
        acc
    });

    let edges = circuits
        .into_iter()
        .map(|(c, cs)| (c.1, cs))
        .collect::<HashMap<Label, Vec<Label>>>();
    // we have to populate conjunction initial states.
    edges.iter().for_each(|(k, vs)| {
        vs.iter().for_each(|v| {
            if let Some(s) = state.get_mut(v) {
                match &mut s.0 {
                    CType::Conjunction(v) => v.push((*k, Energy::Low)),
                    _ => (),
                }
            }
        })
    });

    let mut inverse_map: InverseMap = HashMap::new();
    edges.iter().for_each(|(cid, maps)| {
        maps.iter().for_each(|map| {
            if let Some(m) = inverse_map.get_mut(map) {
                m.push(*cid);
            } else {
                inverse_map.insert(*map, vec![*cid]);
            }
        })
    });
    (DesertMachine { state, edges }, inverse_map)
}

const BROADCAST: Label = [b'Z', b'Z'];
const BUTTON: Label = [b'Z', b'Y'];
const RX: Label = [b'r', b'x'];

fn cycle(input: &mut DesertMachine, checked: &[Label]) -> (u64, u64, Vec<bool>) {
    let mut low = 0u64;
    let mut high = 0u64;
    let mut queue = VecDeque::from([(Energy::Low, BROADCAST, BUTTON)]);
    let mut checked_v = vec![false; checked.len()];
    while let Some((energy, circuit_id, activated_by)) = queue.pop_back() {
        match energy {
            Energy::High => high += 1,
            Energy::Low => low += 1,
        }
        // get activated circuit
        if let Some(circuit) = input.state.get_mut(&circuit_id) {
            // get next energy level
            if let Some(next_e) = circuit.interact(activated_by, energy) {
                // get next circuits and apply next_energy to them
                if let Some(circuits) = input.edges.get(&circuit_id) {
                    circuits.iter().for_each(|cid| {
                        queue.push_front((next_e, *cid, circuit_id));
                    })
                }
                // look at watched circuits and detect the energies they emitted.
                if let Some(pos) = checked.iter().position(|c| *c == circuit_id) {
                    if matches!(next_e, Energy::High) {
                        checked_v[pos] = true;
                    }
                }
            }
        }
    }
    (low, high, checked_v)
}

fn solve1(mut input: DesertMachine) -> u64 {
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (l, h, _) = cycle(&mut input, &[]);
        low += l;
        high += h;
    }
    low * high
}

fn solve2(mut input: DesertMachine, inverse: InverseMap) -> u64 {
    let mut i = 0u64;
    // we need to go one level above rx's dep because of how conjugators work...
    // we assume looking one level gets the cycle based on a visualization of
    // the input's cycles and connections..... is not generalized.
    let rx_watches = inverse
        .get(&RX)
        .expect("rx should be mapped")
        .iter()
        .flat_map(|v| inverse.get(v).unwrap())
        .copied()
        .collect::<Vec<Label>>();
    let mut rx_s = vec![0; rx_watches.len()];
    while !rx_s.iter().all(|v| *v != 0) {
        i += 1;
        cycle(&mut input, &rx_watches)
            .2
            .iter()
            .enumerate()
            .for_each(|(pos, e)| {
                if *e && rx_s[pos] == 0 {
                    rx_s[pos] = i;
                }
            });
    }
    rx_s.into_iter().product()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (machine, inverse) = parse_input(&input);
    let part1 = solve1(machine.clone());
    print!("Part1: {part1}, ");
    let part2 = solve2(machine, inverse);
    print!("Part2: {part2}");
    println!();
    Ok(())
}
