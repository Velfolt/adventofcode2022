use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use crate::file::read_lines;

pub fn part_1() {
    let input: Vec<_> = read_lines("day8.txt")
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect();

    let length = input.len() as f32;

    let size: (usize, usize) = (length.sqrt() as usize, length.sqrt() as usize);

    let mut count = (size.0 * size.1) - ((size.0 - 2) * (size.1 - 2));

    for y in 1..(size.1 - 1) {
        for x in 1..(size.0 - 1) {
            let index = y * size.0 + x;
            let height = input[index];

            let left = (0..x)
                .map(|x| y * size.0 + x)
                .map(|index| input[index])
                .max()
                .unwrap();
            let right = ((x + 1)..size.0)
                .map(|x| y * size.0 + x)
                .map(|index| input[index])
                .max()
                .unwrap();
            let top = (0..y)
                .map(|y| y * size.0 + x)
                .map(|index| input[index])
                .max()
                .unwrap();
            let bottom = ((y + 1)..size.1)
                .map(|y| y * size.0 + x)
                .map(|index| input[index])
                .max()
                .unwrap();

            if left < height || right < height || top < height || bottom < height {
                count += 1;
            }
        }
    }

    println!("{:?}", count);
}

#[test]
fn test_part_1() {
    part_1()
}

pub fn part_2() {
    let input: Vec<_> = read_lines("day8.txt")
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect();

    let length = input.len() as f32;

    let size: (usize, usize) = (length.sqrt() as usize, length.sqrt() as usize);

    let mut score = vec![];

    for y in 0..size.1 {
        for x in 0..size.0 {
            let index = y * size.0 + x;
            let height = input[index];

            let left: _ = (0..x)
                .rev()
                .map(|x| y * size.0 + x)
                .map(|index| input[index])
                .fold_while(0, |acc, tree| {
                    if tree < height {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();
            let right: _ = ((x + 1)..size.0)
                .map(|x| y * size.0 + x)
                .map(|index| input[index])
                .fold_while(0, |acc, tree| {
                    if tree < height {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();
            let top: _ = (0..y)
                .rev()
                .map(|y| y * size.0 + x)
                .map(|index| input[index])
                .fold_while(0, |acc, tree| {
                    if tree < height {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();
            let bottom: _ = ((y + 1)..size.1)
                .map(|y| y * size.0 + x)
                .map(|index| input[index])
                .fold_while(0, |acc, tree| {
                    if tree < height {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();

            let scenic_score = left * right * top * bottom;

            score.push(scenic_score);
        }
    }

    let max_score = score.iter().max().unwrap();
    println!("{:?}", max_score);
}

#[test]
fn test_part_2() {
    part_2()
}
