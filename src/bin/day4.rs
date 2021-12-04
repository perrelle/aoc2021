use std::io::{self, prelude::*};

fn read_line<B>(buf : &mut String, mut f : impl FnMut(&[&str]) -> B) -> B {
    buf.clear();
    io::stdin().read_line(buf).expect("cannot read line");
    let words : Vec<&str> = buf.trim().split_whitespace().collect();
    f(words.as_slice())
}

fn input() -> (Vec<u32>,Vec<[[u32 ; 5] ; 5]>) {
    let mut buf = String::new();
    
    let numbers : Vec<u32> = read_line(&mut buf, |s|
        match s {
            [w] => w.split(',').map(|n|
                    n.parse::<u32>().expect("failed to parse input")
                ).collect(),
            _ => panic!("invalid input")
        });

    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut grids = Vec::new();
    let mut current_grid = [[0 ; 5] ; 5];
    let mut i = 0;

    lines.for_each(|s| {
        let str = s.unwrap();
        let words: Vec<&str> = str.split_whitespace().collect();
        match words.as_slice() {
            [] => (),
            w => {
                for j in 0..5 {
                    let x = w[j].parse::<u32>().expect("failed to parse input");
                    current_grid[i][j] = x;
                }
                i += 1;
                if i >= 5 {
                    grids.push(current_grid);
                    i = 0;
                }
            }
        }
    });

    (numbers, grids)
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

fn main() { 
    let (numbers, grids) = input();
    println!("Part 1 - final score is {}", part1(&numbers, &grids));
    println!("Part 2 - final score is {}", part2(&numbers, &grids));
}
