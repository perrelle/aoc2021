mod parser {
    use nom::{
        IResult, multi::*, character::complete::*,
        bytes::complete::*, combinator::*};

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<u32>> {
        let (input, l) = separated_list0(tag(","), u32)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, l))
    }
}

pub fn naive_solve(input : &[u8], iterations : u32) -> u32 {
    let (_,starting_timers) = parser::parse(input).unwrap();
    let mut timers = starting_timers;

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

        new_timers.extend(vec![8; born]);

        timers = new_timers;
        println!("After {} days: ({}) {:?}", i, timers.len(), timers);
    }

    timers.len() as u32
}

fn simulate(starting_timers : &[u32], iterations : u32) -> u64 {
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

pub fn solve(input : &[u8]) -> (u64,u64) {
    let (_,starting_timers) = parser::parse(input).unwrap();

    let total80 = simulate(&starting_timers, 80);
    println!("After 80 days, there would be a total of {} fish", total80);

    let total256 = simulate(&starting_timers, 256);
    println!("After 256 days, there would be a total of {} fish", total256);

    (total80,total256)
}

#[test]
fn test_naive() {
    let solution = naive_solve(include_bytes!("../inputs/day6.0"), 18);
    assert_eq!(solution, 26);
}

#[test]
fn test6_0() {
    let solution = solve(include_bytes!("../inputs/day6.0"));
    assert_eq!(solution, (5934,26984457539));
}

#[test]
fn test6_1() {
    let solution = solve(include_bytes!("../inputs/day6.1"));
    assert_eq!(solution, (353079,1605400130036));
}
