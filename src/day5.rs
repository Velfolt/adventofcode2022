use regex::Regex;

use crate::{file::read_lines, group_by::GroupByTrait};

pub fn part_1() {
    let mut lines = read_lines("day5.txt");

    let mut crates: Vec<_> = lines
        .by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| line != "")
        .map(|line| {
            line.chars()
                .group_by(4)
                .map(|x| x[1])
                .collect::<Vec<char>>()
        })
        .fold(vec![], |acc, val| {
            let mut acc = acc;

            for (index, val) in val.into_iter().enumerate() {
                if val != ' ' && val as u8 > 57 {
                    while acc.len() <= index {
                        acc.push(vec![])
                    }

                    acc[index].push(val)
                }
            }
            acc
        })
        .into_iter()
        .map(|x| x.into_iter().rev().collect::<Vec<char>>())
        .collect();

    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let procedure: Vec<_> = lines
        .map(|line| line.unwrap())
        .map(|line| {
            let caps = regex.captures(&line).unwrap();

            (
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    for (n, from, to) in procedure {
        for _ in 0..n {
            let element = crates[from].pop().unwrap();
            crates[to].push(element);
        }
    }

    let top_of_stack = crates
        .iter()
        .map(|x| x.last().unwrap_or(&' '))
        .collect::<String>();

    println!("{:?}", top_of_stack.trim());
}

#[test]
fn test_part1() {
    part_1();
}

pub fn part_2() {
    let mut lines = read_lines("day5.txt");

    let mut crates: Vec<_> = lines
        .by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| line != "")
        .map(|line| {
            line.chars()
                .group_by(4)
                .map(|x| x[1])
                .collect::<Vec<char>>()
        })
        .fold(vec![], |acc, val| {
            let mut acc = acc;

            for (index, val) in val.into_iter().enumerate() {
                if val != ' ' && val as u8 > 57 {
                    while acc.len() <= index {
                        acc.push(vec![])
                    }

                    acc[index].push(val)
                }
            }
            acc
        })
        .into_iter()
        .map(|x| x.into_iter().rev().collect::<Vec<char>>())
        .collect();

    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let procedure: Vec<_> = lines
        .map(|line| line.unwrap())
        .map(|line| {
            let caps = regex.captures(&line).unwrap();

            (
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    for (n, from, to) in procedure {
        let mut vec = vec![];
        for _ in 0..n {
            vec.push(crates[from].pop().unwrap());
        }

        vec.reverse();

        for c in vec {
            crates[to].push(c);
        }
    }

    let top_of_stack = crates
        .iter()
        .map(|x| x.last().unwrap_or(&' '))
        .collect::<String>();

    println!("{:?}", top_of_stack.trim());
}

#[test]
fn test_part2() {
    part_2();
}
