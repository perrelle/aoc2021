use std::io::{self, prelude::*};

fn input<B>(mut f : impl FnMut(&[&str]) -> B) -> Vec<B> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    lines.map(|s| {
        let str = s.unwrap();
        let words: Vec<&str> = str.split_whitespace().collect();
        f(words.as_slice())
    }).collect()
}

fn convert_rate(v : &Vec<u32>) -> u32 {
    v.iter().fold(0, |rate, &x| { 2 * rate + x })
}

fn compute_rate(majority: bool, data : &Vec<Vec<u32>>) -> &Vec<u32> {
    let mut set : Vec<&Vec<u32>> = data.iter().collect();
    let mut index = 0;

    while set.len() > 1 {
        let total : u32 = set.len().try_into().unwrap();
        let count = set.iter().fold(0, |n, v| n + v[index]);
        let bit = if majority == (count >= total - count) { 1 } else { 0 };
        set = set.into_iter().filter(|v| v[index] == bit).collect();
        index += 1;
    } 

    set[0]
}

fn main() { 
    let data : Vec<Vec<u32>> = input(|words : &[&str]| {
        match words {
            [data] => data.chars().map(|c| c.to_digit(2).unwrap()).collect(),
            _ => panic!("incorrect input")
        }
    });

    let oxygen = convert_rate(&compute_rate(true, &data));
    let dhmo = convert_rate(&compute_rate(false, &data));
    println!("Ogygen: {}, CO²: {}, Life support rating: {}", oxygen, dhmo, oxygen * dhmo);
}
