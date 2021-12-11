mod parser  {
    use nom::{IResult, multi::*, character::complete::*, combinator::*};

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
        let (input, l) = separated_list1(multispace1, i32)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, l))
    }
}

pub fn solve(input: &[u8]) -> (i32,i32) {
    let (_,numbers) = parser::parse(input).unwrap();

    let mut last = None;
    let mut penultimate = None;
    let mut lastsum = None;
    let mut number_increases = 0;
    let mut sum_increases = 0;

    for n in numbers {
        if let Some (m) = last {
            if n > m {
                number_increases += 1;
            }
        }

        if let (Some(m),Some(p)) = (last,penultimate) {
            let sum = n+m+p;
            match lastsum {
                None => (),
                Some(s) =>
                    if sum > s {
                        sum_increases += 1;
                    }
            }
            lastsum = Some(sum);
        }

        penultimate = last;
        last = Some(n);
    }
    
    println!("{} measurements are larger than the previous measurement",
        number_increases);
    println!("{} sums are larger than than the previous sum",
    sum_increases);
    (number_increases, sum_increases)
}

#[test]
fn test1_0() {
    let solution = solve(include_bytes!("../inputs/day1.0"));
    assert_eq!(solution, (7,5));
}

#[test]
fn test1_1() {
    let solution = solve(include_bytes!("../inputs/day1.1"));
    assert_eq!(solution, (1722,1748));
}
