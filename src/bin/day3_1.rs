use std::io::{self, prelude::*};

fn input(mut f : impl FnMut(&[&str]) -> ()) {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    lines.for_each(|s| {
        let str = s.unwrap();
        let words: Vec<&str> = str.split_whitespace().collect();
        f(words.as_slice());
    })
}

fn convert_rate(v : Vec<u32>) -> u32 {
    v.iter().fold(0, |rate, &x| { 2 * rate + x })
}

fn main() { 
    let mut counts : Option<Vec<u32>> = None;
    let mut n = 0;

    input(|words : &[&str]| {
        let data =
            match words {
                [data] => data.chars().map(|c| c.to_digit(2).unwrap()),
                _ => panic!("incorrect input")
            };
        n = n + 1;

        match &mut counts {
            None => counts = Some (data.collect::<Vec<u32>>()),
            Some(counts) => {
                for (i, b) in data.enumerate() {
                    counts[i] += b;
                }
            }
        };
    });

    if let Some(counts) = counts {
        let (g,e) =
            counts
                .into_iter()
                .map(|x| if x > n - x { (1,0) } else { (0,1) })
                .unzip();
        let gamma = convert_rate(g);
        let epsilon = convert_rate(e);
        println!("gamma: {} epsilon: {}, consumption: {}", gamma, epsilon, gamma * epsilon);   
    }
}
