use std::collections::HashMap;

pub type Polymer = Vec<char>;
pub type Production = ((char,char),char);
pub type Input = (Polymer, Vec<Production>);

mod parser  {
    use nom::{
        IResult, multi::*, character::complete::*,
        bytes::complete::*, sequence::*, combinator::*};
    use super::*;

    fn elem(input: &[u8]) -> IResult<&[u8], char> {
        satisfy(|c| c > 'A' && c < 'Z')(input)
    }

    fn production(input: &[u8]) -> IResult<&[u8], Production> {
        let (input, (left, _, right, _)) =
            tuple((pair(elem, elem), tag(" -> "), elem, multispace0))(input)?;
        Ok((input, (left, right)))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Input> {
        let (input, (polymer, _, productions)) =
            tuple((many1(elem), multispace1, many1(production)))(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, (polymer.to_vec(), productions)))
    }
}

fn polymer_to_string(p: &[char]) -> String {
    let s : String = p.iter().cloned().collect();
    if s.len() <= 60 {
        s
    } else {
        String::from(s.as_str().get(0..59).unwrap()) + "..."
    }
}

fn incr_map<K>(map: &mut HashMap<K,u64>, k: K, incr : u64)
    where K: std::cmp::Eq, K: std::hash::Hash
{
    let e = map.entry(k);
    e.and_modify(|x| { *x += incr }).or_insert(incr);
}

fn count(p: &[char]) -> HashMap<char, u64> {
    let mut map = HashMap::new();
    for &c in p {
        incr_map(&mut map, c, 1);
    }
    map
}

fn counts_to_solution(counts: &HashMap<char, u64>) -> u64 {
    let min = counts.iter().min_by_key(|&(_,n)| n).unwrap();
    let max = counts.iter().max_by_key(|&(_,n)| n).unwrap();
    println!("{:?}, min : {:?}, max : {:?}", counts, min, max);
    max.1 - min.1
}

fn part1(starting_polymer: &[char],
    productions: &HashMap<(char,char), char>, max_iterations: u32) -> u64
{
    let mut current_polymer : Polymer = starting_polymer.to_owned(); 

    for s in 1..=max_iterations {
        let mut p = Vec::new();
        let mut iter = current_polymer.iter();
        let mut last = *iter.next().unwrap();
        p.push(last);
        for &cur in iter {
            if let Some(&new) = productions.get(&(last,cur)) {
                p.push(new);
            }
            p.push(cur);
            last = cur;
        }
        current_polymer = p;

        println!("After step {}: {}", s, polymer_to_string(&current_polymer));
    }

    let counts = count(&current_polymer);
    counts_to_solution(&counts)
}

fn part2(starting_polymer: &[char],
    productions: &HashMap<(char,char), char>, max_iterations: u32) -> u64
{
    // Count initial pairs
    let mut starting_pairs = HashMap::new();    

    let mut iter = starting_polymer.iter();
    let mut last = *iter.next().unwrap();
    for &cur in iter {
        incr_map(&mut starting_pairs, (last,cur), 1);
        last = cur;
    }

    // Produce pairs by pairs
    let mut current_pairs = starting_pairs;
    for _s in 1..=max_iterations {
        let mut new_pairs = HashMap::new();
        for (pair @ (l1,l2), i) in current_pairs {
            if let Some(&r) = productions.get(&pair) {
                incr_map(&mut new_pairs,(l1,r),i);
                incr_map(&mut new_pairs,(r,l2),i);
            } else {
                incr_map(&mut new_pairs,pair,i);
            }
        }
        current_pairs = new_pairs;
    }

    // Count element
    let mut counts = HashMap::new();
    for ((l1,_), i) in current_pairs {
        incr_map(&mut counts, l1, i);
    }
    if let Some(&element) = starting_polymer.iter().last() {
        incr_map(&mut counts, element, 1);
    }

    counts_to_solution(&counts)
}

pub fn solve(input: &[u8]) -> (u64,u64) {
    let (_,(polymer,productions)) = parser::parse(input).unwrap();
    let table : HashMap<(char,char), char> = productions.into_iter().collect();
    let solution1 = part1(&polymer, &table, 10);
    let solution2 = part2(&polymer, &table, 40);
    (solution1, solution2)
}

#[test]
fn test14_0() {
    let solution = solve(include_bytes!("../inputs/day14.0"));
    assert_eq!(solution, (1588,2188189693529));
}

#[test]
fn test14_1() {
    let solution = solve(include_bytes!("../inputs/day14.1"));
    assert_eq!(solution, (3143,4110215602456));
}
