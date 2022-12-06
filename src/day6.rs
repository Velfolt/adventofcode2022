use std::collections::HashSet;

use crate::file::read_lines;

pub fn part_1() {
    let lines = read_lines("day6.txt").map(|x| x.unwrap());

    for line in lines {
        for (index, window) in line.as_bytes().windows(4).enumerate() {
            let mut hash = HashSet::new();
            for element in window {
                hash.insert(*element);
            }

            if hash.len() == 4 {
                println!("{:?}", index + 4);
                break;
            }
        }
    }
}

#[test]
fn test_part_1() {
    part_1();
}

pub fn part_2() {
    let lines = read_lines("day6.txt").map(|x| x.unwrap());

    for line in lines {
        for (index, window) in line.as_bytes().windows(14).enumerate() {
            let mut hash = HashSet::new();
            for element in window {
                hash.insert(*element);
            }

            if hash.len() == 14 {
                println!("{:?}", index + 14);
                break;
            }
        }
    }
}

#[test]
fn test_part_2() {
    part_2();
}
