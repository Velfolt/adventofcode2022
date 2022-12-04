use std::{io::Error, ops::RangeInclusive};

use crate::file::read_lines;

fn line_to_ranges(line: Result<String, Error>) -> Vec<RangeInclusive<i32>> {
    let groups: Vec<String> = line.unwrap().split(',').map(|x| x.to_string()).collect();
    groups
        .iter()
        .map(|group| {
            group
                .split('-')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|x| x[0]..=x[1])
        .collect::<Vec<RangeInclusive<i32>>>()
}

fn ranges_fully_contained(ranges: &Vec<RangeInclusive<i32>>) -> bool {
    let range1 = &ranges[0];
    let range2 = &ranges[1];

    if range1 == range2 {
        // equal
        true
    } else if range1.start() <= range2.start() && range1.end() >= range2.end() {
        // 1 is fully contained by 2
        true
    } else if range2.start() <= range1.start() && range2.end() >= range1.end() {
        // 2 is fully contained by 1
        true
    } else {
        false
    }
}

pub fn part_1() {
    let pairs = read_lines("day4.txt")
        .map(line_to_ranges)
        .filter(ranges_fully_contained)
        .count();

    println!("{:?}", pairs);
}

#[test]
fn test_part_1() {
    part_1()
}

fn ranges_overlap(ranges: &Vec<RangeInclusive<i32>>) -> bool {
    let range1 = &ranges[0];
    let range2 = ranges[1].clone();

    for x in range2 {
        if range1.contains(&x) {
            return true;
        }
    }

    false
}

pub fn part_2() {
    let pairs = read_lines("day4.txt")
        .map(line_to_ranges)
        .filter(ranges_overlap)
        .count();

    println!("{:?}", pairs);
}

#[test]
fn test_part_2() {
    part_2()
}
