use array2d::Array2D;
pub type Model = Array2D<u32>;

mod parser  {
    use nom::{IResult, multi::*, character::complete::*, combinator::*};

    fn line(input: &[u8]) -> IResult<&[u8], Vec<u32>> {
        let (input, slice) = digit1(input)?;
        let vec = slice.iter().map(|&d| (d as u32) - ('0' as u32)).collect();
        Ok((input, vec))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], super::Model> {
        let (input, l) = separated_list1(multispace1, line)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, super::Model::from_rows(l.as_slice())))
    }
}

pub fn print_model(model: &Model) {
    for row_iter in model.rows_iter() {
        for element in row_iter {
            print!("{}",element);
        }
        println!();
    }
}

const ADJACENT: [(i32,i32) ; 8] = [
    (-1,-1),(-1,0),(-1,1),
    (0,-1),(0,1),
    (1,-1),(1,0),(1,1)];

pub fn increment(model: &mut Model, i: usize, j: usize, if_zero: bool) {
    if let Some(&v) = model.get(i,j) {
        if v != 0 || if_zero {
            let _ = model.set(i,j,v + 1);
        }
    }
}

pub fn flash(model: &mut Model, i: usize, j: usize) {
    if let Some(&v) = model.get(i, j) {
        if v >= 10 {
            let _ = model.set(i,j,0);
            for (a,b) in ADJACENT {
                let i = (i as i32 + a) as usize;
                let j = (j as i32 + b) as usize;
                increment(model, i, j, false);
                flash(model, i, j);
            }
        }
    }
}

pub fn step(model: &mut Model) {
    // Increment everything by 1
    for i in 0..model.num_rows() {
        for j in 0..model.num_columns() {
            increment(model, i, j, true);
        }
    }

    // Flashes
    for i in 0..model.num_rows() {
        for j in 0..model.num_columns() {
            flash(model, i, j)
        }
    }
}

pub fn simulate(input: &Model, iterations: u32) -> u32 {
    let mut model = input.clone();
    let mut flashes = 0;

    if iterations <= 10 {
        println!("Before any steps:");
        print_model(&model);
        println!();
    }

    for s in 1..=iterations {
        step(&mut model);

        // Count flashes
        for i in 0..model.num_rows() {
            for j in 0..model.num_columns() {
                if let Some(&v) = model.get(i,j) {
                    if v == 0 {
                        flashes += 1;
                    }
                }
            }
        }
        
        if iterations <= 10 || s == iterations {
            println!("After step {}:", s);
            print_model(&model);
            println!();
        }
    }

    println!("After {} steps, there have been {} flashes",
        iterations, flashes);
    flashes
}

pub fn find_synchronization(input: &Model) -> u32 {
    let mut model = input.clone();
    let mut s = 0;

    loop {
        s += 1;
        step(&mut model);

        // Count flashes
        let mut flashes = 0;
        for i in 0..model.num_rows() {
            for j in 0..model.num_columns() {
                if let Some(&v) = model.get(i,j) {
                    if v == 0 {
                        flashes += 1;
                    }
                }
            }
        }

        if flashes == model.num_elements() {
            println!("Synchronization after {} steps", s);
            break s;
        }
    }
}

pub fn solve(input: &[u8]) -> (u32,u32) {
    let (_,model) = parser::parse(input).unwrap();
    let solution1 = simulate(&model, 100);
    let solution2 = find_synchronization(&model);
    (solution1, solution2)
}

#[test]
fn test11_0() {
    let solution = solve(include_bytes!("../inputs/day11.0"));
    assert_eq!(solution, (1656,195));
}

#[test]
fn test11_1() {
    let solution = solve(include_bytes!("../inputs/day11.1"));
    assert_eq!(solution, (1747,505));
}
