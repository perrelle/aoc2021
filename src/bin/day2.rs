use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;

    while {
        buf.clear();
        stdin.read_line(&mut buf).expect("cannot read line") != 0
    } {
        let words: Vec<_> = buf.trim().split_whitespace().collect();

        let (direction, value) =
            match words.as_slice() {
                [direction,value] =>
                    (*direction, value.parse::<i32>().expect("failed to parse input")),
                _ => panic!("incorrect input")
            };
        match direction {
            "forward" => {
                x += value;
                z += aim * value;
            }
            "down" => { aim += value; }
            "up" => { aim -= value; }
            _ => { panic!("incorrect command"); }
        }
    }
    
    println!("{}", x * z);
}
