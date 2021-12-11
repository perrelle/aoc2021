pub struct Line {
    pub x1:      u32,
    pub y1:      u32,
    pub x2:      u32,
    pub y2:      u32,
}

pub type Grid = array2d::Array2D<u32>;

mod parser {
    use nom::{
        IResult, multi::*, character::complete::*, bytes::complete::*,
        sequence::*, combinator::*};

    fn line(input: &[u8]) -> IResult<&[u8], super::Line> {
        let (input, (x1, _, y1, _ , x2, _, y2)) = tuple((
                u32, tag(","), u32,
                tag(" -> "),
                u32, tag(","), u32))(input)?;
        Ok((input, super::Line { x1, y1, x2, y2 }))
    }
    
    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<super::Line>> {
        let (input, l) = separated_list1(multispace1, line)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, l))
    }
}

pub fn print_grid(grid : &Grid) {
    for row_iter in grid.rows_iter() {
        for element in row_iter {
            print!("{}",element);
        }
        println!("");
    }
}

fn draw_point(grid : &mut Grid, x : u32, y : u32) {
    if let Some(element) = grid.get_mut(y as usize, x as usize) {
        *element += 1;
    }
}

fn draw_line(grid : &mut Grid, line : &Line, draw_diag: bool) {
    let Line { x1, y1, x2, y2 } = *line;
    if x1 == x2 {
        for y in if y1 < y2 { y1..=y2 } else { y2..=y1 } {
            draw_point(grid, x1, y);
        }
    }
    else if y1 == y2 {
        for x in if x1 < x2 { x1..=x2 } else { x2..=x1 } {
            draw_point(grid, x, y1);
        }
    } else if draw_diag {
        if x1 < x2 {
            let mut y = y1 as i32;
            for x in x1..=x2 {
                draw_point(grid, x, y as u32);
                y += if y1 < y2 { 1 } else { -1 };
            }
        } else {
            let mut y = y2 as i32;
            for x in x2..=x1 {
                draw_point(grid, x, y as u32);
                y += if y2 < y1 { 1 } else { -1 };
            }
        }
    }
}

fn count_overlaps(grid : &Grid) -> u32 {
    let mut count = 0;
    for row_iter in grid.rows_iter() {
        for &element in row_iter {
            if element > 1 {
                count += 1;
            }
        }
    }
    count
}

pub fn solve_part(part : i32, lines : &Vec<Line>) -> u32 {
    use std::cmp::max;

    let max_coord = |l : &&Line| max(max(max(l.x1, l.y1), l.x2), l.y2);
    let size = max_coord(&lines.iter().max_by_key(max_coord).unwrap()) as usize + 1;
    let mut grid = Grid::filled_with(0, size, size);

    for line in lines {
        draw_line(&mut grid, &line, part == 2);
    }

    if size < 40 {
        print_grid(&grid);
    }

    let overlaps = count_overlaps(&grid);
    println!("Part {} - ovelapping points: {}", part, overlaps);
    overlaps
}

pub fn solve(input : &[u8]) -> (u32,u32) {
    let (_,lines) = parser::parse(input).unwrap();
    (solve_part(1, &lines), solve_part(2, &lines))
}

#[test]
fn test5_0() {
  let solution = solve(include_bytes!("../inputs/day5.0"));
  assert_eq!(solution, (5,12));
}

#[test]
fn test5_1() {
    let solution = solve(include_bytes!("../inputs/day5.1"));
    assert_eq!(solution, (6283,18864));
}
