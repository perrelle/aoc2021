mod parser  {
    use nom::{
        multi::separated_list1,
        IResult,
        character::complete::{digit1, multispace0, multispace1},
        combinator::all_consuming,
    };
    use array2d::Array2D;
    pub type Input = Array2D<i32>;

    fn line(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
        let (input, slice) = digit1(input)?;
        let vec = slice.iter().map(|&d| (d as i32) - ('0' as i32)).collect();
        Ok((input, vec))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Input> {
        let (input, l) = separated_list1(multispace1, line)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, Array2D::from_rows(l.as_slice())))
    }
}

use array2d::Array2D;
const adjacent : [(i32,i32) ; 4] = [(-1,0),(0,-1),(1,0),(0,1)];

fn part1(input : &parser::Input) {
    let mut risk = 0;

    for (i,row_iter) in input.rows_iter().enumerate() {
        for (j,element) in row_iter.enumerate() {
            let minimum = adjacent.iter().all(|(a,b)| {
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
}

fn dfs(input : &parser::Input, marks : &mut Array2D<bool>, i : usize, j : usize) -> u32 {
    match (marks.get(i, j), input.get(i,j)) {
        (Some(false), Some(&v)) => if v >= 9 { return 0; }
        _ => { return 0; }
    }
    marks.set(i,j,true);

    let mut size = 1;
    for (a,b) in adjacent {
        let x = usize::try_from(i as i32 + a);
        let y = usize::try_from(j as i32 + b);
        if let (Ok(x),Ok(y)) = (x,y) {
            size += dfs(input, marks, x, y)
        }
    }
    size
}

fn part2(input : &parser::Input) {
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
}

fn solve(data: &[u8]) {
    let (_,input) = parser::parse(data).unwrap();
    part1(&input);
    part2(&input);
}

fn main() {

}

#[test]
fn test0() {
    solve(include_bytes!("../../inputs/day9.0"));
}

#[test]
fn test1() {
    solve(include_bytes!("../../inputs/day9.1"));
}
