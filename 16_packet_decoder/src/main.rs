use std::io::{BufRead, BufReader};
use std::num::TryFromIntError;
use std::str::FromStr;

#[derive(Debug)]
enum PacketError {
    /// An invalid character was found. Valid ones are: `0...9`, `a...f`
    /// or `A...F`.
    InvalidHexCharacter {
        c: char,
        index: usize,
    },
    InvalidPacketType {
        value: u64,
    },
    InvalidTryFrom(TryFromIntError),
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    content: PacketContent,
}

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Sum,
    Product,
    MinProduct,
    MaxProduct,
    Literal,
    GtProduct,
    LtProduct,
    EqProduct,
}

impl TryFrom<u64> for PacketType {
    type Error = PacketError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketType::Sum),
            1 => Ok(PacketType::Product),
            2 => Ok(PacketType::MinProduct),
            3 => Ok(PacketType::MaxProduct),
            4 => Ok(PacketType::Literal),
            5 => Ok(PacketType::GtProduct),
            6 => Ok(PacketType::LtProduct),
            7 => Ok(PacketType::EqProduct),
            _ => Err(PacketError::InvalidPacketType { value }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PacketContent {
    Literal(u64),
    SubPackets(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = PacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nibbles = s
            .chars()
            .enumerate()
            .map(|(idx, c)| hex_char_to_nibble(c as u8, idx))
            .collect::<Result<Vec<u8>, Self::Err>>()?;
        extract_packet(&nibbles, 0).map(|p| p.0)
    }
}

fn extract_packet(payload: &[u8], bit_pos: usize) -> Result<(Packet, usize), PacketError> {
    let version = u8::try_from(get_number(payload, bit_pos, 3))
        .map_err(|a| PacketError::InvalidTryFrom(a))?;

    let packet_type = PacketType::try_from(get_number(payload, bit_pos + 3, 3))?;
    let (content, bit_pos) = if packet_type == PacketType::Literal {
        read_literal(&payload, bit_pos + 6, 0)
    } else {
        read_operation(&payload, bit_pos + 6)
    }?;
    let packet = Packet {
        version,
        packet_type,
        content,
    };
    Ok((packet, bit_pos))
}

fn read_literal(
    payload: &[u8],
    bit_pos: usize,
    current_val: u64,
) -> Result<(PacketContent, usize), PacketError> {
    let has_more = get_bit(payload, bit_pos) != 0;
    let val = (current_val << 4) + get_number(payload, bit_pos + 1, 4);
    if has_more {
        read_literal(payload, bit_pos + 5, val)
    } else {
        Ok((PacketContent::Literal(val), bit_pos + 5))
    }
}

fn read_operation(payload: &[u8], bit_pos: usize) -> Result<(PacketContent, usize), PacketError> {
    let know_number_of_sub_packets = get_bit(payload, bit_pos) != 0;
    if know_number_of_sub_packets {
        let total_bits = get_number(payload, bit_pos + 1, 11);
        let (packets, bit_pos) =
            (0..total_bits).fold(Ok((Vec::new(), bit_pos + 12)), |r, _| match r {
                Ok((mut packets, start_bit_pos)) => {
                    let (packet, ended_bit_pos) = extract_packet(payload, start_bit_pos)?;
                    packets.push(packet);
                    Ok((packets, ended_bit_pos))
                }
                _ => r,
            })?;
        Ok((PacketContent::SubPackets(packets), bit_pos))
    } else {
        let total_bits = get_number(payload, bit_pos + 1, 15);
        let mut bit_pos = bit_pos + 16;
        let goal =
            bit_pos + usize::try_from(total_bits).map_err(|a| PacketError::InvalidTryFrom(a))?;
        let mut packets = Vec::new();
        while bit_pos < goal {
            let (packet, ended_bit_pos) = extract_packet(payload, bit_pos)?;
            packets.push(packet);
            bit_pos = ended_bit_pos;
        }
        Ok((PacketContent::SubPackets(packets), bit_pos))
    }
}

fn summed_versions(packet: &Packet) -> u64 {
    match &packet.content {
        PacketContent::Literal(_) => u64::from(packet.version),
        PacketContent::SubPackets(sp) => sp
            .iter()
            .fold(u64::from(packet.version), |s, c| s + summed_versions(c)),
    }
}

fn derived_values(packet: &Packet) -> u64 {
    match &packet.content {
        PacketContent::Literal(l) => match &packet.packet_type {
            PacketType::Literal => l.to_owned(),
            _ => panic!("Expecting literal type"),
        },
        PacketContent::SubPackets(sp) => {
            let mut values = sp.iter().map(|p| derived_values(p));

            match &packet.packet_type {
                PacketType::Sum => values.sum(),
                PacketType::Product => values.product(),
                PacketType::MinProduct => values.min().unwrap_or(0),
                PacketType::MaxProduct => values.max().unwrap_or(0),
                PacketType::GtProduct => {
                    let m = values.next().unwrap_or(0);
                    let m2 = values.next().unwrap_or(0);
                    if m > m2 {
                        1
                    } else {
                        0
                    }
                }
                PacketType::LtProduct => {
                    let m = values.next().unwrap_or(0);
                    let m2 = values.next().unwrap_or(0);
                    if m < m2 {
                        1
                    } else {
                        0
                    }
                }
                PacketType::EqProduct => {
                    let m = values.next().unwrap_or(0);
                    let m2 = values.next().unwrap_or(0);
                    if m == m2 {
                        1
                    } else {
                        0
                    }
                }
                PacketType::Literal => panic!("Expecting non literal type"),
            }
        }
    }
}

fn get_number(payload: &[u8], bit_pos: usize, count: usize) -> u64 {
    (0..count).fold(0, |v, i| {
        (v << 1) + u64::from(get_bit(payload, bit_pos + i))
    })
}

fn get_bit(payload: &[u8], bit_pos: usize) -> u8 {
    payload[bit_pos / 4] >> (3 - (bit_pos % 4)) & 0x1
}

fn hex_char_to_nibble(c: u8, idx: usize) -> Result<u8, PacketError> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(PacketError::InvalidHexCharacter {
            c: c as char,
            index: idx,
        }),
    }
}

fn main() {
    let lines: Vec<String> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(|f| f.ok())
        .collect();

    let input = lines[0].clone();
    let packet_result: Result<Packet, PacketError> = input.parse();
    match packet_result {
        Ok(p) => {
            let summed_versions_score = summed_versions(&p);
            println!("summed versions: {}", summed_versions_score);
            let derived_values_score = derived_values(&p);
            println!("derived values score: {}", derived_values_score);
        }
        Err(p_err) => match p_err {
            PacketError::InvalidHexCharacter { index, c } => {
                println!("Encountered invalid hex char '{}' at {}", c, index);
            }
            PacketError::InvalidPacketType { value } => {
                println!("Encountered invalid package type value {}", value);
            }
            PacketError::InvalidTryFrom(err) => {
                println!("Could not convert the following {}", err);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! packet_tests {
        ($($name:ident($input:expr, $field:ident: $expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                let packet: Result<Packet, PacketError> = $input.parse();
                let packet = packet.unwrap();

                assert_eq!(packet.$field, $expected);
            }
        )*
        }
    }

    macro_rules! packet_summed_version_tests {
        ($($name:ident($input:expr, summed_version: $expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                let packet: Result<Packet, PacketError> = $input.parse();
                let score = summed_versions(&packet.unwrap());

                assert_eq!(score, $expected);
            }
        )*
        }
    }

    macro_rules! packet_derived_values_tests {
        ($($name:ident($input:expr, summed_value: $expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                let packet: Result<Packet, PacketError> = $input.parse();
                let score = derived_values(&packet.unwrap());

                assert_eq!(score, $expected);
            }
        )*
        }
    }

    packet_tests! {
        literal_packet_version("D2FE28", version: 6),
        operator_packet_version("38006F45291200", version: 1),
        count_based_operator_packet_version("EE00D40C823060", version: 7),
    }

    packet_tests! {
        literal_packet_type_id("D2FE28", packet_type: PacketType::Literal),
        operator_packet_type_id("38006F45291200", packet_type: PacketType::LtProduct),
    }

    packet_tests! {
        single_group_literal_packet_content("D1E0", content: PacketContent::Literal(15)),
        single_group_literal_packet_content_variant("D1A0", content: PacketContent::Literal(13)),
        literal_packet_content("D2FE28", content: PacketContent::Literal(2021)),
        literal_packet_content_variant("D2BA38", content: PacketContent::Literal(1447)),
    }

    packet_tests! {
        bit_sized_operator_packet_content("38006F45291200", content: PacketContent::SubPackets(vec![
            Packet { version: 6, packet_type: PacketType::Literal, content: PacketContent::Literal(10) },
            Packet { version: 2, packet_type: PacketType::Literal, content: PacketContent::Literal(20) },
        ])),

        count_based_operator_packet_content("EE00D40C823060", content: PacketContent::SubPackets(vec![
            Packet { version: 2, packet_type: PacketType::Literal, content: PacketContent::Literal(1) },
            Packet { version: 4, packet_type: PacketType::Literal, content: PacketContent::Literal(2) },
            Packet { version: 1, packet_type: PacketType::Literal, content: PacketContent::Literal(3) },
        ])),
    }

    packet_summed_version_tests! {
        literal_packet_summed_version("D2FE28", summed_version: 6),

        bit_sized_operator_packet_summed_version("38006F45291200", summed_version: 9),

        count_based_operator_packet_summed_version("EE00D40C823060", summed_version: 14),
    }

    packet_derived_values_tests! {
        sum_derived_values("C200B40A82", summed_value: 3),
        product_derived_values("04005AC33890", summed_value: 54),
        minimum_derived_values("880086C3E88112", summed_value: 7),
        maximum_derived_values("CE00C43D881120", summed_value: 9),
        less_than_derived_values("D8005AC2A8F0", summed_value: 1),
        greater_than_derived_values("F600BC2D8F", summed_value: 0),
        not_equal_derived_values("9C005AC2F8F0", summed_value: 0),
        equal_derived_values("9C0141080250320F1802104A08", summed_value: 1),

    }
}
