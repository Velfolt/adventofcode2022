use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
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
    let row = grid.beaconless(10);
    println!("{:?}", row);
}

#[test]
fn test_part_1() {
    part_1();
}

fn cross_product(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 * b.1) - (a.1 * b.0)
}

fn line_from(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 - b.0, a.1 - b.1)
}

#[derive(Debug, PartialEq, Hash, Eq)]
enum Intersect {
    Colinear,
    Parallel,
    Intersect((i64, i64)),
    NoIntersect,
}

fn intersect(p: (i64, i64), p2: (i64, i64), q: (i64, i64), q2: (i64, i64)) -> Intersect {
    let r = line_from(p2, p);
    let s = line_from(q2, q);

    let numerator = cross_product(line_from(q, p), r);
    let denominator = cross_product(r, s);

    if numerator == 0 && denominator == 0 {
        return Intersect::Colinear;
    }

    if denominator == 0 {
        // parallel lines
        return Intersect::Parallel;
    }

    let u = numerator as f64 / denominator as f64;
    let t = cross_product(line_from(q, p), s) as f64 / denominator as f64;

    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        Intersect::Intersect((
            (p.0 as f64 + t * r.0 as f64).floor() as i64,
            (p.1 as f64 + t * r.1 as f64).floor() as i64,
        ))
    } else {
        Intersect::NoIntersect
    }
}

impl Grid<Entity> {
    fn hidden_beacon(&self, upper_limit: i64) -> Option<(i64, i64)> {
        let lines: Vec<_> = self
            .data
            .iter()
            .map(|(position, entity)| {
                if let Sensor(distance) = *entity {
                    let distance = distance + 1;
                    vec![
                        (
                            (position.0, position.1 + distance),
                            (position.0 - distance, position.1),
                        ),
                        (
                            (position.0 - distance, position.1),
                            (position.0, position.1 - distance),
                        ),
                        (
                            (position.0, position.1 - distance),
                            (position.0 + distance, position.1),
                        ),
                        (
                            (position.0 + distance, position.1),
                            (position.0, position.1 + distance),
                        ),
                    ]
                } else {
                    vec![]
                }
            })
            .flatten()
            .filter(|line| line.0 .0 < upper_limit && line.0 .0 > 0)
            .collect();

        // println!("{:?}", lines);

        let counts: _ = lines
            .clone()
            .iter()
            .map(|x| {
                lines
                    .clone()
                    .iter()
                    .map(|y| (intersect(x.0, x.1, y.0, y.1), (*x, *y)))
                    .filter(|(x, _)| {
                        if let Intersect::Intersect(_) = *x {
                            true
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<(_, _)>>()
            })
            .flatten()
            .counts_by(|a| a.0);

        /*for x in counts.iter().sorted_by(|a, b| a.1.cmp(b.1)) {
            println!("{:?}", x);
        }*/

        let max = counts.iter().max_by_key(|(_, count)| **count).unwrap();

        if let Intersect::Intersect(position) = *max.0 {
            Some(position)
        } else {
            None
        }
    }
}

pub fn part_2() {
    let grid: Grid<Entity> = input().into();

    let position = grid.hidden_beacon(4000000).unwrap();
    println!("{:?}", position);
    let tuning_frequency = position.0 * 4000000 + position.1;
    println!("{:?}", tuning_frequency);
}

#[test]
fn test_part_2() {
    part_2();
}
