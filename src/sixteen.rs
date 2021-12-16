use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let numbers = read_input();
    let answer = solve_1(&numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
    let answer = solve_2(&numbers);

    println!("{}", answer);
}

fn i64_to_bits(value: i64) -> Vec<u8> {
    let mut bits = vec![];

    for i in (0..4).rev() {
        let mask = 1 << i;
        if value & mask != 0 {
            bits.push(1);
        } else {
            bits.push(0);
        }
    }

    bits
}

fn bits_to_i64(bits: &[u8]) -> i64 {
    let mut value: i64 = 0;

    for b in bits.iter() {
        value <<= 1;
        value += *b as i64;
    }

    value
}

fn read_input() -> Vec<u8> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let hex_nums: Vec<i64> = line
            .chars()
            .map(|h| i64::from_str_radix(&(h.to_string()), 16).unwrap())
            .collect();

        return hex_nums.iter().map(|d| i64_to_bits(*d)).flatten().collect();
    }

    panic!()
}

#[derive(Debug)]
struct Header {
    version: i64,
    packet_id: i64,
}

impl Header {
    fn from_bits(buffer: &[u8], start: usize) -> Header {
        let version = bits_to_i64(&buffer[start..(start + 3)]);
        let packet_id = bits_to_i64(&buffer[(start + 3)..(start + 6)]);

        Header { version, packet_id }
    }
}

#[derive(Debug)]
enum Packet {
    LiteralValue(Header, i64),
    Operator(Header, Vec<Packet>),
}

impl Packet {
    fn get_sum_versions(&self) -> i64 {
        match self {
            Packet::LiteralValue(header, _) => header.version,
            Packet::Operator(header, sub_packets) => {
                let child_versions_sum: i64 =
                    sub_packets.iter().map(|p| p.get_sum_versions()).sum();
                header.version + child_versions_sum
            }
        }
    }

    fn calc(&self) -> i64 {
        match self {
            Packet::LiteralValue(_, v) => *v,
            Packet::Operator(header, subs) => {
                let sub_values: Vec<i64> = subs.iter().map(|p| p.calc()).collect();

                match header.packet_id {
                    0 => sub_values.iter().sum(),
                    1 => sub_values.iter().product(),
                    2 => *sub_values.iter().min().unwrap(),
                    3 => *sub_values.iter().max().unwrap(),
                    5 => {
                        if sub_values[0] > sub_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if sub_values[0] < sub_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if sub_values[0] == sub_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!(),
                }
            }
        }
    }

    fn from_bits(buffer: &[u8], start: usize) -> (Packet, usize) {
        let header = Header::from_bits(buffer, start);

        match header.packet_id {
            4 => Packet::literal_value_from_bits(header, buffer, start + 6),
            _ => Packet::operator_from_bits(header, buffer, start + 6),
        }
    }

    fn literal_value_from_bits(header: Header, buffer: &[u8], start: usize) -> (Packet, usize) {
        let mut value = 0;

        let mut i = start;
        let mut done = false;
        while !done {
            if buffer[i] == 0 {
                done = true;
            }

            i += 1;

            value <<= 4;
            value += bits_to_i64(&buffer[i..(i + 4)]);

            i += 4;
        }

        (Packet::LiteralValue(header, value), i)
    }

    fn operator_from_bits(header: Header, buffer: &[u8], start: usize) -> (Packet, usize) {
        let mut i = start;

        let length_type_id = buffer[i];
        i += 1;

        let length = match length_type_id {
            0 => {
                let len = bits_to_i64(&buffer[i..(i + 15)]);
                i += 15;

                len
            }
            1 => {
                let len = bits_to_i64(&buffer[i..(i + 11)]);
                i += 11;

                len
            }
            _ => panic!(),
        };

        let mut j = i;
        let mut sub_i = 0;
        let mut sub_packets = vec![];
        loop {
            sub_i += 1;

            if length_type_id == 0 && j >= i + length as usize {
                break;
            }
            if length_type_id == 1 && sub_i > length {
                break;
            }

            let results = Packet::from_bits(buffer, j);

            j = results.1;
            sub_packets.push(results.0);
        }

        (Packet::Operator(header, sub_packets), j)
    }
}

fn solve_1(data: &[u8]) -> i64 {
    let packet = Packet::from_bits(data, 0);

    packet.0.get_sum_versions()
}

fn solve_2(data: &[u8]) -> i64 {
    let packet = Packet::from_bits(data, 0);

    packet.0.calc()
}
