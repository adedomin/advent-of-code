use std::io;
use aoc_shared::{fold_decimal, read_input, AoCTokenizer, Token};

#[derive(PartialEq, Eq, Debug)]
enum Packet {
    Data(u64),
    List(Vec<Packet>),
}

#[derive(PartialEq, Eq, Debug)]
enum PartialPacket {
    Start,
    P(Packet),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Data(lhs), Packet::Data(rhs)) => lhs.cmp(rhs),
            (Packet::Data(lhs), other @ Packet::List(_)) => {
                Packet::List(vec![Packet::Data(*lhs)]).cmp(other)
            }
            (lhs @ Packet::List(_), Packet::Data(rhs)) => {
                lhs.cmp(&Packet::List(vec![Packet::Data(*rhs)]))
            }
            (Packet::List(lhs), Packet::List(rhs)) => {
                let tmp = lhs
                    .iter()
                    .zip(rhs)
                    .fold(std::cmp::Ordering::Equal, |acc, (l, r)| {
                        if acc.is_eq() {
                            l.cmp(r)
                        } else {
                            acc
                        }
                    });
                if tmp.is_eq() && lhs.len() < rhs.len() {
                    std::cmp::Ordering::Less
                } else if tmp.is_eq() && rhs.len() < lhs.len() {
                    std::cmp::Ordering::Greater
                } else {
                    tmp
                }
            }
        }
    }
}

// impl Packet {
//     pub fn values(&self) -> Vec<u64> {
//         match self {
//             Packet::Data(data) => vec![*data],
//             Packet::List(list) => list.iter().fold(vec![], |mut acc, packet| {
//                 acc.extend(packet.values().iter());
//                 acc
//             }),
//         }
//     }
// }

type Output = Vec<(Packet, Packet)>;

fn parse(input: Vec<u8>) -> Output {
    let mut ret = vec![];
    let mut stack = vec![];

    for token in AoCTokenizer::new(&input) {
        match token {
            Token::Something(num) => {
                stack.push(PartialPacket::P(Packet::Data(
                    num.iter().fold(0u64, fold_decimal),
                )));
            }
            Token::Delimiter(sep) if sep == b'[' => {
                stack.push(PartialPacket::Start);
            }
            Token::Delimiter(sep) if sep == b']' => {
                let mut new_partial_packet = Vec::new();
                loop {
                    let part = stack
                        .pop()
                        .expect("Unmatched PartialPacket::Start in packet data.");

                    match part {
                        PartialPacket::Start => break,
                        PartialPacket::P(packet) => new_partial_packet.push(packet),
                    }
                }
                new_partial_packet.reverse();
                stack.push(PartialPacket::P(Packet::List(new_partial_packet)));
            }
            Token::DoubleNewline | Token::End if !stack.is_empty() => {
                if let PartialPacket::P(r) = stack.pop().expect("Not enough packets in the stack!")
                {
                    if let PartialPacket::P(l) =
                        stack.pop().expect("Not enough packets in the stack!")
                    {
                        ret.push((l, r));
                    } else {
                        panic!("Got a List start token instead of data!");
                    }
                } else {
                    panic!("Got a List start token instead of data!");
                }
            }
            _ => (),
        }
    }

    ret
}

fn solve_p1(packets: &[(Packet, Packet)]) -> usize {
    packets
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| l.cmp(r).is_le())
        .fold(0, |acc, (i, _)| acc + (i + 1))
}

fn solve_p2(packets: &[(Packet, Packet)]) -> usize {
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Data(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Data(6)])]);
    let mut sorted_packets = packets.iter().fold(Vec::new(), |mut acc, (l, r)| {
        acc.push(l);
        acc.push(r);
        acc
    });
    sorted_packets.push(&div1);
    sorted_packets.push(&div2);
    sorted_packets.sort_unstable();

    (sorted_packets
        .iter()
        .position(|packet| {
            if let Packet::List(v) = packet {
                if let [Packet::List(v)] = &v[..] {
                    matches!(&v[..], [Packet::Data(2)])
                } else {
                    false
                }
            } else {
                false
            }
        })
        .expect("could not find packet: [[2]]")
        + 1)
        * (sorted_packets
            .iter()
            .position(|packet| {
                if let Packet::List(v) = packet {
                    if let [Packet::List(v)] = &v[..] {
                        matches!(&v[..], [Packet::Data(6)])
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .expect("could not find packet [[6]]")
            + 1)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let packets = parse(input);
    let part1 = solve_p1(&packets);
    let part2 = solve_p2(&packets);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
