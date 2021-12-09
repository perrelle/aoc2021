mod parser  {
    use std::collections::HashSet;
    use nom::{
        multi::separated_list1,
        sequence::tuple,
        IResult,
        bytes::complete::tag,
        character::complete::{alpha1, multispace0, multispace1, space1},
        combinator::all_consuming,
    };

    pub type Word = HashSet<char>;
    pub type Entry = (Vec<Word>,Vec<Word>);

    fn word(input: &[u8]) -> IResult<&[u8], Word> {
        let (input, v) = alpha1(input)?;
        let word = HashSet::from_iter(v.iter().map(|&c| c as char));
        Ok((input, word))
    }

    fn entry(input: &[u8]) -> IResult<&[u8], Entry> {
        let (input, (v1,_,v2)) = tuple((
            separated_list1(space1, word),
            tag(" | "),
            separated_list1(space1, word)))(input)?;
        Ok((input, (v1,v2)))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Entry>> {
        let (input, l) = separated_list1(multispace1, entry)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, l))
    }
}

use std::collections::HashSet;
use std::collections::HashMap;


fn part1(input : &Vec<parser::Entry>) {
    // Part 1
    let mut count = 0;
    for (_signals,outputs) in input {
        for w in outputs {
            if [2, 3, 4, 7].contains(&(w.len() as i32)) {
                count += 1;
            }
        }
    }

    println!("Outputs that use a unique number of segments: {}", count);
}

struct VariableMap<K,V> {
    pub values: HashMap<K,V>,
    name: String
}

impl<K,V> VariableMap<K,V> where
    K: std::cmp::Eq,
    K: std::hash::Hash,
    K: std::fmt::Display,
    K: std::clone::Clone
{
    fn new(name: &str) -> VariableMap<K,V> {
        VariableMap { values: HashMap::new(), name: String::from(name) }
    }

    fn def(&mut self, x : K, y : V) {
        if let Some(_) = self.values.insert(x.clone(), y) {
            panic!("Found several candidates for {} {} in entry", self.name, x);
        }
    }

    fn get(&self, x : K) -> &V {
        match self.values.get(&x) {
            Some(y) => y,
            None => panic!("Did not find {} {} in entry", self.name, &x)
        }
    }
}

trait Singleton<K> {
    fn singleton(x : K) -> HashSet<K>;
    fn to_singleton(&self) -> Option<&K>;
}

impl<K> Singleton<K> for HashSet<K> where
        K: std::cmp::Eq,
        K: std::hash::Hash {
    fn singleton(x : K) -> HashSet<K> {
        let mut set = HashSet::new();
        let _ = set.insert(x);
        set
    }

    fn to_singleton(&self) -> Option<&K> {
        let mut iter = self.iter();
        let r = iter.next()?;
        if let None = iter.next() { Some(r) } else { None }
    }
}

fn solve_entry(signals : &Vec<parser::Word>, outputs : &Vec<parser::Word>) -> i32 {
    let mut digits : VariableMap<i32, &parser::Word> = VariableMap::new("digit");
    let mut segments : VariableMap<char, char> = VariableMap::new("segment");

    let mut five_seg_words : Vec<&parser::Word> = Vec::new();
    let mut six_seg_words : Vec<&parser::Word> = Vec::new();
    
    // - Step 1 -
    // Identify easy digits, digits which are the only one to use a number
    // of segments
    for w in signals {
        match w.len() {
            2 => digits.def(1, w),
            3 => digits.def(7, w),
            4 => digits.def(4, w),
            5 => five_seg_words.push(w),
            6 => six_seg_words.push(w),
            7 => digits.def(8, w),
            _ => panic!("Found a strange input word: {:?}", w)
        }
    }
    digits.get(7).difference(digits.get(1)).
        for_each(|&y| segments.def('a',y));

    // - Step 2 -
    // The 6 is the only digit not containing 1 segments
    for w in &six_seg_words {
        if !digits.get(1).iter().all(|s| w.contains(s)) {
            digits.def(6, w);
        }
    }

    // - Step 3 -
    // Segment c is in the difference 6 / 1
    digits.get(1).difference(digits.get(6)).
        for_each(|&y| segments.def('c',y));
    // Segment f is the other one
    digits.get(1).difference(&HashSet::singleton(*segments.get('c'))).
        for_each(|&y| segments.def('f',y));
   
    // - Step 4 -
    // Classify 5 segments digits
    {
        let c = segments.get('c');
        let f = segments.get('f');
    
        for w in &five_seg_words {
            match (w.contains(c), w.contains(f)) {
                (true, false) => digits.def(2, w),
                (true, true) => digits.def(3, w),
                (false, true) => digits.def(5, w),
                (false, false) => panic!("Found a strange input word: {:?}", w)
            }
        }
    }

    // - Step 5 -
    // Segment b is in the difference 5 / 3
    digits.get(5).difference(digits.get(3)).
        for_each(|&y| segments.def('b',y));
    // Segment e is in the difference 2 / 3
    digits.get(2).difference(digits.get(3)).
        for_each(|&y| segments.def('e',y));
   
    // - Step 6 -
    // Classify 6 segments digits
    {
        let c = segments.get('c');
        let e = segments.get('e');
    
        for w in &six_seg_words {
            match (w.contains(c), w.contains(e)) {
                (true, true) => digits.def(0, w),
                (false, true) => (), // 6 already defined in step 2
                (true, false) => digits.def(9, w),
                (false, false) => panic!("Found a strange input word: {:?}", w)
            }
        }
    }

    // - Decode output -
    
    outputs.iter().map(|w| { 
        let (d,_) = digits.values.iter().find(|(_,v)| **v == w).expect("Can't map digit");
        d
     }).fold(0, |acc,d| acc * 10 + d)
}

fn part2(input : &Vec<parser::Entry>) {
    let mut sum = 0;
    for (signals,outputs) in input {
        let r = solve_entry(signals, outputs);
        println!("{:?}: {}", outputs, r);
        sum += r;
    }
    println!("Addition of values: {}", sum);
}

fn solve(data: &[u8]) {
    let (_,input) = parser::parse(data).unwrap();
    part1(&input);
    part2(&input);
}

fn main() {

}

#[test]
fn test0() {
    solve(include_bytes!("../../inputs/day8.0"));
}

#[test]
fn test1() {
    solve(include_bytes!("../../inputs/day8.1"));
}
