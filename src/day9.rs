use std::collections::HashSet;

use itertools::Itertools;

use crate::file::read_lines;

fn distance_to(head_position: (i32, i32), tail_position: (i32, i32)) -> f32 {
    ((tail_position.0 as f32 - head_position.0 as f32).powf(2.0)
        + (tail_position.1 as f32 - head_position.1 as f32).powf(2.0))
    .sqrt()
}

fn update_tail(head_position: (i32, i32), tail_position: (i32, i32)) -> (i32, i32) {
    let distance = distance_to(head_position, tail_position);

    if distance <= 1.5 {
        return tail_position;
    }

    if distance_to(head_position, (tail_position.0 + 1, tail_position.1)) <= 1.0 {
        (tail_position.0 + 1, tail_position.1)
    } else if distance_to(head_position, (tail_position.0 - 1, tail_position.1)) <= 1.0 {
        (tail_position.0 - 1, tail_position.1)
    } else if distance_to(head_position, (tail_position.0, tail_position.1 + 1)) <= 1.0 {
        (tail_position.0, tail_position.1 + 1)
    } else if distance_to(head_position, (tail_position.0, tail_position.1 - 1)) <= 1.0 {
        (tail_position.0, tail_position.1 - 1)
    } else if distance_to(head_position, (tail_position.0 + 1, tail_position.1 + 1)) <= 1.5 {
        (tail_position.0 + 1, tail_position.1 + 1)
    } else if distance_to(head_position, (tail_position.0 + 1, tail_position.1 - 1)) <= 1.5 {
        (tail_position.0 + 1, tail_position.1 - 1)
    } else if distance_to(head_position, (tail_position.0 - 1, tail_position.1 - 1)) <= 1.5 {
        (tail_position.0 - 1, tail_position.1 - 1)
    } else if distance_to(head_position, (tail_position.0 - 1, tail_position.1 + 1)) <= 1.5 {
        (tail_position.0 - 1, tail_position.1 + 1)
    } else {
        panic!("{:?} should not happen", (tail_position.0, tail_position.1));
    }
}

fn directions() -> Vec<(String, i32)> {
    read_lines("day9.txt")
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        })
        .map(|direction| (direction[0].clone(), direction[1].parse::<i32>().unwrap()))
        .collect()
}

pub fn part_1() {
    let directions = directions();

    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);

    let mut visited = HashSet::new();

    for (direction, length) in directions {
        match direction.as_str() {
            "R" => {
                for _ in 0..length {
                    head_position.0 += 1;
                    tail_position = update_tail(head_position, tail_position);
                    visited.insert(tail_position);
                }
            }
            "L" => {
                for _ in 0..length {
                    head_position.0 -= 1;
                    tail_position = update_tail(head_position, tail_position);
                    visited.insert(tail_position);
                }
            }
            "U" => {
                for _ in 0..length {
                    head_position.1 += 1;
                    tail_position = update_tail(head_position, tail_position);
                    visited.insert(tail_position);
                }
            }
            "D" => {
                for _ in 0..length {
                    head_position.1 -= 1;
                    tail_position = update_tail(head_position, tail_position);
                    visited.insert(tail_position);
                }
            }
            _ => panic!("direction not supported"),
        }
    }

    println!("{:?}", visited.len());
}

#[test]
fn test_part_1() {
    part_1()
}

pub fn part_2() {
    let directions = directions();

    let mut head_position = (0, 0);
    let mut rope = vec![(0, 0); 10];

    let mut visited = HashSet::new();

    for (direction, length) in directions {
        match direction.as_str() {
            "R" => {
                for _ in 0..length {
                    head_position.0 += 1;
                    rope[0] = head_position;

                    for (prev, current) in (0..10).tuple_windows() {
                        rope[current] = update_tail(rope[prev], rope[current]);
                    }
                    visited.insert(rope.last().unwrap().clone());
                }
            }
            "L" => {
                for _ in 0..length {
                    head_position.0 -= 1;
                    rope[0] = head_position;

                    for (prev, current) in (0..10).tuple_windows() {
                        rope[current] = update_tail(rope[prev], rope[current]);
                    }
                    visited.insert(rope.last().unwrap().clone());
                }
            }
            "U" => {
                for _ in 0..length {
                    head_position.1 += 1;
                    rope[0] = head_position;

                    for (prev, current) in (0..10).tuple_windows() {
                        rope[current] = update_tail(rope[prev], rope[current]);
                    }
                    visited.insert(rope.last().unwrap().clone());
                }
            }
            "D" => {
                for _ in 0..length {
                    head_position.1 -= 1;
                    rope[0] = head_position;

                    for (prev, current) in (0..10).tuple_windows() {
                        rope[current] = update_tail(rope[prev], rope[current]);
                    }
                    visited.insert(rope.last().unwrap().clone());
                }
            }
            _ => panic!("direction not supported"),
        }
    }

    println!("{:?}", visited.len());
}

#[test]
fn test_part_2() {
    part_2()
}
