pub type Grid = [[u32 ; 5] ; 5];

mod parser  {
    use nom::{IResult, multi::*, character::complete::*, combinator::*};

    pub fn digit(input: &[u8]) -> IResult<&[u8], u32> {
        let (input,c) = satisfy(|c| c == '0' || c == '1')(input)?;
        Ok((input,c.to_digit(2).unwrap()))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Vec<u32>>> {
        let (input, numbers) = separated_list1(multispace1, many1(digit))(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, numbers))
    }
}


fn convert_rate(v : &Vec<u32>) -> u32 {
    v.iter().fold(0, |rate, &x| { 2 * rate + x })
}

fn compute_power_rate(data : &Vec<Vec<u32>>) -> u32 {
    let mut counts : Option<Vec<u32>> = None;

    for number in data {
        match &mut counts {
            None => counts = Some (number.clone()),
            Some(counts) => {
                for (i, b) in number.iter().enumerate() {
                    counts[i] += b;
                }
            }
        };
    }

    let n = data.len() as u32;
    if let Some(counts) = counts {
        let (g,e) =
            counts
                .into_iter()
                .map(|x| if x > n - x { (1,0) } else { (0,1) })
                .unzip();
        let gamma = convert_rate(&g);
        let epsilon = convert_rate(&e);
        let power = gamma * epsilon;
        println!("gamma: {} epsilon: {}, consumption: {}", gamma, epsilon, power);
        power
    }
    else {
        0
    }
}

fn compute_rate(majority: bool, data : &Vec<Vec<u32>>) -> &Vec<u32> {
    let mut set : Vec<&Vec<u32>> = data.iter().collect();
    let mut index = 0;

    while set.len() > 1 {
        let total : u32 = set.len() as u32;
        let count = set.iter().fold(0, |n, v| n + v[index]);
        let bit = if majority == (count >= total - count) { 1 } else { 0 };
        set = set.into_iter().filter(|v| v[index] == bit).collect();
        index += 1;
    } 

    set[0]
}

fn compute_ls_rate(data : &Vec<Vec<u32>>) -> u32 {
    let oxygen = convert_rate(&compute_rate(true, &data));
    let dhmo = convert_rate(&compute_rate(false, &data));
    let life_support = oxygen * dhmo;
    println!("Ogygen: {}, CO²: {}, Life support rating: {}", oxygen, dhmo, life_support);
    life_support
}

pub fn solve(input: &[u8]) -> (u32,u32) {
    let (_,data) = parser::parse(input).unwrap();
    let power = compute_power_rate(&data);
    let life_support = compute_ls_rate(&data);
    (power,life_support)
}

#[test]
fn test3_0() {
    let solution = solve(include_bytes!("../inputs/day3.0"));
    assert_eq!(solution, (198,230));
}

#[test]
fn test3_1() {
    let solution = solve(include_bytes!("../inputs/day3.1"));
    assert_eq!(solution, (693486,3379326));
}
