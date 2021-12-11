pub enum Command { Forward, Down, Up }
pub type Order = (Command,i32);

mod parser  {
    use nom::{IResult, multi::*, character::complete::*,  sequence::*, combinator::*};
    pub fn command(input: &[u8]) -> IResult<&[u8], super::Order> {
        let (input,(cmd,_,units)) = tuple((alpha1, space1, i32))(input)?;
        let cmd = match std::str::from_utf8(cmd).unwrap() {
                "forward" => super::Command::Forward,
                "down" => super::Command::Down,
                "up" => super::Command::Up,
                _ => { panic!("incorrect command"); }
            };
        Ok((input,(cmd,units)))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<super::Order>> {
        let (input, orders) = separated_list1(multispace1, command)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, orders))
    }
}

fn part1(commands: &Vec<Order>) -> i32 { 
    let mut x = 0;
    let mut z = 0;

    for (order,units) in commands {
        match order {
            Command::Forward => { x += units; },
            Command::Down => { z += units; },
            Command::Up => { z -= units; }
        }
    }

    println!("Step 1 - Final position ({},{}). Answer: {}", x, z, x * z);
    x * z
}

fn part2(commands: &Vec<Order>) -> i32 { 
    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;

    for (order,units) in commands {
        match order {
            Command::Forward => { x += units; z += aim * units; },
            Command::Down => { aim += units; },
            Command::Up => { aim -= units; }
        }
    }

    println!("Step 2 - Final position ({},{}). Answer: {}", x, z, x * z);
    x * z
}

pub fn solve(data: &[u8]) -> (i32,i32) {
    let (_,commands) = parser::parse(data).unwrap();
    (part1(&commands),part2(&commands))
}

#[test]
fn test2_0() {
    let solution = solve(include_bytes!("../inputs/day2.0"));
    assert_eq!(solution, (150,900));
}

#[test]
fn test2_1() {
    let solution = solve(include_bytes!("../inputs/day2.1"));
    assert_eq!(solution, (1604850,1685186100));
}

