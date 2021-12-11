mod parser  {
    use nom::{
        IResult, multi::*, character::complete::*,
        bytes::complete::*, combinator::*};

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
        let (input, l) = separated_list0(tag(","), i32)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, l))
    }
}

fn fuel_needed1(start_positions: &[i32], position: i32) -> i32 {
    start_positions.iter().fold(0, |sum, x| sum + (position-x).abs())
}

fn fuel_needed2(start_positions: &[i32], position: i32) -> i32 {
    start_positions.iter().fold(0, |sum, x| {
        let distance = (position-x).abs();
        sum + (distance * (distance + 1) / 2)
    })
}

pub fn solve(data: &[u8]) -> (i32,i32) {
    let (_,start_positions) = parser::parse(data).unwrap();

    let p = (0..2000).min_by_key(|&p| fuel_needed1(&start_positions, p)).unwrap();
    let fuel1 = fuel_needed1(&start_positions, p);
    println!("Part 1 - Optimal position is {} for {} fuel", p, fuel1);

    let p = (0..2000).min_by_key(|&p| fuel_needed2(&start_positions, p)).unwrap();
    let fuel2 = fuel_needed2(&start_positions, p);
    println!("Part 2 - Optimal position is {} for {} fuel", p, fuel2);

    (fuel1, fuel2)
}

#[test]
fn test7_0() {
    let solution = solve(include_bytes!("../inputs/day7.0"));
    assert_eq!(solution, (37,168));
}

#[test]
fn test7_1() {
    let solution = solve(include_bytes!("../inputs/day7.1"));
    assert_eq!(solution, (359648,100727924));
}
