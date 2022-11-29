#![feature(array_chunks)]
use std::io;

use y2021::read_input;
use bitvec::{field::BitField, order::Msb0, view::BitView};

fn hexdigit(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'A'..=b'F' => hex - b'A' + 10,
        b'a'..=b'f' => hex - b'a' + 10,
        _ => 0,
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Sum,
    Mul,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

impl From<u8> for Operator {
    fn from(val: u8) -> Self {
        match val {
            0 => Operator::Sum,
            1 => Operator::Mul,
            2 => Operator::Min,
            3 => Operator::Max,
            4 => panic!("Invalid Operator type 4."),
            5 => Operator::Gt,
            6 => Operator::Lt,
            7 => Operator::Eq,
            _ => panic!("Invalid Operator type unknown."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Typ {
    Value,
    Operator(Operator),
}

#[derive(Debug)]
struct Header {
    ver: u8,
    typ: Typ,
}

#[derive(Debug)]
enum Data {
    Value(u64),
    Children(usize),
}

#[derive(Debug)]
struct Packet {
    header: Header,
    data: Data,
}

#[derive(Debug)]
enum Counter {
    Count((usize, usize, usize)),
    Bytes((usize, usize, usize)),
    Data((bool, u64)),
}

fn parse(input: Vec<u8>) -> Vec<Packet> {
    let bindata = input.array_chunks::<2>().into_iter().fold(
        Vec::<u8>::with_capacity(input.len() / 2),
        |mut acc, &[msb, lsb]| {
            acc.push(hexdigit(msb) << 4 | hexdigit(lsb));
            acc
        },
    );

    let mut packets = Vec::<Packet>::new();
    let mut stack = Vec::<(Header, Counter)>::new();

    let bitdata = bindata.view_bits::<Msb0>();
    let mut i = 0;

    while i + 7 < bitdata.len() {
        let ver = bitdata[i..3 + i].load_be();
        i += 3;
        let typ = bitdata[i..3 + i].load_be::<u8>();
        i += 3;
        let (typ, cnt) = if typ == 4 {
            i += 5;
            (
                Typ::Value,
                Counter::Data((bitdata[i - 5], bitdata[i - 4..i].load_be())),
            )
        } else if bitdata[i] {
            i += 12;
            (
                Typ::Operator(typ.into()),
                Counter::Count((packets.len(), 0, bitdata[i - 11..i].load_be())),
            )
        } else {
            i += 16;
            (
                Typ::Operator(typ.into()),
                Counter::Bytes((packets.len(), i, bitdata[i - 15..i].load_be())),
            )
        };

        let head = Header { ver, typ };
        stack.push((head, cnt));

        while let Some((Header { ver, typ }, counter)) = stack.pop() {
            match typ {
                Typ::Value => match counter {
                    Counter::Data((cont, data)) if cont => {
                        let new_cont = bitdata[i];
                        i += 1;
                        let new_data = bitdata[i..i + 4].load_be::<u64>();
                        i += 4;
                        let new_counter = Counter::Data((new_cont, data << 4 | new_data));
                        stack.push((Header { ver, typ }, new_counter));
                    }
                    Counter::Data((_, data)) => packets.push(Packet {
                        header: Header { ver, typ },
                        data: Data::Value(data),
                    }),
                    _ => unreachable!(),
                },
                Typ::Operator(_) => match counter {
                    Counter::Count((start, cur, count)) => {
                        if cur == 0 {
                            packets.push(Packet {
                                header: Header { ver, typ },
                                data: Data::Children(start),
                            });
                        }
                        if cur < count {
                            stack.push((
                                Header { ver, typ },
                                Counter::Count((start, cur + 1, count)),
                            ));
                            break;
                        } else {
                            packets[start].data = Data::Children(packets.len());
                        }
                    }
                    Counter::Bytes((start, last_i, len)) => {
                        if start == packets.len() {
                            packets.push(Packet {
                                header: Header { ver, typ },
                                data: Data::Children(start),
                            });
                        }

                        if i - last_i < len {
                            stack.push((Header { ver, typ }, Counter::Bytes((start, last_i, len))));
                            break;
                        } else {
                            packets[start].data = Data::Children(packets.len());
                        }
                    }
                    _ => unreachable!(),
                },
            }
        }
        if i % 8 != 0 && stack.is_empty() {
            i += 8 - (i % 8)
        }
    }

    packets
}

fn part1_vsum(packets: &[Packet]) -> u64 {
    packets
        .iter()
        .map(|packet| packet.header.ver)
        .fold(0u64, |acc, byte| acc + (byte as u64))
}

fn part2_eval(packets: &[Packet]) -> u64 {
    let mut sum = 0;

    let mut i = 0usize;
    let mut packet_stack = vec![(i, vec![])];
    while i < packets.len() {
        i += 1;
        while let Some((packet_idx, values)) = packet_stack.pop() {
            let packet = &packets[packet_idx];
            match packet.data {
                Data::Children(size) => {
                    if i < size {
                        packet_stack.push((packet_idx, values));
                        packet_stack.push((i, vec![]));
                        break;
                    } else {
                        let res: u64 = match packet.header.typ {
                            Typ::Operator(op) => match op {
                                Operator::Sum => values.iter().sum(),
                                Operator::Mul => values.iter().product(),
                                Operator::Min => *values
                                    .iter()
                                    .min()
                                    .expect("at least 1 subpacket under min."),
                                Operator::Max => *values
                                    .iter()
                                    .max()
                                    .expect("at least 1 subpacket under max."),
                                Operator::Gt => {
                                    if values[0] > values[1] {
                                        1
                                    } else {
                                        0
                                    }
                                }
                                Operator::Lt => {
                                    if values[0] < values[1] {
                                        1
                                    } else {
                                        0
                                    }
                                }
                                Operator::Eq => {
                                    if values[0] == values[1] {
                                        1
                                    } else {
                                        0
                                    }
                                }
                            },
                            Typ::Value => panic!("Invalid operator type."),
                        };
                        if let Some(last) = packet_stack.last_mut() {
                            last.1.push(res);
                        }
                        sum = res;
                    }
                }
                Data::Value(data) => {
                    if let Some(last) = packet_stack.last_mut() {
                        last.1.push(data);
                    }
                    sum = data;
                }
            }
        }
    }

    sum
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let packets = parse(input);
    let p1 = part1_vsum(&packets);
    let p2 = part2_eval(&packets);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
