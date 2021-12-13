use std::collections::HashMap;

pub type Edge = (String,String);
pub type Graph = HashMap<String,Vec<String>>;
pub type Marks = HashMap<String,u32>;

mod parser  {
    use nom::{
        IResult, multi::*, character::complete::*, bytes::complete::*,
        sequence::*, combinator::*};

    fn line(input: &[u8]) -> IResult<&[u8], super::Edge> {
        let (input, (src,_,dst,_)) =
            tuple((alpha1, tag("-"), alpha1, multispace0))(input)?;
        let src_string = String::from(std::str::from_utf8(src).unwrap());
        let dst_string = String::from(std::str::from_utf8(dst).unwrap());
        Ok((input, (src_string,dst_string)))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<super::Edge>> {
        let (input, l) = many1(line)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, l))
    }
}

fn add_edge(graph: &mut Graph, src: &str, dst: &str) {
    let v = match graph.get_mut(src) {
        Some(v) => v,
        None => {
            let v = Vec::new();
            graph.insert(src.to_string(), v);
            graph.get_mut(src).unwrap()
        }
    };
    v.push(dst.to_string());
}

fn build_graph(edges : Vec<Edge>) -> Graph {
    let mut graph : Graph = HashMap::new();
    for (src,dst) in edges {
        add_edge(&mut graph, &src, &dst);
        add_edge(&mut graph, &dst, &src);
    }
    graph
}

fn is_small_cave(v : &str) -> bool {
    let first_char : char = v.chars().next().unwrap();
    first_char.is_lowercase()
}

fn get_mark(marks : &Marks, v : &str) -> u32 {
    match marks.get(v) {
        Some(&i) => i,
        _ => 0
    }
}

fn is_marked(marks : &Marks, v : &str) -> bool {
    get_mark(marks, v) > 0
}

fn mark(marks : &mut Marks, v : &str) {
    if is_small_cave(v) {
        match marks.get_mut(v) {
            Some(i) => *i += 1,
            _ => {
                marks.insert(v.to_string(), 1);
            }
        }
    }
}

fn unmark(marks : &mut Marks, v : &str) {
    if is_small_cave(v) {
        if let Some(i) = marks.get_mut(v) {
            *i -= 1
        }
    }
}


fn dfs(graph : &Graph, marks : &mut Marks, v : &str, extra: bool, prefix: &mut Vec<String>) -> u32 {
    let marked = is_marked(marks, v);

    if marked && !extra {
        0
    }
    else {
        prefix.push(v.to_string());

        let r =
            if v == "end" {
                //println!("{}", prefix.join(","));
                1
            }
            else {
                let extra = extra && !marked;
                let mut sum = 0;
                mark(marks, v);

                if let Some(successors) = graph.get(v) {
                    for succ in successors {
                        if succ != "start" {
                            sum += dfs(graph, marks, succ, extra, prefix);
                        }
                    }
                }

                unmark(marks, v);
                sum
            };
        prefix.pop();
        r
    }
}

pub fn solve(input: &[u8]) -> (u32,u32) {
    let (_,edges) = parser::parse(input).unwrap();
    let graph = build_graph(edges);
    let mut marks = HashMap::new();
    let path_count1 = dfs(&graph, &mut marks, &String::from("start"), false, &mut Vec::new());
    let path_count2 = dfs(&graph, &mut marks, &String::from("start"), true, &mut Vec::new());
    (path_count1, path_count2)
}

#[test]
fn test12_0() {
    let solution = solve(include_bytes!("../inputs/day12.0"));
    assert_eq!(solution, (10,36));
}

#[test]
fn test12_1() {
    let solution = solve(include_bytes!("../inputs/day12.1"));
    assert_eq!(solution, (19,103));
}

#[test]
fn test12_2() {
    let solution = solve(include_bytes!("../inputs/day12.2"));
    assert_eq!(solution, (226,3509));
}

#[test]
fn test12_3() {
    let solution = solve(include_bytes!("../inputs/day12.3"));
    assert_eq!(solution, (5874,153592));
}
