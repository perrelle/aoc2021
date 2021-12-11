pub type Input = Vec<Vec<char>>;

mod parser  {
    use nom::{IResult, multi::*, character::complete::*, combinator::*};

    fn line(input: &[u8]) -> IResult<&[u8], Vec<char>> {
        many1(one_of("<>(){}[]"))(input)
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], super::Input> {
        let (input, l) = separated_list1(multispace1, line)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, l))
    }
}

fn matching_delimiter(&c : &char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!()
    }
}

fn char_error_score(c : char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!()
    }
}

fn char_completion_score(c : char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!()
    }
}

fn completion_score(v : &Vec<char>) -> u64 {
    let mut score = 0;
    for &c in v {
        score = 5 * score + (char_completion_score(c) as u64);
    }
    score
}

pub fn solve(data: &[u8]) -> (u32,u64) {
    let (_,input) = parser::parse(data).unwrap();

    let mut stack : Vec<char> = Vec::new();
    let mut syntax_score = 0;
    let mut completion_scores = Vec::new();

    for line in input {
        // --- Part 1 ---
        let mut correct = true;
        stack.clear();
        for &c in &line {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                },
                _ => {
                    if let Some(d) = stack.pop() {
                        let cexpected = matching_delimiter(&d);
                        if c == cexpected {
                            continue;
                        }
                        println!("Expected {}, but found {} instead.",
                            cexpected, c);
                    }
                    syntax_score += char_error_score(c);
                    correct = false;
                    break;
                }
            }
        }

        // --- Part 2 ---
        if correct && !stack.is_empty() {
            let completion =
                stack.iter().rev().map(matching_delimiter).collect();
            let completion_score = completion_score(&completion);
            completion_scores.push(completion_score);
            println!("{} - Complete by adding {} ({} points)",
                String::from_iter(line),
                String::from_iter(completion),
                completion_score);
        }
    }

    completion_scores.sort();
    let completion_score = completion_scores[completion_scores.len() / 2];

    println!("Total syntax error score: {}", syntax_score);
    println!("Middle completion score: {}", completion_score);
    (syntax_score, completion_score)
}

#[test]
fn test10_0() {
    let solution = solve(include_bytes!("../inputs/day10.0"));
    assert_eq!(solution, (26397,288957));
}

#[test]
fn test10_1() {
    let solution = solve(include_bytes!("../inputs/day10.1"));
    assert_eq!(solution, (294195,3490802734));
}
