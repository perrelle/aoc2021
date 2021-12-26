/* Traits should indeed be implemented, but it's painful for now to do that in
   Rust since the 4 versions (with and without references) has to be provided */
#![allow(clippy::should_implement_trait)]

use std::collections::HashSet;

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum Register { W, X, Y, Z }

#[derive(Debug)]
pub enum Operand {
    Integer(i32),
    Register(Register)
}

#[derive(Debug)]
pub enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand)
}

impl Instruction {
    pub fn left(&self) -> Register {
        match self {
            Instruction::Inp(l) => *l,
            Instruction::Add(l, _r) => *l,
            Instruction::Mul(l, _r) => *l,
            Instruction::Div(l, _r) => *l,
            Instruction::Mod(l, _r) => *l,
            Instruction::Eql(l, _r) => *l
        } 
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Register::W => write!(f, "w"),
            Register::X => write!(f, "x"),
            Register::Y => write!(f, "y"),
            Register::Z => write!(f, "z")
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operand::Register(r) => write!(f, "{}", r),
            Operand::Integer(i) => write!(f, "{}", i)
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instruction::Inp(l) => write!(f, "inp {}", l),
            Instruction::Add(l, r) => write!(f, "add {} {}", l, r),
            Instruction::Mul(l, r) => write!(f, "mul {} {}", l, r),
            Instruction::Div(l, r) => write!(f, "div {} {}", l, r),
            Instruction::Mod(l, r) => write!(f, "mod {} {}", l, r),
            Instruction::Eql(l, r) => write!(f, "eql {} {}", l, r),
        }
    }
}


mod parser {
    use nom::{IResult, character::complete::*, branch::*, sequence::*, multi::*, combinator::*};
    use super::*;

    pub fn register(input: &[u8]) -> IResult<&[u8], Register> {
        let (input,c) = satisfy(|c| ('w'..='z').contains(&c))(input)?;
        let r = match c {
            'w' => Register::W,
            'x' => Register::X,
            'y' => Register::Y,
            'z' => Register::Z,
            _ => panic!()
        };
        Ok((input, r))
    }

    pub fn regsister_op(input: &[u8]) -> IResult<&[u8], Operand> {
        let (input,r) = register(input)?;
        Ok((input, Operand::Register(r)))
    }

    pub fn integer_op(input: &[u8]) -> IResult<&[u8], Operand> {
        let (input,r) = i32(input)?;
        Ok((input, Operand::Integer(r)))
    }

    pub fn operand(input: &[u8]) -> IResult<&[u8], Operand> {
        alt((regsister_op, integer_op))(input)
    }

    pub fn instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
        let (input, ((op,lhs),rhs)) = 
            pair(
                separated_pair(alpha1, space1, register),
                opt(preceded(space1, operand)))(input)?;
        let instruction = match op {
            b"inp" => { assert!(rhs.is_none()); Instruction::Inp(lhs) },
            b"add" => Instruction::Add(lhs, rhs.unwrap()),
            b"mul" => Instruction::Mul(lhs, rhs.unwrap()),
            b"div" => Instruction::Div(lhs, rhs.unwrap()),
            b"mod" => Instruction::Mod(lhs, rhs.unwrap()),
            b"eql" => Instruction::Eql(lhs, rhs.unwrap()),
            _ => panic!()
        };
        Ok((input, instruction))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Instruction>> {
        terminated(
            separated_list1(multispace1, instruction),
            all_consuming(multispace0))(input)
    }
}

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
pub struct State {
    w: i64,
    x: i64,
    y: i64,
    z: i64
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "w: {}, x: {}, y: {}, z: {}", self.w, self.x, self.y, self.z)
    }
}

impl State {
    pub fn initial() -> State {
        State { w: 0, x: 0, y: 0, z: 0 }
    }

    pub fn get(&self, r: &Register) -> i64 {
        match r {
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
            Register::W => self.w
        }        
    }

    fn getl(&mut self, r: &Register) -> &mut i64 {
        match r {
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
            Register::W => &mut self.w
        }        
    }

    fn getr(&self, op: &Operand) -> i64 {
        match op {
            Operand::Register(r) => self.get(r),
            Operand::Integer(i) => *i as i64
        }
    }

    pub fn execute(&mut self, instruction: &Instruction, input: Option<i64>) {
        match instruction {
            Instruction::Inp(l) => *self.getl(l) = input.unwrap(),
            Instruction::Add(l, r) => *self.getl(l) += self.getr(r),
            Instruction::Mul(l, r) => *self.getl(l) *= self.getr(r),
            Instruction::Div(l, r) => *self.getl(l) /= self.getr(r),
            Instruction::Mod(l, r) => *self.getl(l) %= self.getr(r),
            Instruction::Eql(l, r) => {
                let right = self.getr(r);
                let left = self.getl(l);
                *left = if *left == right {1} else {0}
            }
        }
    }
}

#[derive(Clone,Copy,PartialEq,Eq)]
pub struct Interval {
    l: i64,
    u: i64
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.l, self.u)
    }
}

impl Interval {
    pub const ZERO: Self = Self { l: 0, u: 0 };

    pub fn singleton(i: i64) -> Interval {
        Interval { l: i, u: i }
    }

    pub fn to_singleton(&self) -> Option<i64> {
        if self.l == self.u {
            Some(self.l)
        }
        else {
            None
        }
    }

    pub fn inter(&mut self, other: &Self) {
        if self.l < other.l {
            self.l = other.l
        }
        if self.u > other.u {
            self.u = other.u
        }
    }

    pub fn contains(&self, x: i64) -> bool {
        self.l <= x && x <= self.u
    }

    pub fn remove(&self, x: i64) -> Self {
        Self {
            l: self.l + if self.l == x {1} else {0},
            u: self.u - if self.u == x {1} else {0}
        }
    }

    pub fn add(x1: Self, x2: Self) -> Self {
        Interval {
            l: x1.l + x2.l,
            u: x1.u + x2.u
        }
    }

    pub fn sub(x1: Self, x2: Self) -> Self {
        Interval {
            l: x1.l - x2.u,
            u: x1.u - x2.l
        }
    }

    pub fn mul(x1: Self, x2: Self) -> Self {
        use std::cmp::{min,max};
        let a = x1.l * x2.l;
        let b = x1.u * x2.l;
        let c = x1.l * x2.u;
        let d = x1.u * x2.u;
        Interval {
            l: min(min(a, b), min(c, d)),
            u: max(max(a, b), max(c, d))
        }
    }

    pub fn div(x1: Self, x2: Self) -> Self {
        use std::cmp::{min,max};
        if x2.l < 0 && x2.u > 0 {
            Interval {
                l: min(x1.l, -x1.u),
                u: max(x1.u, -x1.l)
            }
        }
        else {
            let a = x1.l / if x2.l == 0 {1} else {x2.l};
            let b = x1.u / if x2.l == 0 {1} else {x2.l};
            let c = x1.l / if x2.u == 0 {-1} else {x2.u};
            let d = x1.u / if x2.u == 0 {-1} else {x2.u};
            Interval {
                l: min(min(a, b), min(c, d)),
                u: max(max(a, b), max(c, d))
            }
        }
    }
    
    pub fn dmod(x1: Self, x2: Self) -> Self {
        if x1.l >= 0 && x1.u < x2.l {
            x1    
        }
        else {
            Interval {l: 0, u: x2.u - 1}
        }
    }

    pub fn eql(x1: Self, x2:Self) -> Self {
        if x1.l == x1.u && x1.l == x2.l && x2.l == x2.u {
            Interval {l: 1, u: 1}
        }
        else if x1.l > x2.u || x2.l > x1.u {
            Interval {l: 0, u: 0}
        }
        else {
            Interval {l: 0, u: 1}
        }
    }
}

#[derive(Clone)]
pub struct AbstractState {
    w: Interval,
    x: Interval,
    y: Interval,
    z: Interval
}

impl std::fmt::Display for AbstractState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "w: {}, x: {}, y: {}, z: {}", self.w, self.x, self.y, self.z)
    }
}

impl AbstractState {
    pub fn initial() -> Self {
        Self {
            w: Interval::ZERO,
            x: Interval::ZERO,
            y: Interval::ZERO,
            z: Interval::ZERO
        }
    }

    pub fn singleton(state: &State) -> Self {
        Self {
            w: Interval::singleton(state.w),
            x: Interval::singleton(state.x),
            y: Interval::singleton(state.y),
            z: Interval::singleton(state.z)
        }
    }

    pub fn inter(&mut self, other: &Self) {
        self.w.inter(&other.w);
        self.x.inter(&other.x);
        self.y.inter(&other.y);
        self.z.inter(&other.z);
    }

    pub fn get(&self, r: &Register) -> Interval {
        match r {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z
        }
    }

    fn getl(&mut self, r: &Register) -> &mut Interval {
        match r {
            Register::W => &mut self.w,
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z
        }
    }

    fn getr(&self, op: &Operand) -> Interval {
        match op {
            Operand::Register(r) => self.get(r),
            Operand::Integer(i) => Interval::singleton(*i as i64)
        }
    }

    fn fwbinop(&mut self, f: fn(Interval,Interval) -> Interval, l: &Register, r: &Operand) {
        let right = self.getr(r);
        let left = self.getl(l);
        *left = f(*left,right);
    }

    pub fn forward(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Inp(l) => *self.getl(l) = Interval {l: 1, u: 9},
            Instruction::Add(l, r) => self.fwbinop(Interval::add, l, r),
            Instruction::Mul(l, r) => self.fwbinop(Interval::mul, l, r),
            Instruction::Div(l, r) => self.fwbinop(Interval::div, l, r),
            Instruction::Mod(l, r) => self.fwbinop(Interval::dmod, l, r),
            Instruction::Eql(l, r) => self.fwbinop(Interval::eql, l, r)
        }
    }

    // Buggy: do not use
    pub fn backward(&mut self, succ: &Self, instruction: &Instruction) {
        let l = instruction.left();
        for reg in [Register::W, Register::X, Register::Y, Register::Z] {
            if reg != l {
                self.getl(&reg).inter(&succ.get(&reg));
            }
        }

        match instruction {
            Instruction::Inp(l) => {
                println!("Input in {}", succ.get(l));
            }
            Instruction::Add(l, r) => {
                // l' = l + r
                // l = l' - r
                self.getl(l).inter(&Interval::sub(succ.get(l), succ.getr(r)));
                // r = l' - l
                if let Operand::Register(r) = r {
                    let prev = self.get(l);
                    self.getl(r).inter(&Interval::sub(succ.get(l), prev));
                }
            }
            Instruction::Mul(l, r) => {
                let left = succ.get(l);
                let right = succ.getr(r);
                // l' = l * r 
                // l = l' / r si r != 0  Ou bien  l' = r = 0
                if !right.contains(0) || !left.contains(0) {
                    self.getl(l).inter(&Interval::div(left, right));
                }
                // r = l' / l si l != 0  Ou bien  l' = l = 0
                if let Operand::Register(r) = r {
                    let prev = self.get(l);
                    if !prev.contains(0) || !left.contains(0) {
                        self.getl(r).inter(&Interval::div(left, prev));
                    }
                }
            }
            Instruction::Div(l, r) => {
                let left = succ.get(l);
                let right = succ.getr(r);
                let prev = self.get(l);
                let rem = Interval {l: 0, u: right.u - 1};
                // l' = (l - rem) / r
                // l = l' * r + rem
                self.getl(l).inter(
                    &Interval::add(
                        Interval::mul(succ.get(l), succ.getr(r)),
                        rem));
                // r = (l - rem) / l'  si l' != 0  Ou bien  l' = l = rem = 0
                if let Operand::Register(r) = r {
                    if !prev.contains(0) || !left.contains(0) {
                        self.getl(r).inter(
                            &Interval::div(
                                Interval::sub(prev, rem),
                                left));
                    }
                }
            }
            Instruction::Mod(_l, _r) => (),
            Instruction::Eql(l, r) => {
                let left = succ.get(l);
                let right = succ.getr(r);
                let prev = self.get(l);
                if left == Interval::singleton(1) {                   
                    self.getl(l).inter(&succ.getr(r));
                    if let Operand::Register(r) = r {
                        self.getl(r).inter(&prev);
                    }                   
                }
                else if succ.get(l) == Interval::singleton(0) {
                    if let Some(x) = right.to_singleton() {
                        self.getl(l).inter(&prev.remove(x));
                    }
                    if let Operand::Register(r) = r {
                        if let Some(x) = prev.to_singleton() {
                            self.getl(r).inter(&prev.remove(x));
                        }
                    }
                }
            }
        }
    }

    pub fn enumerate(&self) -> HashSet<State> {
        let mut r = HashSet::new();
        for w in self.w.l..=self.w.u {
            for x in self.x.l..=self.x.u {
                for y in self.y.l..=self.y.u {
                    for z in self.z.l..=self.z.u {
                        r.insert(State {w, x, y, z});
                    }
                }
            }
        }
        r
    }
}

fn forward_interpret(program: &[Instruction], initial: &State) -> AbstractState {
    let mut state = AbstractState::singleton(initial);
    for instruction in program {
        state.forward(instruction);
    }    
    state
}

fn branched_execution(program: &[Instruction], mut state: State, ascending: bool) -> Option<Vec<i64>> {
    for (i,instruction) in program.iter().enumerate() {
        if let Instruction::Inp(_) = instruction {
            let range: Vec<i64> =
                if ascending {
                    (1..=9).collect()
                }
                else {
                    (1..=9).rev().collect()
                };
            for x in range  {               
                let mut state = state.clone();
                state.execute(instruction, Some(x));
                let final_state = forward_interpret(&program[(i+1)..], &state);
                let interval = final_state.get(&Register::Z);
                if let Interval {l:0, u:0} = interval {
                    return Some(vec![x]);
                } 
                else if interval.contains(0) {
                    if let Some(mut v) = branched_execution(&program[(i+1)..], state, ascending) {
                        v.push(x);
                        return Some(v);
                    }
                }
            }
            return None
        }
        else {
            state.execute(instruction,None);
        }
    }

    panic!();
}

pub fn execute(program: &[Instruction], input: &[i64]) {
    let mut state = State::initial();
    let mut iter = input.to_owned().into_iter();

    for (i,instruction) in program.iter().enumerate() {
        println!("{}", state);
        println!("{}: {}", i, instruction);
        if let Instruction::Inp(_) = instruction {
            state.execute(instruction, iter.next());
        }
        else {
            state.execute(instruction,None);
        }
    }

    println!("Final state: {}", state);
}

fn solve_part(program: &[Instruction], part2: bool) -> i64 {
    let result = branched_execution(program, State::initial(), part2).unwrap();
    result.iter().rev().fold(0, |acc, d| acc * 10 + d)
}

pub fn solve(input: &[u8]) -> (i64,i64) {
    let (_,program) = parser::parse(input).unwrap();
    let solution1 = solve_part(&program, false);
    println!("largest serial number: {}", solution1);
    let solution2 = solve_part(&program, true);
    println!("smallest serial number: {}", solution2);
    (solution1, solution2)
}

pub fn naive_solve(input: &[u8]) -> (i32,i32) {
    let (_,program) = parser::parse(input).unwrap();
    let mut set = HashSet::new();
    set.insert(State::initial());

    for (i,instruction) in program.iter().enumerate() {
        println!("{} states", set.len());
        println!("{}: {}", i, instruction);

        let mut new_set = HashSet::new();
        for mut s in set {
            if let Instruction::Inp(_) = instruction {
                for i in 1..=9 {
                    let mut s2 = s.clone();
                    s2.execute(instruction, Some(i));
                    new_set.insert(s2);
                }
            }
            else {
                s.execute(instruction,None);
                new_set.insert(s);
            }
        }
        set = new_set;
    }
    (0, 0)
}

#[test]
fn test24() {
    let solution = solve(include_bytes!("../inputs/day24"));
    assert_eq!(solution, (96918996924991,91811241911641));
}
