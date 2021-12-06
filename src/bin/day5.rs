use nom::{
    combinator::{iterator,opt},
    IResult,
    character::complete::{u32,line_ending},
    bytes::complete::tag,
    sequence::tuple
};


#[derive(Debug,PartialEq)]
pub struct Line {
  pub x1:      u32,
  pub y1:      u32,
  pub x2:      u32,
  pub y2:      u32,
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Line> {
  let (input, (x1, _, y1, _ , x2, _, y2, _)) = tuple((
        u32, tag(","), u32,
        tag(" -> "),
        u32, tag(","), u32,
        opt(line_ending)))(input)?;
  Ok((input, Line { x1, y1, x2, y2 }))
}

fn parse(input: &[u8]) -> Vec<Line> {
    iterator(input, parse_line).collect()
}

fn print_grid(grid : &Vec<Vec<u32>>) {
    for row in grid {
        for cell in row {
            print!("{}",cell);
        }
        println!("");
    }
}

fn draw_line(line : &Line, grid : &mut Vec<Vec<u32>>) {
    let Line { x1, y1, x2, y2 } = *line;
    if x1 == x2 {
        for y in if y1 < y2 { y1..=y2 } else { y2..=y1 } {
            grid[y as usize][x1 as usize] += 1;
        }
    }
    else if y1 == y2 {
        for x in if x1 < x2 { x1..=x2 } else { x2..=x1 } {
            grid[y1 as usize][x as usize] += 1;
        }
    } else {
        if x1 < x2 {
            let mut y = y1 as i32;
            for x in x1..=x2 {
                grid[y as usize][x as usize] += 1;
                y += if y1 < y2 { 1 } else { -1 };
            }
        } else {
            let mut y = y2 as i32;
            for x in x2..=x1 {
                grid[y as usize][x as usize] += 1;
                y += if y2 < y1 { 1 } else { -1 };
            }
        }
    }
}

fn count_overlaps(grid : &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for row in grid {
        for &cell in row {
            if cell > 1 {
                count += 1;
            }
        }
    }
    count
}

fn solve(input : &Vec<Line>) -> u32 {
    let mut grid = vec![vec![0u32; 1000]; 1000];
    for line in input {
        draw_line(line, &mut grid);
    }
    //print_grid(&grid);
    count_overlaps(&grid)
}

fn main() {

}

#[test]
fn test0() {
  let data = include_bytes!("../../inputs/day5.0");
  let input = parse(data);
  println!("ovelapping points: {}", solve(&input));
}

#[test]
fn test1() {
    let data = include_bytes!("../../inputs/day5.1");
    let input = parse(data);
    println!("ovelapping points: {}", solve(&input));
}
