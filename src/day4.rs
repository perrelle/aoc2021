pub type Grid = [[u32 ; 5] ; 5];

mod parser  {
    use nom::{
        IResult, multi::*, character::complete::*, bytes::complete::*,
        sequence::*, combinator::*};

    pub fn line(input: &[u8]) -> IResult<&[u8], [u32 ; 5]> {
        let (input,line) = terminated(separated_list1(space1, u32), multispace0)(input)?;
        Ok((input, line.try_into().unwrap()))
    }

    pub fn grid(input: &[u8]) -> IResult<&[u8], super::Grid> {
        let (input,grid) = count(line, 5)(input)?;
        Ok((input, grid.try_into().unwrap()))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], (Vec<u32>,Vec<super::Grid>)> {
        let (input,numbers) = terminated(separated_list1(tag(","), u32), multispace1)(input)?;
        let (input, grids) = many1(grid)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, (numbers,grids)))
    }
}


fn check_bingo(grid: &[[u32 ; 5] ; 5], drawn_numbers : &Vec<bool>) -> bool {
    (0..5).any(|i| {
        (0..5).all(|j| {
            drawn_numbers[grid[i][j] as usize]
        }) ||
        (0..5).all(|j| {
            drawn_numbers[grid[j][i] as usize]
        })
    })
}

fn score(grid: &[[u32 ; 5] ; 5], drawn_numbers : &Vec<bool>, final_number: u32) -> i32 {
    let sum =
        (0..5).fold(0, |sum, i| {
            (0..5).fold(sum, |sum, j| {
                if drawn_numbers[grid[i][j] as usize] {
                    sum
                } else {
                    sum + grid[i][j]
                }
            })
        });
    (sum * final_number) as i32
}

fn part1(numbers : &Vec<u32>, grids: &Vec<[[u32 ; 5] ; 5]>) -> i32 {
    let mut drawn_numbers : Vec<bool> = Vec::new();
    drawn_numbers.resize(numbers.len(), false);

    for &n in numbers {
        drawn_numbers[n as usize] = true;
        for grid in grids {
            if check_bingo(&grid, &drawn_numbers) {
                return score(&grid, &drawn_numbers, n);
            }
        }
    }
    -1
}

fn part2(numbers : &Vec<u32>, grids: &Vec<[[u32 ; 5] ; 5]>) -> i32 {
    let mut drawn_numbers : Vec<bool> = Vec::new();
    drawn_numbers.resize(numbers.len(), false);
    let mut remaining_grids : Vec<&[[u32 ; 5] ; 5]> = grids.iter().collect();

    for &n in numbers {
        drawn_numbers[n as usize] = true;
        match remaining_grids.as_slice() {
            [grid] => {
                if check_bingo(&grid, &drawn_numbers) {
                    return score(&grid, &drawn_numbers, n);
                }
            },
            _ => {
                remaining_grids = remaining_grids.into_iter().filter(|grid|
                    !check_bingo(&grid, &drawn_numbers)).collect();
            }
        }
    }
    -1
}

pub fn solve(data: &[u8]) -> (i32,i32) {
    let (_,(numbers, grids)) = parser::parse(data).unwrap();
    let score1 = part1(&numbers, &grids);
    println!("Part 1 - final score is {}", score1);
    let score2 = part2(&numbers, &grids);
    println!("Part 2 - final score is {}", score2);
    (score1,score2)
}

#[test]
fn test4_0() {
    let solution = solve(include_bytes!("../inputs/day4.0"));
    assert_eq!(solution, (4512,1924));
}

#[test]
fn test4_1() {
    let solution = solve(include_bytes!("../inputs/day4.1"));
    assert_eq!(solution, (58838,6256));
}
