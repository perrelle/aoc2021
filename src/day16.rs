fn bin_to_u32(bin: &[bool]) -> u64 {
    let mut x = 0;
    for &b in bin {
        x = x * 2 + if b {1} else {0};
    }
    x
}

fn u32_to_bin(i: u8) -> [bool ; 4] {
    let mut x = i;
    let mut bin = [false ; 4];
    for b in bin.iter_mut().rev() {
        *b = x % 2 == 1;
        x /= 2;
    }
    bin
}

mod parser {
    use nom::{IResult, character::complete::*, multi::*, combinator::*};
    use super::*;

    pub fn hexa(input: &[u8]) -> IResult<&[u8], [bool ; 4]> {
        let (input,c) = satisfy(|c|
            ('0'..='9').contains(&c) ||
            ('A'..='F').contains(&c))(input)?;
        let x = if ('0'..='9').contains(&c) {
            c as u8 - b'0'
        }
        else {
            c as u8 - b'A' + 10
        };
        Ok((input,u32_to_bin(x)))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<bool>> {
        let (input, numbers) = many1(hexa)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, numbers.concat()))
    }
}

fn parse_packet(packet: &[bool]) -> (u64, u64, usize) {
    let version = bin_to_u32(&packet[0..3]);
    let mut sum = version;
    let typ = bin_to_u32(&packet[3..6]);
    let mut s = 6;

    let value =
        if typ == 4 { // Literal value
            let mut bits = Vec::new();
            loop {
                let continued = packet[s];
                bits.push(&packet[s+1..s+5]);
                s += 5;
                if !continued {
                    break;
                }
            }
            let value = bin_to_u32(&bits.concat());
            println!("Litteral {}", value);
            value
        } else { // Operator
            let lengthtype = packet[6];
            let mut values = Vec::new();
            if lengthtype {
                let count = bin_to_u32(&packet[7..18]);
                s = 18;
                for _ in 1..=count {
                    let (sversion, value, tail) = parse_packet(&packet[s..]);
                    s += tail;
                    sum += sversion;
                    values.push(value);
                }
            }
            else {
                let size = bin_to_u32(&packet[7..22]) as usize;
                s = 22;
                while s < 22 + size {
                    let (sversion, value, tail) = parse_packet(&packet[s..]);
                    s += tail;
                    sum += sversion;
                    values.push(value);
                }
                if s != 22 + size {
                    panic!("Subpackets sizes do not match");
                }
            }

            let (op, result) =
                match typ {
                    0 => ("+", values.iter().sum()),
                    1 => ("*", values.iter().product()),
                    2 => ("min", *values.iter().min().unwrap()),
                    3 => ("max", *values.iter().max().unwrap()),
                    _ =>
                        match values.as_slice() {
                            [v1,v2] =>
                                match typ {
                                    5 => (">", if v1 > v2 {1} else {0}),
                                    6 => ("<", if v1 < v2 {1} else {0}),
                                    7 => ("=", if v1 == v2 {1} else {0}),
                                    _ => panic!() // typ is alwas < 8
                                }
                            _ => panic!("Comparison must be between two values")
                        }                   
                };
            println!("{}{:?} -> {}", op, values, result);
            result
        };
  (sum, value, s)
}


pub fn solve(input: &[u8]) -> (u64,u64) {
    let (_,packet) = parser::parse(input).unwrap();
    let (sversion,value,tail) = parse_packet(&packet);
    if packet[tail..].iter().any(|&b| b) {
        panic!("Tail is not 0: {:?}", &packet[tail..]);
    }
    (sversion,value)
}


#[test]
fn test16_0() {
    let solution = solve(include_bytes!("../inputs/day16.0"));
    assert_eq!(solution, (6,2021));
}

#[test]
fn test16_1() {
    let solution = solve(include_bytes!("../inputs/day16.1"));
    assert_eq!(solution, (9,1));
}

#[test]
fn test16_2() {
    let solution = solve(include_bytes!("../inputs/day16.2"));
    assert_eq!(solution, (16,15));
}

#[test]
fn test16_3() {
    let solution = solve(include_bytes!("../inputs/day16.3"));
    assert_eq!(solution, (12,46));
}

#[test]
fn test16_4() {
    let solution = solve(include_bytes!("../inputs/day16.4"));
    assert_eq!(solution, (23,46));
}

#[test]
fn test16_5() {
    let solution = solve(include_bytes!("../inputs/day16.5"));
    assert_eq!(solution, (31,54));
}

#[test]
fn test16_6() {
    let solution = solve(include_bytes!("../inputs/day16.6"));
    assert_eq!(solution, (14,3));
}

#[test]
fn test16_7() {
    let solution = solve(include_bytes!("../inputs/day16.7"));
    assert_eq!(solution, (8,54));
}

#[test]
fn test16_8() {
    let solution = solve(include_bytes!("../inputs/day16.8"));
    assert_eq!(solution, (15,7));
}

#[test]
fn test16_9() {
    let solution = solve(include_bytes!("../inputs/day16.9"));
    assert_eq!(solution, (11,9));
}

#[test]
fn test16_10() {
    let solution = solve(include_bytes!("../inputs/day16.10"));
    assert_eq!(solution, (13,1));
}

#[test]
fn test16_11() {
    let solution = solve(include_bytes!("../inputs/day16.11"));
    assert_eq!(solution, (19,0));
}

#[test]
fn test16_12() {
    let solution = solve(include_bytes!("../inputs/day16.12"));
    assert_eq!(solution, (16,0));
}

#[test]
fn test16_13() {
    let solution = solve(include_bytes!("../inputs/day16.13"));
    assert_eq!(solution, (20,1));
}

#[test]
fn test16_14() {
    let solution = solve(include_bytes!("../inputs/day16.14"));
    assert_eq!(solution, (951,902198718880));
}
