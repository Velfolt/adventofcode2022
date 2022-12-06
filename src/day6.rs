use std::collections::HashSet;

use crate::file::read_lines;

fn find_marker(n: usize, bytes: &[u8]) -> Option<usize> {
    for (index, window) in bytes.windows(n).enumerate() {
        let mut hash = HashSet::new();
        for element in window {
            hash.insert(*element);
        }

        if hash.len() == n {
            return Some(index + n);
        }
    }

    None
}

pub fn part_1() {
    let lines = read_lines("day6.txt").map(|x| x.unwrap());

    for line in lines {
        if let Some(marker) = find_marker(4, &line.as_bytes()) {
            println!("{:?}", marker);
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
        if let Some(marker) = find_marker(14, &line.as_bytes()) {
            println!("{:?}", marker);
        }
    }
}

#[test]
fn test_part_2() {
    part_2();
}
