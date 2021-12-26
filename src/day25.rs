use array2d::Array2D;

#[derive(Clone)]
pub enum Cell { Horizontal, Vertical, Empty }

pub struct Map(Array2D<Cell>);

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Horizontal => write!(f, ">"),
            Cell::Vertical => write!(f, "v"),
            Cell::Empty => write!(f, ".")
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row_iter in self.0.rows_iter() {
            for pixel in row_iter {
                write!(f, "{}", pixel)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

mod parser {
    use nom::{IResult, character::complete::*, sequence::*, multi::*, combinator::*};
    use super::*;

    pub fn cell(input: &[u8]) -> IResult<&[u8], Cell> {
        let (input,c) = satisfy(|c| c == '.' || c == '>' || c == 'v')(input)?;
        let cell = match c {
            '>' => Cell::Horizontal,
            'v' => Cell::Vertical,
            '.' => Cell::Empty,
            _ => panic!()         
        };
        Ok((input, cell))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Map> {
        let (input, rows) =
            terminated(
                separated_list1(multispace1, many1(cell)),
                all_consuming(multispace0))(input)?;
        Ok((input, Map(Array2D::from_rows(&rows))))
    }
}

fn hstep(map: &Map) -> (Map,bool) {
    let (n_rows,n_cols) = (map.0.num_rows(), map.0.num_columns());
    let mut result = Array2D::filled_with(Cell::Empty, n_rows, n_cols);
    let mut movement = false;

    for i in 0..n_rows {
        for j in 0..n_cols {
            match map.0.get(i,j) {
                Some(Cell::Horizontal) => {
                    let next = (j + 1) % n_cols;
                    if let Some(Cell::Empty) = map.0.get(i, next) {
                        result.set(i, next, Cell::Horizontal).unwrap();
                        movement = true;
                    }
                    else {
                        result.set(i, j, Cell::Horizontal).unwrap();
                    }
                }
                Some(Cell::Vertical) => {
                    result.set(i, j, Cell::Vertical).unwrap();
                }
                _ => ()
            }
        }
    }

    (Map(result), movement)
}


fn vstep(map: &Map) -> (Map,bool) {
    let (n_rows,n_cols) = (map.0.num_rows(), map.0.num_columns());
    let mut result = Array2D::filled_with(Cell::Empty, n_rows, n_cols);
    let mut movement = false;

    for i in 0..n_rows {
        for j in 0..n_cols {
            match map.0.get(i,j) {
                Some(Cell::Vertical) => {
                    let next = (i + 1) % n_rows;
                    if let Some(Cell::Empty) = map.0.get(next, j) {
                        result.set(next, j, Cell::Vertical).unwrap();
                        movement = true;
                    }
                    else {
                        result.set(i, j, Cell::Vertical).unwrap();
                    }
                }
                Some(Cell::Horizontal) => {
                    result.set(i, j, Cell::Horizontal).unwrap();
                }
                _ => ()
            }
        }
    }

    (Map(result), movement)
}


pub fn solve(input: &[u8]) -> (i32,i32) {
    let (_,mut map) = parser::parse(input).unwrap();
    
    println!("Initial map\n{}", map);

    let mut step = 0;
    let mut movement = true;
    while movement {
        step += 1;
        let (rmap,rmovement) = hstep(&map);
        map = rmap;
        movement = rmovement;
        let (rmap,rmovement) = vstep(&map);
        map = rmap;
        movement = movement || rmovement;
        println!("After {} steps\n{}", step, map);
    }

    (step, 0)
}

#[test]
fn test25_0() {
    let solution = solve(include_bytes!("../inputs/day25.0"));
    assert_eq!(solution, (58,0));
}

#[test]
fn test25_1() {
    let solution = solve(include_bytes!("../inputs/day25.1"));
    assert_eq!(solution, (432,0));
}
