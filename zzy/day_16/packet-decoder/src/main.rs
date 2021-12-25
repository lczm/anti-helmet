//!
//! Anti Helmet
//! Advent of Code
//! Day 16: Packet Decoder
//!

use std::io::{stdin, Read};

/// Parse the given bitmap as an integer
fn parse_int(bitmap: &[bool]) -> u64 {
    if bitmap.len() > 64 {
        panic!("Parsing the bitmap as an integer will cause an overflow");
    }
    let bin_str = bitmap
        .iter()
        .map(|&has_bit| if has_bit { "1" } else { "0" })
        .collect::<String>();

    u64::from_str_radix(&bin_str, 2).expect("Failed to parse the given bitmap as an integer")
}

/// Ungroup literal (5bit) groups and returned extracted data bits
/// Parses (5 bit) groups with the first bit of each group signifying if
/// more groups are present and the extracts the latter 4 data bits.
/// Returns the data bits parsed and the no. total no of bits ungrouped.
fn ungroup(bitmap: &[bool]) -> (Vec<bool>, usize) {
    let more_groups = bitmap[0];
    let (mut data, mut n_read) = (Vec::new(), 1);

    // read 4 latter data bits in group
    data.extend(&bitmap[n_read..n_read + 4]);
    n_read += 4;

    if more_groups {
        // more groups present: recursively ungroup bitmap
        let (sub_data, sub_read) = ungroup(&bitmap[n_read..]);
        data.extend(sub_data);
        n_read += sub_read;
    }
    (data, n_read)
}

/// Defines the possible BITS Packet expressions
#[derive(Debug)]
enum Expr {
    Literal(u64),
    Sum(Vec<Box<Packet>>),
    Product(Vec<Box<Packet>>),
    Min(Vec<Box<Packet>>),
    Max(Vec<Box<Packet>>),
    Greater(Vec<Box<Packet>>),
    Less(Vec<Box<Packet>>),
    Equal(Vec<Box<Packet>>),
}
impl Expr {
    /// Parse the packet expr of the packet of the given type.
    /// Returns the parsed packet expr and the no. of bits read when parsing.
    fn parse(type_id: u8, bitmap: &[bool]) -> (Self, usize) {
        use Expr::*;
        match type_id {
            4 => {
                // parse literal packet containing an integer value
                let (int_bits, n_read) = ungroup(bitmap);
                (Literal(parse_int(&int_bits)), n_read)
            }
            _ => {
                // parse operator packet & recursively parse sub packets
                let is_length_type_1 = bitmap[0];
                let (mut sub_packets, mut n_read) = (Vec::new(), 1);

                if is_length_type_1 {
                    // length type 1: 10 bits forming no. of sub packets
                    let n_sub_packets = parse_int(&bitmap[n_read..n_read + 11]);
                    n_read += 11;

                    for _ in 0..n_sub_packets {
                        let (sub_packet, n_bits) = Packet::parse(&bitmap[n_read..]);
                        sub_packets.push(Box::new(sub_packet));
                        // advance no. of bits read to read next packet
                        n_read += n_bits;
                    }
                } else {
                    // length type 0: 15 bits forming the total bit length of sub packets
                    let n_sub_packet_bits = parse_int(&bitmap[n_read..n_read + 15]) as usize;
                    let mut n_sub_bits = 0;
                    n_read += 15;

                    while n_sub_bits < n_sub_packet_bits {
                        let (sub_packet, n_bits) = Packet::parse(&bitmap[n_read..]);
                        sub_packets.push(Box::new(sub_packet));
                        // advance no. of bits read to read next packet
                        n_read += n_bits;
                        n_sub_bits += n_bits;
                    }
                }

                // construct operator used parsed sub packets and type id
                let operator = match type_id {
                    0 => Sum(sub_packets),
                    1 => Product(sub_packets),
                    2 => Min(sub_packets),
                    3 => Max(sub_packets),
                    5 if sub_packets.len() == 2 => Greater(sub_packets),
                    6 if sub_packets.len() == 2 => Less(sub_packets),
                    7 if sub_packets.len() == 2 => Equal(sub_packets),
                    _ => panic!(
                        "Attempted to parse packet body with unsupported type id / sub packets."
                    ),
                };

                (operator, n_read)
            }
        }
    }

    /// Evaluate this packet expression to derive its value
    fn eval(&self) -> u64 {
        use Expr::*;

        match self {
            Literal(value) => *value,
            Sum(sub_packets) => sub_packets.into_iter().map(|packet| packet.eval()).sum(),
            Product(sub_packets) => sub_packets
                .into_iter()
                .map(|packet| packet.eval())
                .product(),
            Min(sub_packets) => sub_packets
                .into_iter()
                .map(|packet| packet.eval())
                .min()
                .unwrap(),
            Max(sub_packets) => sub_packets
                .into_iter()
                .map(|packet| packet.eval())
                .max()
                .unwrap(),
            Greater(sub_packets) => {
                if sub_packets[0].eval() > sub_packets[1].eval() {
                    1
                } else {
                    0
                }
            }
            Less(sub_packets) => {
                if sub_packets[0].eval() < sub_packets[1].eval() {
                    1
                } else {
                    0
                }
            }
            Equal(sub_packets) => {
                if sub_packets[0].eval() == sub_packets[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

/// Represents a single packet in BITS transmissio
#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    expr: Expr,
}
impl Packet {
    /// Read the given binary bitmap representation of a packet expression  & parse.
    /// Returns the parsed packet ppacket the no. of bits read when parsing.
    fn parse(bitmap: &[bool]) -> (Packet, usize) {
        let mut n_read = 0;
        let version = parse_int(&bitmap[n_read..n_read + 3]) as u8;
        n_read += 3;
        let type_id = parse_int(&bitmap[n_read..n_read + 3]) as u8;
        n_read += 3;
        // parse packet expr based on type id
        let (expr, n_expr_bits) = Expr::parse(type_id, &bitmap[n_read..]);
        n_read += n_expr_bits;
        (
            Packet {
                version: version,
                type_id: type_id,
                expr: expr,
            },
            n_read,
        )
    }

    /// Folds every packet nested in this packet into an accumulator by applying
    /// given function f.
    fn fold<A, F: Fn(A, &Packet) -> A + Copy>(&self, acc: A, f: F) -> A {
        use Expr::*;
        match &self.expr {
            Literal(_) => f(acc, self),
            Sum(sub_packets) | Product(sub_packets) | Min(sub_packets) | Max(sub_packets)
            | Greater(sub_packets) | Less(sub_packets) | Equal(sub_packets) => {
                // fold nested sub packets using function
                let sub_acc = sub_packets
                    .iter()
                    .map(|boxed_packet| &**boxed_packet)
                    .fold(acc, |acc, packet| packet.fold(acc, f));
                f(sub_acc, self)
            }
        }
    }

    /// Evaluate the expression contained in this packet
    /// Returns the value result of evaluating the packet's expression
    fn eval(&self) -> u64 {
        self.expr.eval()
    }
}

fn main() {
    // read the hexidemcial BITS transmission into a binary bitmap
    let mut hex_msg = String::new();
    stdin()
        .read_to_string(&mut hex_msg)
        .expect("Failed to read BITS transmission from stdin");
    let bin_msg: Vec<_> = hex_msg
        .trim_end()
        .chars()
        .map(|hex| {
            u8::from_str_radix(&hex.to_string(), 16)
                .expect("Failed to parse BITS transmission as an hex digit")
        })
        .flat_map(|nibble| {
            (0..4)
                .rev()
                .map(|bit| nibble & (1 << bit) == (1 << bit))
                .collect::<Vec<_>>()
        })
        .collect();

    let (root, _) = Packet::parse(&bin_msg);
    let version_sum = root.fold(0, |sum, packet| sum + packet.version as u32);
    println!("Version sum: {}", version_sum);
    println!("Evaluated value: {}", root.eval());
}
