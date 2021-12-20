use array2d::Array2D;

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Pixel(bool);
pub struct Image(Array2D<Pixel>);
pub struct Enhancement(Vec<Pixel>);

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", if self.0 {"#"} else {"."})
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row_iter in self.0.rows_iter() {
            for pixel in row_iter {
                write!(f, "{}", pixel)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl std::fmt::Display for Enhancement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for pixel in &self.0 {
            write!(f, "{}", pixel)?;
        }
        writeln!(f)
    }
}

mod parser {
    use nom::{IResult, character::complete::*, sequence::*, multi::*, combinator::*};
    use super::*;

    pub fn pixel(input: &[u8]) -> IResult<&[u8], Pixel> {
        let (input,c) = satisfy(|c| c == '.' || c == '#')(input)?;
        Ok((input, Pixel(c == '#')))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], (Enhancement, Image)> {
        let (input, (enhancement,_,image)) = tuple((
            count(pixel, 512),
            multispace1,
            separated_list1(multispace1, many1(pixel))))(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, (Enhancement(enhancement), Image(Array2D::from_rows(&image)))))
    }
}

pub fn pad(image: &Image, padding: usize) -> Image {
    let mut array = Array2D::filled_with(
        Pixel(false),
        image.0.num_rows() + padding * 2,
        image.0.num_columns() + padding * 2);

    for (i,row_iter) in image.0.rows_iter().enumerate() {
        for (j,pixel) in row_iter.enumerate() {
            let _ = array.set(i + padding, j + padding, *pixel);
        }
    }

    Image(array)
}

fn enhance(image: &Image, enhancement: &Enhancement, infinity: Pixel) -> Image {
    let padding = 1;
    let mut result = Array2D::filled_with(
        Pixel(false),
        image.0.num_rows() + padding * 2,
        image.0.num_columns() + padding * 2);

    for i in 0..result.num_rows() {
        for j in 0..result.num_columns() {
            let mut value = 0;
            for a in -1..=1 {
                for b in -1..=1 {
                    let row = (i as i32 + a - 1) as usize;
                    let col = (j as i32 + b - 1) as usize;
                    let pixel = image.0.get(row,col).unwrap_or(&infinity);
                    value = value * 2 + if pixel.0 {1} else {0};
                }
            }
            let new_pixel = enhancement.0.get(value).unwrap();
            let _ = result.set(i, j, *new_pixel);
        }
    }

    Image(result)
}

fn count_pixels(image: &Image) -> u32 {
    let mut c = 0;

    for row_iter in image.0.rows_iter() {
        for pixel in row_iter {
            if pixel.0 {
                c += 1;
            }
        }
    }
    c
}

pub fn solve(input: &[u8]) -> (u32,u32) {
    let (_,(enhancement,image)) = parser::parse(input).unwrap();

    let mut count1 = 0;
    let mut count2 = 0;
    let mut enhanced_image = image;
    let mut infinity = Pixel(false);

    for step in 1..=50 {
        enhanced_image = enhance(&enhanced_image, &enhancement, infinity);
        infinity = enhancement.0[if infinity.0 { 511 } else { 0 }];
        if enhanced_image.0.num_elements() < 400 {
            println!("After {} steps:\n{}", step, enhanced_image);
        }
        match step {
            2 => count1 = count_pixels(&enhanced_image),
            50 => count2 = count_pixels(&enhanced_image),
            _ => ()
        }
    }

    println!("{} pixels lit after 2 steps", count1);
    println!("{} pixels lit after 50 steps", count2);
    (count1,count2)
}

#[test]
fn test20_0() {
    let solution = solve(include_bytes!("../inputs/day20.0"));
    assert_eq!(solution, (35,3351));
}

#[test]
fn test20_1() {
    let solution = solve(include_bytes!("../inputs/day20.1"));
    assert_eq!(solution, (5218,15527));
}
