use array2d::Array2D;
pub type Input = Array2D<i32>;

mod parser  {
    use nom::{IResult, multi::*, character::complete::*, combinator::*};

    fn line(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
        let (input, slice) = digit1(input)?;
        let vec = slice.iter().map(|&d| (d as i32) - ('0' as i32)).collect();
        Ok((input, vec))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], super::Input> {
        let (input, l) = separated_list1(multispace1, line)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, super::Input::from_rows(l.as_slice())))
    }
}

const ADJACENT : [(i32,i32) ; 4] = [(-1,0),(0,-1),(1,0),(0,1)];

fn part1(input : &Input) -> i32 {
    let mut risk = 0;

    for (i,row_iter) in input.rows_iter().enumerate() {
        for (j,element) in row_iter.enumerate() {
            let minimum = ADJACENT.iter().all(|(a,b)| {
                let x = usize::try_from(i as i32 + a);
                let y = usize::try_from(j as i32 + b);
                if let (Ok(x),Ok(y)) = (x,y) {
                    if let Some(v) = input.get(x, y) {
                        return element < v
                    }
                };
                true
            });

            if minimum {
                println!("minimum at {},{}: {}", i, j, element);
                risk += 1 + element;
            }
        }
    }

    println!("Sum of risk levels: {}", risk);
    risk
}

fn dfs(input : &Input, marks : &mut Array2D<bool>, i : usize, j : usize) -> u32 {
    match (marks.get(i, j), input.get(i,j)) {
        (Some(false), Some(&v)) => if v >= 9 { return 0; }
        _ => { return 0; }
    }
    let _ = marks.set(i,j,true);

    let mut size = 1;
    for (a,b) in ADJACENT {
        let x = usize::try_from(i as i32 + a);
        let y = usize::try_from(j as i32 + b);
        if let (Ok(x),Ok(y)) = (x,y) {
            size += dfs(input, marks, x, y)
        }
    }
    size
}

fn part2(input : &Input) -> u32 {
    let mut marks : Array2D<bool> =
        Array2D::filled_with(false, input.num_rows(), input.num_columns());
    let mut sizes = Vec::new();

    for (i,row_iter) in input.rows_iter().enumerate() {
        for (j,_element) in row_iter.enumerate() {
            let size = dfs(input, &mut marks, i, j);
            if size > 0 {
                sizes.push(size);
            }
        }
    }

    sizes.sort();
    let product = sizes.drain((sizes.len()-3)..).fold(1, |acc,n| acc * n);
    println!("Product of 3 biggest sizes: {}", product);
    product
}

pub fn solve(data: &[u8]) -> (i32,u32) {
    let (_,input) = parser::parse(data).unwrap();
    (part1(&input), part2(&input))
}

#[test]
fn test9_0() {
    let solution = solve(include_bytes!("../inputs/day9.0"));
    assert_eq!(solution, (15,1134));
}

#[test]
fn test9_1() {
    let solution = solve(include_bytes!("../inputs/day9.1"));
    assert_eq!(solution, (631,821560));
}
