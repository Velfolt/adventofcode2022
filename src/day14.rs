use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Display,
    ops::{Bound, RangeInclusive},
};

use regex::Regex;

use crate::file::read_lines;

fn rocks() -> Vec<Vec<(i64, i64)>> {
    let regex = Regex::new(r"((\d+),(\d+))( -> )?").unwrap();

    read_lines("day14.txt")
        .map(|line| line.unwrap())
        .map(|line| {
            regex
                .captures_iter(line.as_str())
                .map(|x| {
                    (
                        x.get(2).unwrap().as_str().parse().unwrap(),
                        x.get(3).unwrap().as_str().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

#[test]
fn test_rocks() {
    let rocks = rocks();

    println!("{:?}", rocks);
}

#[derive(PartialEq, Clone, Copy)]
enum Entity {
    Air,
    Rock,
    Sand,
    SandSource,
}

use Entity::*;

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Air => write!(f, "."),
            Entity::Rock => write!(f, "#"),
            Entity::Sand => write!(f, "o"),
            Entity::SandSource => write!(f, "+"),
        }
    }
}

struct Cave {
    cave: HashMap<(i64, i64), Entity>,
}

impl Cave {
    fn max_y(&self) -> i64 {
        self.cave.keys().map(|(_, y)| *y).max().unwrap()
    }

    fn x_range(&self) -> RangeInclusive<i64> {
        let keys = self.cave.keys().map(|(x, _)| *x);

        keys.clone().min().unwrap()..=keys.clone().max().unwrap()
    }

    fn count_sand(&self) -> usize {
        self.cave.values().filter(|value| **value == Sand).count()
    }

    fn simulate(
        &mut self,
        sand_source: (i64, i64),
        end_condition: &dyn Fn(&(i64, i64), &HashMap<(i64, i64), Entity>) -> bool,
    ) -> Option<()> {
        let mut sand_position = (sand_source.0, sand_source.1);

        loop {
            if end_condition(&sand_position, &self.cave) {
                return None;
            }

            let below_blocked = self
                .cave
                .contains_key(&(sand_position.0, sand_position.1 + 1));
            let below_left_blocked = self
                .cave
                .contains_key(&(sand_position.0 - 1, sand_position.1 + 1));
            let below_right_blocked = self
                .cave
                .contains_key(&(sand_position.0 + 1, sand_position.1 + 1));

            if below_blocked {
                if below_left_blocked {
                    if below_right_blocked {
                        break;
                    } else {
                        sand_position.0 += 1;
                    }
                } else {
                    sand_position.0 -= 1;
                }
            }

            sand_position.1 += 1;
        }

        self.cave.insert(sand_position, Sand);
        Some(())
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_range = self.x_range();
        let y_range = 0..=self.max_y();

        for y in y_range.clone() {
            for x in x_range.clone() {
                write!(f, "{}", self.cave.get(&(x, y)).map_or(Air, |e| *e))?;
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn range_inclusive(a: i64, b: i64) -> impl Iterator<Item = i64> {
    let x: Box<dyn Iterator<Item = i64>>;
    if b > a {
        x = Box::new(a..=b)
    } else {
        x = Box::new((b..=a).rev())
    }
    x
}

impl From<Vec<Vec<(i64, i64)>>> for Cave {
    fn from(rocks: Vec<Vec<(i64, i64)>>) -> Self {
        let mut cave = HashMap::new();

        for rock in rocks {
            for window in rock.windows(2) {
                let (from, to) = if let &[from, to] = window {
                    (from, to)
                } else {
                    panic!()
                };

                for y in range_inclusive(from.1, to.1) {
                    for x in range_inclusive(from.0, to.0) {
                        cave.insert((x, y), Rock);
                    }
                }
            }
        }

        cave.insert((500, 0), SandSource);

        Cave { cave }
    }
}

pub fn part_1() {
    let rocks = rocks();
    let mut cave: Cave = rocks.into();

    let max_y = cave.max_y();

    while let Some(_) = cave.simulate((500, 0), &|sand_position, _| sand_position.1 > max_y) {}
    println!("{}\n", cave);

    println!("{}", cave.count_sand())
}

#[test]
fn test_part_1() {
    part_1();
}

pub fn part_2() {
    let rocks = rocks();
    let mut cave: Cave = rocks.into();

    let floor = cave.max_y() + 2;

    for x in 300..700 {
        cave.cave.insert((x, floor), Rock);
    }

    while let Some(_) = cave.simulate((500, 0), &|_, cave| *cave.get(&(500, 0)).unwrap() == Sand) {}
    println!("{}\n", cave);

    println!("{}", cave.count_sand())
}

#[test]
fn test_part_2() {
    part_2();
}
