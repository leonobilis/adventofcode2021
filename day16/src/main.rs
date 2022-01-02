use bitvec::prelude::{BitSlice, BitVec, Msb0};
use std::fs;

fn parse_input(inp: String) -> Vec<Box<dyn Packet>> {
    let decoded = hex::decode(inp).unwrap();
    let data = BitVec::from_slice(&decoded).unwrap();

    let mut i = 0;
    let mut packets = Vec::<Box<dyn Packet>>::new();
    while i < data.len() - 11 {
        let header = Header::from_bit(&data[i..]);
        let packet: Box<dyn Packet> = match header.id {
            4 => Box::new(Literal::new(header, &data[i + 6..])),
            _ => Box::new(Operator::new(header, &data[i + 6..])),
        };
        i += packet.len();

        packets.push(packet);
    }
    packets
}

fn convert_to_uint(b: &BitSlice<Msb0, u8>) -> u32 {
    b.iter_ones().map(|idx| 1 << (b.len() - idx - 1)).sum()
}

trait Packet {
    fn sum_version(&self) -> u32;
    fn value(&self) -> u64;
    fn len(&self) -> usize;
}

struct Header {
    version: u32,
    id: u32,
}

impl Header {
    fn from_bit(b: &BitSlice<Msb0, u8>) -> Self {
        Self {
            version: convert_to_uint(b.get(..3).unwrap()),
            id: convert_to_uint(b.get(3..6).unwrap()),
        }
    }
}

struct Literal {
    header: Header,
    value: u64,
    len: usize,
}

impl Literal {
    fn new(header: Header, b: &BitSlice<Msb0, u8>) -> Self {
        let mut i = 0;
        let mut value = 0u64;

        loop {
            value = (value << 4) + convert_to_uint(&b[i * 5 + 1..i * 5 + 5]) as u64;
            if !b[i * 5] {
                break;
            }
            i += 1;
        }

        Self {
            header,
            value,
            len: 6 + (i + 1) * 5,
        }
    }
}

impl Packet for Literal {
    fn sum_version(&self) -> u32 {
        self.header.version
    }

    fn value(&self) -> u64 {
        self.value
    }

    fn len(&self) -> usize {
        self.len
    }
}

struct Operator {
    header: Header,
    sub_packets: Vec<Box<dyn Packet>>,
    len: usize,
}

enum LengthTypeID {
    BitsLen,
    PacketsNum,
}

impl Operator {
    fn new(header: Header, b: &BitSlice<Msb0, u8>) -> Self {
        let (mut start, end, mut pos, length_type_id) = match b[0] {
            true => (
                0,
                convert_to_uint(&b[1..12]) as usize,
                12,
                LengthTypeID::PacketsNum,
            ),
            false => (
                16usize,
                16 + convert_to_uint(&b[1..16]) as usize,
                16,
                LengthTypeID::BitsLen,
            ),
        };

        let mut sub_packets = Vec::<Box<dyn Packet>>::new();

        while start < end {
            let sub_header = Header::from_bit(&b[pos..]);
            let packet: Box<dyn Packet> = match sub_header.id {
                4 => Box::new(Literal::new(sub_header, &b[pos + 6..])),
                _ => Box::new(Operator::new(sub_header, &b[pos + 6..])),
            };
            pos += packet.len();
            start += match &length_type_id {
                LengthTypeID::PacketsNum => 1,
                LengthTypeID::BitsLen => packet.len(),
            };
            sub_packets.push(packet);
        }

        Self {
            header,
            sub_packets,
            len: 6 + pos,
        }
    }
}

impl Packet for Operator {
    fn sum_version(&self) -> u32 {
        self.sub_packets
            .iter()
            .map(|p| p.sum_version())
            .sum::<u32>()
            + self.header.version
    }

    fn value(&self) -> u64 {
        match self.header.id {
            0 => self.sub_packets.iter().map(|p| p.value()).sum(),
            1 => self.sub_packets.iter().map(|p| p.value()).product(),
            2 => self.sub_packets.iter().map(|p| p.value()).min().unwrap(),
            3 => self.sub_packets.iter().map(|p| p.value()).max().unwrap(),
            5 => {
                if self.sub_packets[0].value() > self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if self.sub_packets[0].value() < self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if self.sub_packets[0].value() == self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            _ => panic!(),
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

fn p1(packets: &[Box<dyn Packet>]) -> u32 {
    packets.iter().map(|p| p.sum_version()).sum()
}

fn p2(packets: &[Box<dyn Packet>]) -> u64 {
    packets[0].value()
}

fn main() {
    let inp = fs::read_to_string("input.txt").unwrap();
    let packets = parse_input(inp);

    println!("Part 1: {}", p1(&packets));
    println!("Part 2: {}", p2(&packets));
}
