use nom::{
    multi::separated_list0,
    IResult, Err, error,
    bytes::complete::tag,
    character::complete::{i32, multispace0},
    combinator::all_consuming,
};

fn parse(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    let (input, l) = separated_list0(tag(","), i32)(input)?;
    let (input, _) = all_consuming(multispace0)(input)?;
    Ok((input, l))
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

fn solve(data: &[u8]) -> Result<i32, Err<error::Error<&[u8]>>> {
    let (_,start_positions) = parse(data)?;
    let p = (0..2000).min_by_key(|&p| fuel_needed1(&start_positions, p)).unwrap();
    let fuel = fuel_needed1(&start_positions, p);
    println!("Part 1 - Optimal position is {} for {} fuel", p, fuel);
    let p = (0..2000).min_by_key(|&p| fuel_needed2(&start_positions, p)).unwrap();
    let fuel = fuel_needed2(&start_positions, p);
    println!("Part 2 - Optimal position is {} for {} fuel", p, fuel);
    Ok(fuel)
}

fn main() {

}

#[test]
fn test0() {
    solve(include_bytes!("../../inputs/day7.0"));
}

#[test]
fn test1() {
    solve(include_bytes!("../../inputs/day7.1"));
}
