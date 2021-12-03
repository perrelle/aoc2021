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

fn main() { 
    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;

    input(|words : &[&str]| {
        let (direction, value) : (&str,i32) =
            match words {
                [direction,value] => (direction, value.parse().unwrap()),
                _ => panic!("incorrect input")
            };
        match direction {
            "forward" => { x += value; z += aim * value; }
            "down" => { aim += value; }
            "up" => { aim -= value; }
            _ => { panic!("incorrect command"); }
        }
    });
    
    println!("{}", x * z);
}
