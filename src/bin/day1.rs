use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut last = None;
    let mut penultimate = None;
    let mut lastsum = None;
    let mut increase = 0;

    while stdin.read_line(&mut buf).expect("cannot read line") != 0 {
        let n = buf.trim().parse::<i32>().expect("failed to parse input");
        match (last,penultimate) {
            (_,None) | (None,_) => (),
            (Some(m),Some(p)) => {
                let sum = n+m+p;
                match lastsum {
                    None => (),
                    Some(s) =>
                        if sum > s {
                            increase = increase + 1;
                        }
                }
                lastsum = Some(sum);
            }
        }

        buf.clear();
        penultimate = last;
        last = Some(n);
    }
    
    println!("{} increase", increase);
}
