use std::{collections::HashMap, fmt::Display};

use regex::Regex;

use crate::{file::read_lines, grid::Grid};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Entity {
    Air,
    Sensor(i64),
    Beacon,
}

impl Default for Entity {
    fn default() -> Self {
        Air
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Air => write!(f, "."),
            Sensor(_) => write!(f, "S"),
            Beacon => write!(f, "B"),
        }
    }
}

use Entity::*;

fn input() -> Vec<((i64, i64), (i64, i64))> {
    let regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    read_lines("day15.txt")
        .map(|line| line.unwrap())
        .map(|line| {
            regex
                .captures_iter(line.as_str())
                .map(|x| {
                    (
                        (
                            x.get(1).unwrap().as_str().parse().unwrap(),
                            x.get(2).unwrap().as_str().parse().unwrap(),
                        ),
                        (
                            x.get(3).unwrap().as_str().parse().unwrap(),
                            x.get(4).unwrap().as_str().parse().unwrap(),
                        ),
                    )
                })
                .collect::<Vec<((i64, i64), (i64, i64))>>()
        })
        .flatten()
        .collect()
}

#[test]
fn test_input() {
    let input = input();
    println!("{:?}", input);
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    a.0.abs_diff(b.0) as i64 + a.1.abs_diff(b.1) as i64
}

impl From<Vec<((i64, i64), (i64, i64))>> for Grid<Entity> {
    fn from(positions: Vec<((i64, i64), (i64, i64))>) -> Self {
        let mut data = HashMap::new();

        for (sensor, beacon) in positions {
            let distance = manhattan_distance(sensor, beacon);

            data.insert(sensor, Sensor(distance));
            data.insert(beacon, Beacon);
        }

        Grid { data }
    }
}

impl Grid<Entity> {
    fn beaconless(&self, y: i64) -> i64 {
        let mut count = 0;
        for x in self.x_range() {
            let mut in_sensor = false;
            for (position, entity) in self.data.iter() {
                if let Sensor(distance) = *entity {
                    if manhattan_distance(*position, (x, y)) <= distance {
                        in_sensor = true;
                        break;
                    }
                }
            }

            if in_sensor {
                if let Some(Beacon) = self.data.get(&(x, y)) {
                } else {
                    count += 1;
                }
            }
        }

        count
    }
}

pub fn part_1() {
    let grid: Grid<Entity> = input().into();
    let row = grid.beaconless(2000000);
    println!("{:?}", row);
}

#[test]
fn test_part_1() {
    part_1();
}

impl Grid<Entity> {
    fn hidden_beacon(&self, upper_limit: i64) -> Option<(i64, i64)> {
        let count = self.x_range().count() as i64;

        println!("{:?}", count);


        for y in 0..upper_limit {
            if count - self.beaconless(y) > 1 {
                continue
            }


            for x in 0..upper_limit {
                let mut in_sensor = false;
                for (position, entity) in self.data.iter() {
                    if let Sensor(distance) = *entity {
                        if manhattan_distance(*position, (x, y)) <= distance {
                            in_sensor = true;
                            break;
                        }
                    }
                }

                if !in_sensor {
                    return Some((x, y));
                }
            }
        }

        None
    }
}

pub fn part_2() {
    let grid: Grid<Entity> = input().into();

    let position = grid.hidden_beacon(20).unwrap();
    let tuning_frequency = position.0 * 4000000 + position.1;
    println!("{:?}", tuning_frequency);
}

#[test]
fn test_part_2() {
    part_2();
}
