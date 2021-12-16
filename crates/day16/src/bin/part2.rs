use day16::*;
use std::{
    io::BufRead,
};

fn solution(mut input_reader: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let mut input = String::new();
    input_reader.read_to_string(&mut input)?;
    let input = input.trim();
    let bytes = input.chars().step_by(2).zip(input.chars().skip(1).step_by(2)).map(|(hi,lo)|{
        let hi = hi.to_digit(16).ok_or("Not hexadecimal digit")? as u8;
        let lo = lo.to_digit(16).ok_or("Not hexadecimal digit")? as u8;
        Ok::<_,advent_of_utils::Error>(hi << 4 | lo)
    }).collect::<Result<Vec<_>,_>>()?;

    let bits = bytes_to_bits(bytes);

    let mut bit_stream = BitStream::new(&bits);

    let packet = read_packet(&mut bit_stream);

    Ok(packet.evaluate())
}

fn bytes_to_bits(bytes: Vec<u8>) -> Vec<bool> {
    let bits = bytes.into_iter().flat_map(|byte|{
        vec![
            (byte >> 7 & 0b1) == 1,
            (byte >> 6 & 0b1) == 1,
            (byte >> 5 & 0b1) == 1,
            (byte >> 4 & 0b1) == 1,
            (byte >> 3 & 0b1) == 1,
            (byte >> 2 & 0b1) == 1,
            (byte >> 1 & 0b1) == 1,
            (byte >> 0 & 0b1) == 1,
        ]
    }).collect::<Vec<_>>();
    bits
}

struct Packet{
    version: u8,
    type_id: u8,
    data: PacketData
}

impl Packet {
    fn evaluate(&self) -> u64 {
        self.data.evaluate()
    }
}

enum Operator{
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

enum PacketData{
    Literal{
        data: u64,
    },
    Operator{
        operator: Operator,
        packets: Vec<Packet>,
    }
}

impl PacketData {
    fn evaluate(&self) ->u64 {
        match self{
            PacketData::Literal { data } => *data,
            PacketData::Operator { operator, packets } => {
                match operator{
                    Operator::Sum => packets.iter().map(|p| p.evaluate()).sum(),
                    Operator::Product => packets.iter().map(|p| p.evaluate()).product(),
                    Operator::Minimum => packets.iter().map(|p| p.evaluate()).min().expect("No minimum"),
                    Operator::Maximum => packets.iter().map(|p| p.evaluate()).max().expect("No maximum"),
                    Operator::Greater => if packets[0].evaluate() > packets[1].evaluate() { 1 } else { 0 }
                    Operator::Less => if packets[0].evaluate() < packets[1].evaluate() { 1 } else { 0 }
                    Operator::Equal => if packets[0].evaluate() == packets[1].evaluate() { 1 } else { 0 }
                }
            }
        }
    }
}

fn read_packet<'a>(bit_stream: &mut BitStream<'a>) -> Packet{
    let version = bit_stream.get_bits(3) as u8;
    let type_id = bit_stream.get_bits(3) as u8;

    let data = if type_id == 4{
        //literal packet
        let mut more_bits = true;
        let mut data = vec![];
        let mut working_byte = None;
        while more_bits {
            let encoded = bit_stream.get_bits(5) as u8;
            let value = encoded & 0xF;
            more_bits = (encoded & 0x10) != 0;
            working_byte = if let Some(working_byte) = working_byte{
                data.push(working_byte << 4 | value);
                None
            }else{
                Some(value)
            };
        }
        let shift_back = if let Some(working_byte) = working_byte{
            data.push(working_byte << 4);
            true
        }else{
            false
        };
        let mut data = data.iter().fold(0u64,|acc,byte|{
            acc << 8 | (*byte as u64)
        });
        if shift_back {
            data = data >> 4;
        }
        PacketData::Literal{
            data,
        }
    }else{
        let operator = match type_id{
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Minimum,
            3 => Operator::Maximum,
            5 => Operator::Greater,
            6 => Operator::Less,
            7 => Operator::Equal,
            other => panic!("Unexpected operator type_id: {}", other),
        };
        //operator packet
        let len_type_id = bit_stream.get_bits(1);
        if len_type_id == 0 {
            let bit_length = bit_stream.get_bits(15);
            let mut sub_reader = bit_stream.get_sub_reader(bit_length as usize);
            let mut packets = vec![];
            while sub_reader.has_bits(){
                packets.push(read_packet(&mut sub_reader))
            }
            PacketData::Operator{
                operator,
                packets,
            }
        }else{
            let packet_length = bit_stream.get_bits(11) as usize;
            let mut packets = vec![];
            while packets.len() != packet_length{
                packets.push(read_packet(bit_stream))
            }
            PacketData::Operator{
                operator,
                packets,
            }
        }
    };
    Packet{
        version,
        type_id,
        data
    }
}

struct BitStream<'a>{
    bits: &'a [bool],
    bit_offset: usize,
}

impl<'a> std::fmt::Display for BitStream<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &bit in &self.bits[self.bit_offset..]{
            write!(f,"{}",if bit {'1'} else {'0'})?;
        }
        Ok(())
    }
}

impl<'a> BitStream<'a>{
    pub fn new(bits: &'a [bool]) -> Self{
        Self{
            bits,
            bit_offset: 0,
        }
    }
    pub fn get_bits(&mut self, mut bit_len: usize) -> u64
    {
        let bits = &self.bits[self.bit_offset..][..bit_len];
        self.bit_offset += bit_len;
        let mut acc = 0u64;
        for &bit in bits{
            acc = (acc << 1) | if bit {1} else {0};
        }
        acc
    }

    pub fn get_sub_reader(&mut self, bit_len: usize) -> BitStream<'a> {
        let sub_reader = Self{
            bits: &self.bits[self.bit_offset..][..bit_len],
            bit_offset: 0,
        };
        self.bit_offset += bit_len;
        sub_reader
    }

    pub fn has_bits(&self) -> bool {
        self.bit_offset < self.bits.len()
    }
}

#[cfg(test)]
#[test]
fn bitstream_smoke_test(){
    let packed = vec![0xAF,0xFF];
    let bits = bytes_to_bits(packed);
    let mut bitstream = BitStream::new(&bits);
    assert_eq!(bitstream.get_bits(4), 0xA);
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day16_part2_example1() {
    advent_of_utils::check_example(
        solution,
        "C200B40A82",
        3,
    );
}

#[cfg(test)]
#[test]
fn day16_part2_example2() {
    advent_of_utils::check_example(
        solution,
        "04005AC33890",
        54,
    );
}

#[cfg(test)]
#[test]
fn day16_part2_example3() {
    advent_of_utils::check_example(
        solution,
        "880086C3E88112",
        7,
    );
}


#[cfg(test)]
#[test]
fn day16_part2_example4() {
    advent_of_utils::check_example(
        solution,
        "CE00C43D881120",
        9,
    );
}

#[cfg(test)]
#[test]
fn day16_part2_example5() {
    advent_of_utils::check_example(
        solution,
        "D8005AC2A8F0",
        1,
    );
}

#[cfg(test)]
#[test]
fn day16_part2_example6() {
    advent_of_utils::check_example(
        solution,
        "F600BC2D8F",
        0,
    );
}

#[cfg(test)]
#[test]
fn day16_part2_example7() {
    advent_of_utils::check_example(
        solution,
        "9C005AC2F8F0",
        0,
    );
}

#[cfg(test)]
#[test]
fn day16_part2_example8() {
    advent_of_utils::check_example(
        solution,
        "9C0141080250320F1802104A08",
        1,
    );
}

#[cfg(test)]
#[test]
fn day16_part2_multibyte_literal() {
    advent_of_utils::check_example(
        solution,
        "d2fe28",
        2021,
    );
}