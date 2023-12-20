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

/* copy paste from day 8 */
const PRIMES_TO_1009: [u32; 169] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009,
];

fn factorize(n: u32) -> Vec<(usize, u32)> {
    let mut ret = [0u32; PRIMES_TO_1009.len()];
    let mut i = 0;
    let mut n = n;
    while n > 1 {
        if n % PRIMES_TO_1009[i] == 0 {
            n /= PRIMES_TO_1009[i];
            ret[i] += 1;
        } else {
            i += 1;
        }
    }

    ret.iter()
        .copied()
        .enumerate()
        .filter(|(_, cnt)| *cnt > 0)
        .collect::<Vec<_>>()
}

fn least_common_mult(factors: Vec<(usize, u32)>) -> u64 {
    let mut resi = [0u32; PRIMES_TO_1009.len()];

    for (prime, cnt) in factors {
        let curr = resi[prime];
        resi[prime] = std::cmp::max(cnt, curr);
    }

    resi.iter()
        .enumerate()
        .filter(|(_, &cnt)| cnt != 0)
        .map(|(p, pow)| {
            let prime = PRIMES_TO_1009[p];
            prime.pow(*pow) as u64
        })
        .product()
}
/* end paste from day 8 */

type Label = [u8; 2];

#[derive(Eq, PartialEq, Clone, Debug)]
enum CType {
    Broadcast(bool),
    FlipFlop(bool),
    Conjunction(Vec<(Label, Energy)>),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
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

#[derive(Eq, PartialEq, Clone, Debug)]
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

fn cycle(input: &mut DesertMachine, checked: &[Label]) -> (u64, u64, Vec<Energy>) {
    let mut low = 0u64;
    let mut high = 0u64;
    let mut queue = VecDeque::from([(Energy::Low, BROADCAST, BUTTON)]);
    let mut checked_v = vec![Energy::Low; checked.len()];
    while let Some((energy, circuit_id, activated_by)) = queue.pop_front() {
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
                    checked_v[pos] = next_e;
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

fn print_label(i: &Label) {
    println!("{}", std::str::from_utf8(i).unwrap());
}

fn solve2(mut input: DesertMachine, inverse: InverseMap) -> u64 {
    let mut i = 0u32;
    // we need to go one level above rx's dep because of how conjugators work...
    // we assume looking one level gets the cycle based on a visualization of
    // the input's cycles and connections..... is not generalized.
    let rx_watches = inverse
        .get(&RX)
        .expect("rx should be mapped")
        .iter()
        .flat_map(|v| inverse.get(v).unwrap())
        .flat_map(|v| inverse.get(v).unwrap())
        .flat_map(|v| inverse.get(v).unwrap())
        .copied()
        .collect::<Vec<Label>>();
    for w in rx_watches.iter() {
        print_label(w);
    }
    let mut rx_s = vec![0; rx_watches.len()];
    while !rx_s.iter().all(|v| *v != 0) {
        i += 1;
        cycle(&mut input, &rx_watches)
            .2
            .iter()
            .enumerate()
            .for_each(|(pos, e)| {
                if matches!(e, Energy::High) {
                    rx_s[pos] = i;
                }
            });
    }
    least_common_mult(
        rx_s.into_iter()
            .flat_map(factorize)
            .collect::<Vec<(usize, u32)>>(),
    )
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
