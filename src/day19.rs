use std::collections::HashSet;
use crate::algebra::*;

pub type Scanner = (i32,Vec<Vector>);

mod parser {
    use nom::{
        IResult, character::complete::*, bytes::complete::*,
        multi::*, combinator::*, sequence::*};
    use super::*;

    pub fn header(input: &[u8]) -> IResult<&[u8], i32> {
        let (input,(_,n,_,_)) =
            tuple((tag("--- scanner "), i32, tag(" ---"), multispace1))(input)?;
        Ok((input, n))
    }

    fn point(input: &[u8]) -> IResult<&[u8], Vector> {
        let (input,(x,_,y,_,z)) =
            tuple((i32, tag(","), i32, tag(","), i32))(input)?;
        Ok((input, Vector { x, y, z }))
    }

    fn section(input: &[u8]) -> IResult<&[u8], Scanner> {
        tuple((header, separated_list1(multispace1, point)))(input)
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Scanner>> {
        let (input, scanners) = separated_list1(multispace1, section)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, scanners))
    }
}


fn find_matching(scanner1: &[Vector], scanner2: &[Vector]) -> Option<AffineMap> {
    for p1 in scanner1 {
        for p2 in scanner2 {
            for x in Vector::AXES {
                for y in Vector::AXES {
                    let z = &x ^ &y;
                    if z.is_zero() {
                        continue;
                    }
                    let linear = LinearMap {
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone()
                    };
                    let translation = p1 - linear.apply(p2);
                    let transformation = AffineMap {linear, translation};
                    let mut count = 0;
                    
                    for p2 in scanner2 {
                        let p2_transformed = transformation.apply(p2);
                        if p2_transformed.norm_inf() <= 1000 {
                            if scanner1.contains(&p2_transformed) {
                                count += 1;
                            }
                            else {
                                break;
                            }
                        }
                    }
                    if count >= 12 {
                        return Some(transformation);
                    }
                }
            }
        }
    }
    None
}


pub fn solve(input: &[u8]) -> (usize,i32) {
    let (_,scanners) = parser::parse(input).unwrap();

    let mut pending : Vec<(&Scanner, AffineMap)> = Vec::new();
    let mut remaining : Vec<&Scanner> = scanners.iter().collect();
    let mut points : HashSet<Vector> = HashSet::new();
    let mut positions : Vec<Vector> = Vec::new();

    pending.push((&scanners[0], AffineMap::ID));

    while let Some((scanner1 @ (i1,s1),f1)) = pending.pop() {
        positions.push(f1.translation.clone());
        remaining.retain(|scanner2|
            scanner2.0 != scanner1.0 &&
            pending.iter().all(|(scanner1,_)| scanner2.0 != scanner1.0));
        for p in s1 {
            let _ = points.insert(f1.apply(p));
        }

        for scanner2 @ (i2,s2) in &remaining {
            if let Some(f2) = find_matching(s1,s2) {
                println!("Matching between scanner {} and {}", i1, i2);
                pending.push((scanner2, AffineMap::compose(&f1,&f2)));
            }
        }
    }

    if points.len() < 100 {
        println!("--- Points at the end, relative to scanner 0 ---");
        let mut sorted_points : Vec<&Vector> = points.iter().collect();
        sorted_points.sort();
        for p in sorted_points {
            println!("{}", p);
        }
    }

    println!("In total, there are {} beacons", points.len());

    let mut max = 0;
    for p1 in &positions {
        for p2 in &positions {
            max = i32::max(max, (p1 - p2).norm1());
        }
    }
    println!("Max distance is {}", max);

    (points.len(),max)
}


#[test]
fn test19_0() {
    let solution = solve(include_bytes!("../inputs/day19.0"));
    assert_eq!(solution, (79,3621));
}

#[test]
fn test19_1() {
    let solution = solve(include_bytes!("../inputs/day19.1"));
    assert_eq!(solution, (451,13184));
}
