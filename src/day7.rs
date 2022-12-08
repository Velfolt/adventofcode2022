use indextree::{Arena, NodeId};

use crate::file::read_lines;

fn get_commands<I>(iter: I) -> Vec<(String, Vec<String>)>
where
    I: Iterator<Item = String>,
{
    let mut commands = vec![];
    let mut command: String = "".into();
    let mut result = vec![];

    for line in iter {
        if line.starts_with('$') {
            if !command.is_empty() {
                commands.push((command, result));
                result = vec![];
            }
            command = line;
        } else {
            result.push(line);
        }
    }

    if result.len() > 0 {
        commands.push((command, result));
    }

    commands
}

#[derive(Debug, Clone, PartialEq)]
enum FSItem {
    Dir(String),
    File(String, usize),
}

fn to_filesystem(arena: &mut Arena<FSItem>, commands: &Vec<(String, Vec<String>)>) -> NodeId {
    let root = arena.new_node(FSItem::Dir("/".to_string()));

    let mut node = root.ancestors(arena).next().unwrap();

    for (command, result) in commands {
        let command: Vec<_> = command.split_whitespace().take(3).collect();
        match command[1] {
            "cd" => {
                if command[2] == ".." {
                    node = arena.get(node).unwrap().parent().unwrap();
                } else if command[2] == "/" {
                } else {
                    let dir = node
                        .children(arena)
                        .find(|x| {
                            *arena.get(*x).unwrap().get() == FSItem::Dir(command[2].to_string())
                        })
                        .unwrap();

                    node = dir;
                }
            }
            "ls" => {
                for line in result
                    .iter()
                    .map(|line| line.split_whitespace().collect::<Vec<_>>())
                {
                    if line[0] == "dir" {
                        node.append(arena.new_node(FSItem::Dir(line[1].to_string())), arena);
                    } else {
                        node.append(
                            arena.new_node(FSItem::File(
                                line[1].to_string(),
                                line[0].parse().unwrap(),
                            )),
                            arena,
                        );
                    }
                }
            }
            _ => panic!("Unsupported command"),
        }
    }

    root
}

pub fn part_1() {
    let lines = read_lines("day7.txt").map(|x| x.unwrap());
    let commands = get_commands(lines);
    let mut arena = Arena::new();
    let filesystem = to_filesystem(&mut arena, &commands);

    let sum = filesystem
        .descendants(&arena)
        .filter(|x| {
            if let FSItem::Dir(_) = arena.get(*x).unwrap().get() {
                true
            } else {
                false
            }
        })
        .map(|node| {
            node.descendants(&arena)
                .map(|inner_node| {
                    if let FSItem::File(_, size) = arena.get(inner_node).unwrap().get() {
                        size
                    } else {
                        &0
                    }
                })
                .sum::<usize>()
        })
        .filter(|x| *x <= 100000)
        .sum::<usize>();

    println!("{:?}", sum)
}

#[test]
fn test_part_1() {
    part_1()
}

pub fn part_2() {
    let lines = read_lines("day7.txt").map(|x| x.unwrap());
    let commands = get_commands(lines);
    let mut arena = Arena::new();
    let filesystem = to_filesystem(&mut arena, &commands);

    let dirs: Vec<_> = filesystem
        .descendants(&arena)
        .filter(|x| {
            if let FSItem::Dir(_) = arena.get(*x).unwrap().get() {
                true
            } else {
                false
            }
        })
        .map(|node| {
            node.descendants(&arena)
                .map(|inner_node| {
                    if let FSItem::File(_, size) = arena.get(inner_node).unwrap().get() {
                        size
                    } else {
                        &0
                    }
                })
                .sum::<usize>()
        })
        .collect();

    let root_size = dirs.iter().max().unwrap();
    let unused = 70000000 - root_size;
    let minimum_freed = 30000000 - unused;

    let dir_size_to_remove = dirs.iter().filter(|x| **x >= minimum_freed).min().unwrap();

    println!("{:?}", dir_size_to_remove);
}

#[test]
fn test_part_2() {
    part_2()
}
