use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use itertools::Itertools;

use crate::file::read_lines;

#[derive(Debug)]
struct Map {
    map: Vec<char>,
    size: (usize, usize),
    start_pos: (i32, i32),
    end_pos: (i32, i32),
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                write!(f, "{}", self.get((x as i32, y as i32)).unwrap().1)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl From<Vec<Vec<char>>> for Map {
    fn from(map: Vec<Vec<char>>) -> Self {
        let size = (map[0].len(), map.len());

        let map = map.iter().flatten().map(|x| *x).collect::<Vec<char>>();
        let start_pos = map.iter().find_position(|x| **x == 'S').unwrap().0;
        let end_pos = map.iter().find_position(|x| **x == 'E').unwrap().0;

        let map = map
            .iter()
            .map(|x| match *x {
                'S' => 'a',
                'E' => 'z',
                _ => *x,
            })
            .collect();

        Map {
            map,
            size,
            start_pos: ((start_pos % size.0) as i32, (start_pos / size.0) as i32),
            end_pos: ((end_pos % size.0) as i32, (end_pos / size.0) as i32),
        }
    }
}

impl Map {
    fn get(&self, position: (i32, i32)) -> Option<((i32, i32), char)> {
        let index = (position.1 as i32 * self.size.0 as i32) + position.0 as i32;
        let x = position.0;
        let y = position.1;

        if x < 0 || y < 0 || x >= self.size.0 as i32 || y >= self.size.1 as i32 {
            None
        } else {
            Some((position, self.map[index as usize]))
        }
    }

    fn find_all(&self, elevation: char) -> Vec<(i32, i32)> {
        self.map
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == elevation)
            .map(|(index, _)| ((index % self.size.0) as i32, (index / self.size.0) as i32))
            .collect()
    }

    fn possible_adjacent(&self, position: (i32, i32)) -> Vec<((i32, i32), char)> {
        let current = self.get(position).unwrap();

        vec![
            self.get((position.0 + 1, position.1)),
            self.get((position.0 - 1, position.1)),
            self.get((position.0, position.1 + 1)),
            self.get((position.0, position.1 - 1)),
        ]
        .iter()
        .filter_map(|x| *x)
        .filter(|(_, value)| {
            (*value as i16) - (current.1 as i16) == 1 || (*value as i16) <= (current.1 as i16)
        })
        .collect()
    }

    fn _print_position(&self, position: (i32, i32)) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                if x == position.0 as usize && y == position.1 as usize {
                    print!("0");
                } else {
                    print!("{}", self.get((x as i32, y as i32)).unwrap().1);
                }
            }
            println!("");
        }
        println!("");
    }
}

#[test]
fn test_map() {
    let map: Map = vec![
        vec!['S', 'o', 'n', 'o', 'o', 'o', 'n'],
        vec!['b', 'o', 'n', 'o', 'o', 'o', 'n'],
        vec!['e', 'o', 'n', 'o', 'o', 'o', 'n'],
        vec!['E', 'o', 'o', 'o', 'o', 'o', 'n'],
    ]
    .into();
    println!("{:?}", map.possible_adjacent((1, 0)));
    println!("{:?}", map.possible_adjacent((1, 1)));
    println!("{:?}", map.possible_adjacent((1, 2)));
    println!("{:?}", map.possible_adjacent((1, 3)));
    println!("{:?}", map.possible_adjacent((4, 1)));
    println!("{:?}", map.possible_adjacent((5, 1)));
    println!("{:?}", map.find_all('o'));
}

fn map() -> Map {
    read_lines("day12.txt")
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .into()
}

fn bfs(map: &Map, start_position: (i32, i32)) -> Option<Vec<((i32, i32), char)>> {
    let mut queue = VecDeque::new();

    let mut explored = HashSet::new();
    explored.insert(start_position);
    queue.push_back((map.get(start_position).unwrap(), vec![]));

    while let Some(((position, value), parents)) = queue.pop_front() {
        if position == map.end_pos {
            return Some(parents);
        }

        for edge in map.possible_adjacent(position) {
            if !explored.contains(&edge.0) {
                explored.insert(edge.0);

                let mut parents = parents.clone();
                parents.push((position, value));
                queue.push_back((edge, parents));
            }
        }
    }

    None
}

pub fn part_1() {
    let map = map();

    let path = bfs(&map, map.start_pos).unwrap();

    println!("it took {}", path.len());
}

#[test]
fn test_part_1() {
    part_1()
}

pub fn part_2() {
    let map = map();

    let shortest_path = map
        .find_all('a')
        .iter()
        .filter_map(|position| bfs(&map, *position))
        .map(|path| path.len())
        .min()
        .unwrap();

    println!("shortest path is {}", shortest_path);
}

#[test]
fn test_part_2() {
    part_2()
}
