use nom::{
    multi::separated_list0,
    IResult,
    bytes::complete::tag,
    character::complete::{u32, multispace0},
    combinator::all_consuming
};

fn parse(input: &[u8]) -> IResult<&[u8], Vec<u32>> {
    let (input, l) = separated_list0(tag(","), u32)(input)?;
    let (input, _) = all_consuming(multispace0)(input)?;
    Ok((input, l))
}

fn naive_solve(starting_timers : &Vec<u32>, iterations : u32) -> u32 {
    let mut timers = starting_timers.clone();

    println!("Initial state: {:?}", timers);

    for i in 1..=iterations {
        let mut new_timers : Vec<u32> = Vec::new();
        let mut born = 0;

        for t in timers {
            if t > 0 {
                new_timers.push(t - 1);
            }
            else {
                new_timers.push(6);
                born += 1;
            }
        }

        for _ in 0..born {
            new_timers.push(8);
        }

        timers = new_timers;
        println!("After {} days: ({}) {:?}", i, timers.len(), timers);
    }

    timers.len() as u32
}

fn solve(starting_timers : &Vec<u32>, iterations : u32) -> u64 {
    let mut timers = [0u64 ; 9];

    for &t in starting_timers {
        timers[t as usize] += 1;
    }

    for _ in 1..=iterations {
        let mut new_timers = [0u64 ; 9];

        for (t,x) in timers.iter().enumerate() {
            if t > 0 {
                new_timers[t-1] += x;
            }
            else {
                new_timers[6] += x;
                new_timers[8] += x;
            }
        }

        timers = new_timers;
    }

    timers.iter().sum()
}

fn main() {

}

fn test(data : &[u8]) {
    let (_,input) = parse(data).unwrap();
    let total = solve(&input, 80);
    println!("After 80 days, there would be a total of {} fish", total);
    let total = solve(&input, 256);
    println!("After 256 days, there would be a total of {} fish", total);
}

#[test]
fn test_naive() {
    let data = include_bytes!("../../inputs/day6.0");
    let (_,input) = parse(data).unwrap();
    naive_solve(&input, 18);
}

#[test]
fn test0() {
    test(include_bytes!("../../inputs/day6.0"));
}

#[test]
fn test1() {
    test(include_bytes!("../../inputs/day6.1"));
}
