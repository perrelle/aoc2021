pub struct Rect {x1: i32, x2: i32, y1: i32, y2: i32}

pub fn inside(target: &Rect, x: i32, y: i32) -> bool {
    x >= target.x1 && x <= target.x2 && y >= target.y1 && y <= target.y2
}

#[allow(clippy::comparison_chain)]
pub fn simulate(target: &Rect, vx0 : i32, vy0: i32) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx0;
    let mut vy = vy0;
    let mut maxy = 0;
    //println!("--- start with velocity {},{}", vx0, vy0);

    while x <= target.x2 && y >= target.y1 {
        //println!("{},{} with speed {},{}", x, y, vx, vy);
        maxy = std::cmp::max(maxy, y);

        if inside(target, x, y) {
            //println!("solution with initial speed {},{} hits target at {},{}", vx0, vy0, x, y);
            return Some(maxy);
        }

        x += vx;
        y += vy;
        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        }
        vy -= 1;
    }
    None
}


pub fn solve(target: &Rect) -> (i32,u32) {
    let mut count: u32 = 0;
    let mut maxy: i32 = 0;

    for vx in 1..=target.x2 {
        for vy in target.y1..=-target.y1 {
            if let Some(y) = simulate(target, vx, vy) {
                maxy = std::cmp::max(maxy, y);
                count += 1;
            }
        }
    }

    (maxy, count)
}


#[test]
fn test17_0() {
    let solution = solve(&Rect{x1:20,x2:30,y1:-10,y2:-5});
    assert_eq!(solution, (45,112));
}

#[test]
fn test17_1() {
    let solution = solve(&Rect{x1:81,x2:129,y1:-150,y2:-108});
    assert_eq!(solution, (11175,3540));
}
