use array2d::Array2D;
use std::cmp::max;

#[derive(Clone, Copy, Debug)]
pub enum Axis { X, Y }

pub type Grid = Array2D<bool>;
pub type Point = (u32,u32);
pub type Fold = (Axis, u32);
pub type Input = (Vec<Point>, Vec<Fold>);

mod parser  {
    use nom::{
        IResult, multi::*, character::complete::*, bytes::complete::*,
        sequence::*, combinator::*};
    use super::*;

    fn point(input: &[u8]) -> IResult<&[u8], Point> {
        let (input, (x,_,y, _)) =
            tuple((u32, tag(","),  u32, multispace1))(input)?;
        Ok((input, (x,y)))
    }

    fn axis(input: &[u8]) -> IResult<&[u8], Axis> {
        let (input, s) =  one_of("xy")(input)?;
        let axis = match s {
            'x' => Axis::X,
            'y' => Axis::Y,
            _ => panic!()
        };
        Ok((input, axis))
    }

    fn fold(input: &[u8]) -> IResult<&[u8], Fold> {
        let (input, (_,_,axis,_,pos)) =
            tuple((tag("fold along"), space1, axis, tag("="), u32))(input)?;
        Ok((input, (axis,pos)))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Input> {
        let (input, points) = many1(point)(input)?;
        println!("{:?}", points);
        let (input, folds) = separated_list1(multispace1, fold)(input)?;
        println!("{:?}", folds);
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, (points, folds)))
    }
}

pub fn print_grid(grid : &Grid) {
    for row_iter in grid.rows_iter() {
        for &element in row_iter {
            if element {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn draw_grid(points: &[Point]) -> Grid {
    let width = points.iter().max_by_key(|&(x,_)| x).unwrap().0 as usize + 1;
    let height = points.iter().max_by_key(|&(_,y)| y).unwrap().1 as usize + 1;
    let mut grid = Array2D::filled_with(false, height, width);

    for &(x,y) in points {
        let _ = grid.set(y as usize, x as usize, true);
    }

    grid
}

fn fold(grid: &Grid, &(axis,pos) : &Fold) -> Grid {
    let height = grid.num_rows();
    let width = grid.num_columns();

    match axis {
        Axis::X => {
            let new_width = max(pos as usize, width - 1 - (pos as usize));
            let mut new_grid = Array2D::filled_with(false, height, new_width);
            let translation = max(0, new_width as i32 - pos as i32) as usize;

            for (i,row_iter) in grid.rows_iter().enumerate() {
                for (j,&element) in row_iter.enumerate() {
                    if element {
                        if j < pos as usize {
                            let _ = new_grid.set(i, j + translation, true);
                        }
                        else if j > pos as usize {
                            let _ = new_grid.set(i, 2 * (pos as usize) - j + translation, true);
                        }
                    }
                }
            }

            new_grid
        },
        Axis::Y => {
            let new_height = max(pos as usize, height - 1 - (pos as usize));
            let mut new_grid = Array2D::filled_with(false, new_height, width);
            let translation = max(0, new_height as i32 - pos as i32) as usize;

            for (i,row_iter) in grid.rows_iter().enumerate() {
                for (j,&element) in row_iter.enumerate() {
                    if element {
                        if i < pos as usize {
                            let _ = new_grid.set(i + translation, j, true);
                        }
                        else if i > pos as usize {
                            let _ = new_grid.set(2 * (pos as usize) - i + translation, j, true);
                        }
                    }
                }
            }

            new_grid
        }
    }
}

fn count(grid: &Grid) -> u32 {
    let mut count = 0;
    for row_iter in grid.rows_iter() {
        for &element in row_iter {
            if element {
                count += 1;
            }
        }
    }
    count
}

pub fn solve(input: &[u8]) -> (u32,u32) {
    let (_,(points,folds)) = parser::parse(input).unwrap();
    let mut grid = draw_grid(&points);
    let mut solution1 = 0;
    if grid.num_elements() < 1000 {
        print_grid(&grid);
    }

    for (s,f) in folds.iter().enumerate() {
        grid = fold(&grid, f);
        if s == 0 {
            solution1 = count(&grid);
        }
        if grid.num_elements() < 1000 {
            print_grid(&grid);
        }
    }

    let solution2 = count(&grid);
    (solution1, solution2)
}

#[test]
fn test13_0() {
    let solution = solve(include_bytes!("../inputs/day13.0"));
    assert_eq!(solution, (17,16));
}

#[test]
fn test13_1() {
    let solution = solve(include_bytes!("../inputs/day13.1"));
    assert_eq!(solution, (763,103));
}
