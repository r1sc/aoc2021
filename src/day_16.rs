struct LiteralPacket {
    value: u64,
}

struct OperatorPacket {
    sub_packets: Vec<Packet>,
}

enum PacketData {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

struct Packet {
    id: u32,
    version: u32,
    data: PacketData,
}

impl Packet {
    fn print(&self) {
        println!("---- PACKET START ----");
        println!("Version {}, Id {}", self.version, self.id);
        match &self.data {
            PacketData::Literal(literal) => println!("Literal packet value: {}", literal.value),
            PacketData::Operator(operator) => {
                println!(
                    "Operator packet sub packet count: {}",
                    operator.sub_packets.len()
                );
                for sub_packet in &operator.sub_packets {
                    sub_packet.print();
                }
            }
        }
        println!("---- PACKET END ----");
    }

    fn sum_versions(&self) -> u32 {
        let mut result = 0_u32;
        match &self.data {
            PacketData::Literal(_) => result += self.version,
            PacketData::Operator(operator) => {
                result += self.version;
                for sub_packet in &operator.sub_packets {
                    result += sub_packet.sum_versions();
                }
            }
        }
        result
    }

    fn evaluate(&self) -> u64 {
        match &self.data {
            PacketData::Literal(literal) => literal.value,
            PacketData::Operator(operator) => {
                let mut evaluations = operator.sub_packets.iter().map(Packet::evaluate);
                match self.id {
                    // Sum packet
                    0 => evaluations.sum(),
                    // Product packet
                    1 => evaluations.product(),
                    // Minimum packet
                    2 => evaluations.fold(u64::MAX, |acc, cur| if acc < cur { acc } else { cur }),
                    // Maximum packet
                    3 => evaluations.fold(u64::MIN, |acc, cur| if acc > cur { acc } else { cur }),
                    // Greater than packets
                    5 => {
                        let (a, b) = (evaluations.next().unwrap(), evaluations.next().unwrap());
                        if a > b {
                            1
                        } else {
                            0
                        }
                    },
                    // Less than packets
                    6 => {
                        let (a, b) = (evaluations.next().unwrap(), evaluations.next().unwrap());
                        if a < b {
                            1
                        } else {
                            0
                        }
                    },
                    // Equal to packets
                    7 => {
                        let (a, b) = (evaluations.next().unwrap(), evaluations.next().unwrap());
                        if a == b {
                            1
                        } else {
                            0
                        }
                    },
                    _ => panic!("Unknown operator packet type")
                }
            }
        }
    }
}

struct Bitstream {
    bits: Vec<char>,
    position: usize,
}

impl Bitstream {
    fn new(bitstring: &str) -> Self {
        let bits: Vec<_> = bitstring.chars().collect();
        Self { bits, position: 0 }
    }

    fn from_hex(hex: &str) -> Self {
        let bitstring: String = hex
            .chars()
            .map(|c| match c {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _ => panic!("Invalid hex digit"),
            })
            .collect();

        Self::new(&bitstring)
    }

    fn pull_bits(&mut self, num_bits: u32) -> String {
        let str: String = self.bits[self.position..self.position + (num_bits as usize)]
            .into_iter()
            .collect();
        self.position += num_bits as usize;
        str
    }

    fn has_bits(&self) -> bool {
        self.position < self.bits.len()
    }

    fn pull_number(&mut self, num_bits: u32) -> u32 {
        let bits = self.pull_bits(num_bits);
        u32::from_str_radix(&bits[..], 2).unwrap()
    }

    fn parse_as_number(&mut self) -> u64 {
        let str: String = self.bits.iter().collect();
        u64::from_str_radix(&str, 2).unwrap()
    }

    fn print(&self) {
        let str: String = self.bits.iter().collect();
        println!("{}", str);
    }

    fn parse_literal_packet(&mut self) -> LiteralPacket {
        let mut result = String::new();
        loop {
            let has_more = self.pull_number(1);
            result += &self.pull_bits(4);
            if has_more == 0 {
                break;
            }
        }
        let value = Bitstream::new(&result).parse_as_number();
        LiteralPacket { value }
    }

    fn parse_operator_packet(&mut self) -> OperatorPacket {
        let length_type_id = self.pull_number(1);
        match length_type_id {
            0 => {
                let num_subpacket_bits = self.pull_number(15);
                let mut sub_bitstream = Bitstream::new(&self.pull_bits(num_subpacket_bits));
                let sub_packets = sub_bitstream.parse_all_packets();
                OperatorPacket { sub_packets }
            }
            1 => {
                let num_subpackets = self.pull_number(11);
                let mut sub_packets: Vec<Packet> = Vec::new();
                for _ in 0..num_subpackets {
                    sub_packets.push(self.parse_packet());
                }
                OperatorPacket { sub_packets }
            }
            _ => panic!("Unrecognized length type id"),
        }
    }

    fn parse_packet(&mut self) -> Packet {
        let version = self.pull_number(3);
        let id = self.pull_number(3);
        match id {
            4 => Packet {
                version,
                id,
                data: PacketData::Literal(self.parse_literal_packet()),
            },
            _ => Packet {
                version,
                id,
                data: PacketData::Operator(self.parse_operator_packet()),
            },
        }
    }

    fn parse_all_packets(&mut self) -> Vec<Packet> {
        let mut packets: Vec<Packet> = Vec::new();
        while self.has_bits() {
            packets.push(self.parse_packet());
        }
        packets
    }
}

pub fn main(data: Vec<&str>) -> (u32, u64) {
    let mut bitstream = Bitstream::from_hex(data[0]);
    let packet = bitstream.parse_packet();
    (packet.sum_versions(), packet.evaluate())
}

#[test]
fn test_part1_samples_1_2_3_4() {
    assert_eq!(main(vec!["8A004A801A8002F478"]).0, 16);
    assert_eq!(main(vec!["620080001611562C8802118E34"]).0, 12);
    assert_eq!(main(vec!["C0015000016115A2E0802F182340"]).0, 23);
    assert_eq!(main(vec!["A0016C880162017C3686B18A3D4780"]).0, 31);
}

#[test]
fn test_part2_samples_1_2_3_4_5_6_7_8() {
    assert_eq!(main(vec!["C200B40A82"]).1, 3);
    assert_eq!(main(vec!["04005AC33890"]).1, 54);
    assert_eq!(main(vec!["880086C3E88112"]).1, 7);
    assert_eq!(main(vec!["CE00C43D881120"]).1, 9);
    assert_eq!(main(vec!["D8005AC2A8F0"]).1, 1);
    assert_eq!(main(vec!["F600BC2D8F"]).1, 0);
    assert_eq!(main(vec!["9C005AC2F8F0"]).1, 0);
    assert_eq!(main(vec!["9C0141080250320F1802104A08"]).1, 1);
}