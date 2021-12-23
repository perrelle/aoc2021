use crate::mdarray::*;

#[derive(Debug,Clone)]
pub struct Interval {
    l: i32,
    u: i32
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.l, self.u)
    }
}

impl Interval {
    pub fn inter(&self, other: &Self) -> Option<Self> {
        use::std::cmp::{min,max};
        let l = max(self.l, other.l);
        let u = min(self.u, other.u);
        if l <= u {
            Some(Interval {l,u})
        }
        else {
            None
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.l <= other.u && other.l <= self.u
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.l <= other.l && self.u >= other.u
    }

    pub fn diff_and_inter(self, other: &Self) -> Vec<Self> {
        if !self.intersects(other) {
            vec![self]
        } else {
            let mut r = Vec::new();
            let l =
                if other.l > self.l {
                    r.push(Interval {l: self.l, u: other.l - 1});
                    other.l
                }
                else {
                    self.l
                };
            let u =
                if other.u < self.u {
                    r.push(Interval {l: other.u + 1, u: self.u});
                    other.u
                }
                else {
                    self.u
                };
            r.push(Interval {l, u});
            r
        }
    }

    pub fn cardinal(&self) -> u64 {
        (self.u + 1 - self.l) as u64
    }
}

#[derive(Debug,Clone)]
pub struct Cuboid {
    xrange: Interval,
    yrange: Interval,
    zrange: Interval
}

impl std::fmt::Display for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.xrange, self.yrange, self.zrange)
    }
}

impl Cuboid {
    pub fn inter(&self, other: &Cuboid) -> Option<Cuboid> {
        Some(Cuboid {
            xrange: self.xrange.inter(&other.xrange)?,
            yrange: self.yrange.inter(&other.yrange)?,
            zrange: self.zrange.inter(&other.zrange)?
        })
    }

    pub fn intersects(&self, other: &Cuboid) -> bool {
        self.xrange.intersects(&other.xrange) &&
        self.yrange.intersects(&other.yrange) &&
        self.zrange.intersects(&other.zrange)
    }

    pub fn includes(&self, other: &Cuboid) -> bool {
        self.xrange.includes(&other.xrange) &&
        self.yrange.includes(&other.yrange) &&
        self.zrange.includes(&other.zrange)
    }

    pub fn diff(self: Cuboid, other: &Cuboid) -> Vec<Cuboid> {
        if self.intersects(other) {
            let mut r = Vec::new();
            for xrange in self.xrange.clone().diff_and_inter(&other.xrange) {
                for yrange in self.yrange.clone().diff_and_inter(&other.yrange) {
                    for zrange in self.zrange.clone().diff_and_inter(&other.zrange) {
                        let subcuboid = Cuboid {
                            xrange: xrange.clone(),
                            yrange: yrange.clone(),
                            zrange: zrange.clone()
                        };
                        if !other.includes(&subcuboid) {
                            assert!(!subcuboid.intersects(other));
                            r.push(subcuboid);
                        }
                    }
                }
            }
            r
        }
        else {
            vec![self]
        }
    }

    pub fn volume(&self) -> u64 {
        self.xrange.cardinal() *
        self.yrange.cardinal() *
        self.zrange.cardinal()
    }
}

pub type State = bool;

#[derive(Debug)]
pub struct Step {
    state: State,
    cuboid: Cuboid
}

mod parser {
    use nom::{
        IResult, character::complete::*, bytes::complete::*,
        branch::*, sequence::*, multi::*, combinator::*
    };
    use super::*;

    pub fn step(input: &[u8]) -> IResult<&[u8], Step> {
        let (input,(state,_,x1,_,x2,_,y1,_,y2,_,z1,_,z2)) = tuple((
            alt((tag("on"), tag("off"))),
            tag(" x="), i32, tag(".."), i32,
            tag(",y="), i32, tag(".."), i32,
            tag(",z="), i32, tag(".."), i32
        ))(input)?;

        let step = Step {
            state: state == b"on",
            cuboid: Cuboid {
                xrange: Interval{l:x1,u:x2},
                yrange: Interval{l:y1,u:y2},
                zrange: Interval{l:z1,u:z2}
            }
        };

        Ok((input, step))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Step>> {
        let (input, steps) = separated_list1(multispace1, step)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, steps))
    }
}

pub fn naive_solve(steps: &[Step], area: &Cuboid) -> u64 {
    let Cuboid {
            xrange: Interval{l:xx1, u:xx2},
            yrange: Interval{l:yy1, u:yy2},
            zrange: Interval{l:zz1, u:zz2}
        } = area;
    let mut cubes = Array3D::new(
        false,
        (xx2 - xx1 + 1) as usize,
        (yy2 - yy1 + 1) as usize,
        (zz2 - zz1 + 1) as usize);

    for step in steps {
        if let Some(Cuboid {
                xrange: Interval{l:x1,u:x2},
                yrange: Interval{l:y1,u:y2},
                zrange: Interval{l:z1,u:z2}
            }) = step.cuboid.inter(area) {  
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        cubes.set(
                            (x-xx1) as usize,
                            (y-yy1) as usize,
                            (z-zz1) as usize,
                            step.state);
                    }
                }
            }
        }
    }

    let mut count = 0;
    for c in cubes.iter() {
        if c { 
            count += 1;
        }
    }
    count
}

pub fn smart_solve(steps: &[Step], area: Option<&Cuboid>) -> u64 {
    let mut cuboids = Vec::new();

    for step in steps {
        let new_cuboid_opt =
            if let Some(area) = area {
                step.cuboid.inter(area)
            } else {
                Some(step.cuboid.clone())
            };
        if let Some(new_cuboid) = new_cuboid_opt {
            if step.state {
                cuboids.push(new_cuboid);
            }
            else {
                let mut new_cuboids = Vec::new();
                for c in cuboids {
                    new_cuboids.append(&mut c.diff(&new_cuboid));
                }
                cuboids = new_cuboids;
            }
        }
    }

    let mut disjoint_cuboids: Vec<Cuboid> = Vec::new();
    for cuboid in cuboids {
        let mut parts = vec![cuboid];
        for c1 in &disjoint_cuboids {
            let iter = parts.into_iter().map(|c2| c2.diff(c1));
            parts = Vec::new();
            iter.for_each(|mut l| parts.append(&mut l))
        }
        disjoint_cuboids.append(&mut parts);
    }

    disjoint_cuboids.iter().map(|c| c.volume()).sum()
}

pub fn solve(input: &[u8], area: Cuboid) -> (u64,u64) {
    let (_,steps) = parser::parse(input).unwrap();
    let solution1 = smart_solve(&steps, Some(&area));
    let solution2 = smart_solve(&steps, None);
    println!("{} cubes after initialization, {} after reboot", solution1, solution2);
    (solution1,solution2)
}

#[test]
fn test22_0() {
    let area = Cuboid {
        xrange: Interval {l: 0, u:20},
        yrange: Interval {l: 0, u:20},
        zrange: Interval {l: 0, u:20}
    };
    let solution = solve(include_bytes!("../inputs/day22.0"), area);
    assert_eq!(solution, (39,39));
}

#[test]
fn test22_1() {
    let area = Cuboid {
        xrange: Interval {l: -50, u: 50},
        yrange: Interval {l: -50, u: 50},
        zrange: Interval {l: -50, u: 50},
    };
    let solution = solve(include_bytes!("../inputs/day22.1"), area);
    assert_eq!(solution, (590784,39769202357779));
}

#[test]
fn test22_2() {
    let area = Cuboid {
        xrange: Interval {l: -50, u: 50},
        yrange: Interval {l: -50, u: 50},
        zrange: Interval {l: -50, u: 50},
    };
    let solution = solve(include_bytes!("../inputs/day22.2"), area);
    assert_eq!(solution, (658691,1228699515783640));
}

#[test]
fn test22_3() {
    let area = Cuboid {
        xrange: Interval {l: -50, u: 50},
        yrange: Interval {l: -50, u: 50},
        zrange: Interval {l: -50, u: 50},
    };
    let solution = solve(include_bytes!("../inputs/day22.3"), area);
    assert_eq!(solution, (474140,2758514936282235));
}
