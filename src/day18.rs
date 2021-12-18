#[derive(Debug, Clone)]
pub enum SnailNum {
    Regular(i32),
    Pair(Box<SnailNum>, Box<SnailNum>)
} 

impl std::fmt::Display for SnailNum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SnailNum::Regular(x) => write!(f, "{}", x),
            SnailNum::Pair(sn1, sn2) => write!(f, "[{},{}]", sn1, sn2)
        }       
    }
}


pub fn pair(sn1: SnailNum, sn2: SnailNum) -> SnailNum {
    SnailNum::Pair(Box::new(sn1), Box::new(sn2))
}

pub fn regular(x: i32) -> SnailNum {
    SnailNum::Regular(x)
}

mod parser {
    use nom::{
        IResult, character::complete::*, bytes::complete::*, multi::*,
        combinator::*, sequence::* };
    use super::*;

    pub fn snum(input: &[u8]) -> IResult<&[u8], SnailNum> {
        if let Ok((input, n)) = i32::<&[u8],nom::error::Error<&[u8]>>(input) {
            Ok((input, SnailNum::Regular(n)))
        }
        else {
            let (input, (_, sn1, _, sn2, _)) =
                tuple((tag("["), snum, tag(","), snum, tag("]")))(input)?;
            Ok((input, super::pair(sn1, sn2)))
        }
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<SnailNum>> {
        let (input, numbers) = separated_list1(multispace1, snum)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, numbers))
    }
}

fn add_right(sn: SnailNum, y: i32) -> SnailNum {
    match sn {
        SnailNum::Regular(x) => regular(x+y),
        SnailNum::Pair(sn1, sn2) =>
            SnailNum::Pair(sn1, Box::new(add_right(*sn2, y)))
    }
}

fn add_left(sn: SnailNum, y: i32) -> SnailNum {
    match sn {
        SnailNum::Regular(x) => regular(x+y),
        SnailNum::Pair(sn1, sn2) =>
            SnailNum::Pair(Box::new(add_left(*sn1, y)), sn2)
    }
}


fn explode(snroot: SnailNum, depth: u32) -> (SnailNum,Option<(i32,i32)>) {
    match snroot {
        SnailNum::Regular(_) => (snroot, None),
        SnailNum::Pair(sn1, sn2) => {
            if depth < 4 {
                let (sn1,explosion) = explode(*sn1, depth + 1);
                if let Some((l,r)) = explosion {
                    let sn2 = if r != 0 { add_left(*sn2, r) } else { *sn2 };
                    (pair(sn1,sn2),Some((l,0)))
                }
                else {
                    let (sn2,explosion) = explode(*sn2, depth + 1);
                    if let Some((l,r)) = explosion {
                        let sn1 = if l != 0 { add_right(sn1, l) } else { sn1 };
                        (pair(sn1,sn2),Some((0,r)))
                    }
                    else {
                        (pair(sn1,sn2),None)
                    }
                }
            }
            else if let (SnailNum::Regular(x1),SnailNum::Regular(x2)) = (*sn1,*sn2) {
                (regular(0),Some((x1,x2)))
            }
            else {
                panic!("Found pair at depth > 4")
            }
        }
    }
}


fn split(sn: SnailNum) -> (SnailNum,bool) {
    match sn {
        SnailNum::Regular(x) => {
            if x < 10 {
                (sn, false)
            }
            else {
                (pair(regular(x / 2), regular(x / 2 + x % 2)), true)
            }
        }
        SnailNum::Pair(sn1, sn2) => {
            let (sn1,did_split) = split(*sn1);
            if did_split {
                (SnailNum::Pair(Box::new(sn1),sn2),true)
            }
            else {
                let (sn2,did_split) = split(*sn2);
                if did_split {
                    (SnailNum::Pair(Box::new(sn1),Box::new(sn2)),true)
                }
                else {
                    (SnailNum::Pair(Box::new(sn1),Box::new(sn2)),false)
                }
            }
        }
    }
}


pub fn add(sn1: SnailNum, sn2: SnailNum) -> SnailNum {
    let mut r = pair(sn1,sn2);
    //println!("After addition :  {}", r);
    loop {
        let (sn,explosion) = explode(r, 0);
        r = sn;
        if explosion.is_some() {
            //println!("After explosion : {}", r);
            continue;
        }
        let (sn,did_split) = split(r);
        r = sn;
        if did_split {
            //println!("After split :     {}", r);
            continue;
        }
        //println!("\n  {}\n+ {}\n= {}", sn1, sn2, r);
        break r;
    }    
}

pub fn magnitude(sn: &SnailNum) -> i64 {
    match sn {
        SnailNum::Regular(x) => *x as i64,
        SnailNum::Pair(sn1, sn2) => 3*magnitude(&*sn1) + 2*magnitude(&*sn2)
    }
}

pub fn part1(numbers: &[SnailNum]) -> i64 {
    let mut iter = numbers.to_owned().into_iter();
    let first = iter.next().unwrap();
    let sum = iter.fold(first, add);
    magnitude(&sum)
}

pub fn part2(numbers: &[SnailNum]) -> i64 {
    let mut max = 0;
    for (i,sn1) in numbers.iter().enumerate() {
        for (j,sn2) in numbers.iter().enumerate() {
            if i != j {
                let sum = add(sn1.clone(), sn2.clone());
                max = std::cmp::max(max, magnitude(&sum));
            }
        }
    }
    max
}

pub fn solve(input: &[u8]) -> (i64,i64) {
    let (_,numbers) = parser::parse(input).unwrap();
    (part1(&numbers),part2(&numbers))
}


#[test]
fn test18_0() {
    let solution = solve(include_bytes!("../inputs/day18.0"));
    assert_eq!(solution, (1384,1384));
}

#[test]
fn test18_1() {
    let solution = solve(include_bytes!("../inputs/day18.1"));
    assert_eq!(solution, (445,90));
}

#[test]
fn test18_2() {
    let solution = solve(include_bytes!("../inputs/day18.2"));
    assert_eq!(solution, (791,115));
}

#[test]
fn test18_3() {
    let solution = solve(include_bytes!("../inputs/day18.3"));
    assert_eq!(solution, (1137,140));
}

#[test]
fn test18_4() {
    let solution = solve(include_bytes!("../inputs/day18.4"));
    assert_eq!(solution, (3488,3805));
}

#[test]
fn test18_5() {
    let solution = solve(include_bytes!("../inputs/day18.5"));
    assert_eq!(solution, (4140,3993));
}

#[test]
fn test18_6() {
    let solution = solve(include_bytes!("../inputs/day18.6"));
    assert_eq!(solution, (3524,4656));
}
