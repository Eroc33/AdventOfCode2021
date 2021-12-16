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

    Ok(packet.version_sum())
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
    fn version_sum(&self) -> u64 {
        self.version as u64 + self.data.version_sum()
    }
}

enum PacketData{
    Literal{
        data: Vec<u8>,
        data_len: usize,
    },
    Operator{
        packets: Vec<Packet>,
    }
}

impl PacketData {
    fn version_sum(&self) ->u64 {
        match self{
            PacketData::Literal { data, data_len } => 0,
            PacketData::Operator { packets } => packets.iter().map(|p| p.version_sum()).sum(),
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
        let mut data_len = 0;
        let mut working_byte = None;
        while more_bits {
            let encoded = bit_stream.get_bits(5) as u8;
            let value = encoded & 0xF;
            more_bits = (encoded & 0x10) != 0;
            working_byte = Some(working_byte.unwrap_or(0) << 4 | value);
            data_len += 4;
            if data_len % 8 == 0 {
                data.push(working_byte.unwrap());
                working_byte = None;
            }
        }
        if let Some(working_byte) = working_byte{
            data.push(working_byte);
        }
        PacketData::Literal{
            data,
            data_len,
        }
    }else{
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
                packets,
            }
        }else{
            let packet_length = bit_stream.get_bits(11) as usize;
            let mut packets = vec![];
            while packets.len() != packet_length{
                packets.push(read_packet(bit_stream))
            }
            PacketData::Operator{
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
fn day16_part1_example1() {
    advent_of_utils::check_example(
        solution,
        "8A004A801A8002F478",
        16,
    );
}

#[cfg(test)]
#[test]
fn day16_part1_example2() {
    advent_of_utils::check_example(
        solution,
        "620080001611562C8802118E34",
        12,
    );
}

#[cfg(test)]
#[test]
fn day16_part1_example3() {
    advent_of_utils::check_example(
        solution,
        "C0015000016115A2E0802F182340",
        23,
    );
}


#[cfg(test)]
#[test]
fn day16_part1_example4() {
    advent_of_utils::check_example(
        solution,
        "C0015000016115A2E0802F182340",
        23,
    );
}