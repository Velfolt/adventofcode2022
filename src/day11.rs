use std::fmt::Debug;

use itertools::Itertools;
use regex::{Match, Regex};

use crate::file::read_lines;

#[derive(Debug, Clone)]
enum Operation {
    MultiplyOld,
    Multiply(i64),
    Add(i64),
}

impl From<(Option<Match<'_>>, Option<Match<'_>>)> for Operation {
    fn from((operation, value): (Option<Match<'_>>, Option<Match<'_>>)) -> Self {
        if value.unwrap().as_str() == "old" {
            return Operation::MultiplyOld;
        }

        match operation.unwrap().as_str() {
            "*" => Operation::Multiply(value.unwrap().as_str().parse().unwrap()),
            "+" => Operation::Add(value.unwrap().as_str().parse().unwrap()),
            _ => panic!("unsupported operation"),
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: (i64, usize, usize),
}

trait Inspect {
    fn items(&mut self) -> &mut Vec<i64>;
    fn operate(&self, n: usize, factors: i64) -> (usize, i64);
}

impl Inspect for Monkey {
    fn operate(&self, n: usize, _factors: i64) -> (usize, i64) {
        let mut item = self.items[n];

        match self.operation {
            Operation::MultiplyOld => item *= item,
            Operation::Multiply(x) => item *= x,
            Operation::Add(x) => item += x,
        }

        item /= 3;

        if item % self.test.0 == 0 {
            (self.test.1, item)
        } else {
            (self.test.2, item)
        }
    }

    fn items(&mut self) -> &mut Vec<i64> {
        &mut self.items
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

trait MonkeyTrait {
    fn throw_item_to(&mut self, n: usize, from: usize, to: usize, value: i64);
}

impl<T> MonkeyTrait for Vec<T>
where
    T: Inspect,
{
    fn throw_item_to(&mut self, n: usize, from: usize, to: usize, value: i64) {
        let from_monkey = &mut self[from];
        from_monkey.items().remove(n);
        self[to].items().push(value);
    }
}

fn monkeys() -> Vec<Monkey> {
    let regex = Regex::new(r"Monkey \d+:\n  Starting items: (.*)\n  Operation: new = old ([+*]) (.+)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)").unwrap();

    read_lines("day11.txt")
        .map(|line| line.unwrap())
        .chunks(7)
        .into_iter()
        .map(|mut chunk| {
            let definition = chunk.join("\n");

            let captures = regex.captures(definition.as_str()).unwrap();

            Monkey {
                items: captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|i| i.parse().unwrap())
                    .collect_vec(),
                operation: (captures.get(2), captures.get(3)).into(),
                test: (
                    captures.get(4).unwrap().as_str().parse().unwrap(),
                    captures.get(5).unwrap().as_str().parse().unwrap(),
                    captures.get(6).unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        .collect()
}

fn round<T>(monkeys: &mut Vec<T>, factors: i64) -> Vec<usize>
where
    T: Inspect + Clone + Debug,
{
    let mut inspect = vec![];

    for monkey_index in 0..monkeys.len() {
        let mut monkey = monkeys[monkey_index].clone();
        inspect.push(monkey.items().len());

        for index in 0..monkey.items().len() {
            let (throw_to, value) = monkey.operate(index, factors);

            monkeys.throw_item_to(0, monkey_index, throw_to, value);
        }
    }

    inspect
}

pub fn part_1() {
    let mut monkeys = monkeys();

    let inspections: _ = (0..20)
        .map(|_| round(&mut monkeys, 1))
        .reduce(|acc, value| {
            acc.iter()
                .enumerate()
                .map(|(index, val)| value[index] + val)
                .collect()
        })
        .unwrap();

    let product: usize = inspections.iter().sorted().rev().take(2).product();

    println!("{:?}", product);
}

#[test]
fn test_part_1() {
    part_1()
}

#[derive(Clone)]
struct MonkeyWithoutRelief {
    items: Vec<i64>,
    operation: Operation,
    test: (i64, usize, usize),
}

impl Debug for MonkeyWithoutRelief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

impl Inspect for MonkeyWithoutRelief {
    fn operate(&self, n: usize, factors: i64) -> (usize, i64) {
        let mut item = self.items[n];

        match self.operation {
            Operation::MultiplyOld => item *= item,
            Operation::Multiply(x) => item *= x,
            Operation::Add(x) => item += x,
        }

        item %= factors;

        if item % self.test.0 == 0 {
            (self.test.1, item)
        } else {
            (self.test.2, item)
        }
    }

    fn items(&mut self) -> &mut Vec<i64> {
        &mut self.items
    }
}

pub fn part_2() {
    let mut monkeys: Vec<MonkeyWithoutRelief> = monkeys()
        .iter()
        .map(|monkey| MonkeyWithoutRelief {
            items: monkey.items.clone(),
            operation: monkey.operation.clone(),
            test: monkey.test,
        })
        .collect();

    let factors = monkeys.iter().map(|monkey| monkey.test.0).product::<i64>();

    let inspections: _ = (0..10000)
        .map(|_| round(&mut monkeys, factors))
        .reduce(|acc, value| {
            acc.iter()
                .enumerate()
                .map(|(index, val)| value[index] + val)
                .collect()
        })
        .unwrap();

    let product: usize = inspections.iter().sorted().rev().take(2).product();

    println!("{:?}", product);
}

#[test]
fn test_part_2() {
    part_2()
}
