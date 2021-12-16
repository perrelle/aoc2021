use std::cmp::Ordering;
use std::collections::BinaryHeap;
use array2d::Array2D;

mod parser {
    use nom::{IResult, character::complete::*, multi::*, combinator::*};
    use super::*;

    pub fn digit(input: &[u8]) -> IResult<&[u8], u32> {
        let (input,c) = satisfy(|c| ('0'..='9').contains(&c))(input)?;
        Ok((input, c as u32 - '0' as u32))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Array2D<u32>> {
        let (input, rows) = separated_list1(multispace1, many1(digit))(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, Array2D::from_rows(&rows)))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    pos: (usize,usize)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const ADJACENT : [(i32,i32) ; 4] = [(-1,0),(0,-1),(1,0),(0,1)];

pub fn shortest_path(grid: &Array2D<u32>, part2 : bool) -> u32 {
    let subheight = grid.num_rows();
    let subwidth = grid.num_columns();
    let (width, height) = 
        if part2 {
            (subwidth * 5, subheight * 5)
        }
        else {
            (subwidth, subheight)
        };

    let risk = |x: usize, y: usize| -> Option<u32> {
        let r =
            grid.get(y % subheight, x % subwidth)? +
            (y / subheight + x / subwidth) as u32;
        if r <= 9 {
            Some(r)
        }
        else {
            Some(r - 9)
        }
    };

    let mut marks =
        Array2D::filled_with(false, height, width);

    let mut next_vertices = BinaryHeap::new();
    next_vertices.push(State { cost: 0, pos: (0,0) });
    while let Some(State { cost, pos: (x,y) }) = next_vertices.pop() {
        if y == height - 1 && x == width - 1 {
            println!("shortest path with cost: {}", cost);
            return cost;
        }

        if let Some(false) = marks.get(y, x) {
            let _ = marks.set(y,x,true);

            for (i,j) in ADJACENT {
                let x = x as i32 + i;
                let y = y as i32 + j;
                if let (Ok(x),Ok(y)) = (usize::try_from(x),usize::try_from(y)) {
                    if y < height && x < width {
                        if let Some(v) = risk(x, y) {
                            next_vertices.push(State {
                                cost: cost+v,
                                pos: (x as usize,y as usize)});
                        }
                    }
                }
            }
        }
    }
    panic!();
}

pub fn solve(input: &[u8]) -> (u32,u32) {
    let (_,grid) = parser::parse(input).unwrap();
    (shortest_path(&grid,false),shortest_path(&grid,true))
}


#[test]
fn test15_0() {
    let solution = solve(include_bytes!("../inputs/day15.0"));
    assert_eq!(solution, (40,315));
}

#[test]
fn test15_1() {
    let solution = solve(include_bytes!("../inputs/day15.1"));
    assert_eq!(solution, (595,2914));
}
