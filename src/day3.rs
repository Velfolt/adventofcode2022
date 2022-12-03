use std::io::Error;

use crate::file::read_lines;

pub fn part_1() {
    let sum = read_lines("day3.txt")
        .map(line_to_halves)
        .map(find_common_char)
        .map(char_to_priority)
        .sum::<i32>();

    println!("{:?}", sum);
}

fn char_to_priority(char: Option<char>) -> i32 {
    let ascii = char.unwrap() as i32;

    match ascii {
        97..=122 => ascii - 97 + 1,
        65..=90 => ascii - 65 + 27,
        _ => panic!("should not happen"),
    }
}

fn line_to_halves(line: Result<String, Error>) -> (String, String) {
    let line = line.unwrap().clone();
    let (left, right) = line.split_at(line.len() / 2);

    (left.to_string(), right.to_string())
}

#[test]
fn test_line_to_halves() {
    assert_eq!(
        line_to_halves(Ok("vJrwpWtwJgWrhcsFMMfFFhFp".to_string())),
        ("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string())
    );
}

fn find_common_char((left, right): (String, String)) -> Option<char> {
    for left_char in left.chars() {
        if let Some(_) = right.find(|x| x == left_char) {
            return Some(left_char);
        }
    }

    None
}

#[test]
fn test_find_common_char() {
    assert_eq!(
        find_common_char(("hej".to_string(), "fest".to_string())),
        Some('e')
    );
}

struct ThreeLines<I> {
    iter: I,
}

impl<I> Iterator for ThreeLines<I>
where
    I: Iterator<Item = Result<String, Error>>,
{
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let three = self.iter.by_ref().take(3).collect::<Vec<I::Item>>();

        if three.len() == 0 {
            return None;
        }

        Some(
            three
                .into_iter()
                .map(|x| x.unwrap())
                .collect::<Vec<String>>(),
        )
    }
}

trait ThreeLinesTrait<I> {
    fn three_lines(self) -> ThreeLines<I>;
}

impl<I> ThreeLinesTrait<I> for I
where
    I: Iterator,
{
    fn three_lines(self) -> ThreeLines<I> {
        ThreeLines { iter: self }
    }
}

fn common_char_in_vec(vec: Vec<String>) -> Option<char> {
    let a = &vec[0];
    let b = &vec[1];
    let c = &vec[2];

    for char in a.chars() {
        if let Some(_) = b.find(|c| c == char) {
            if let Some(_) = c.find(|c| c == char) {
                return Some(char);
            }
        }
    }

    None
}

pub fn part_2() {
    let sum = read_lines("day3.txt")
        .three_lines()
        .map(common_char_in_vec)
        .map(char_to_priority)
        .sum::<i32>();

    println!("{:?}", sum);
}
