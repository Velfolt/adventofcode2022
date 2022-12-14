use itertools::Itertools;
use serde_json::{json, Value};

use crate::file::read_lines;

fn packets() -> Vec<Value> {
    read_lines("day13.txt")
        .map(|line| line.unwrap())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .take(2)
                .map(|x| serde_json::from_str(x.as_str()).unwrap())
                .collect::<Vec<Value>>()
        })
        .flatten()
        .collect()
}

#[derive(Debug, PartialEq)]
enum Compare {
    RightOrder,
    WrongOrder,
    Continue,
}

use Compare::*;

fn compare_nodes(left: &Value, right: &Value) -> Compare {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            let left = left.as_i64().unwrap();
            let right = right.as_i64().unwrap();

            if left < right {
                RightOrder
            } else if left > right {
                WrongOrder
            } else {
                Continue
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            for x in left.iter().zip_longest(right) {
                let result = match x {
                    itertools::EitherOrBoth::Both(left, right) => compare_nodes(left, right),
                    itertools::EitherOrBoth::Left(_) => WrongOrder,
                    itertools::EitherOrBoth::Right(_) => RightOrder,
                };

                if result != Continue {
                    return result;
                }
            }

            Continue
        }
        (Value::Number(left), Value::Array(right)) => {
            let left = Value::Array(vec![Value::Number(left.clone())]);
            let right = Value::Array(right.clone());

            compare_nodes(&left, &right)
        }
        (Value::Array(left), Value::Number(right)) => {
            let left = Value::Array(left.clone());
            let right = Value::Array(vec![Value::Number(right.clone())]);

            compare_nodes(&left, &right)
        }
        _ => panic!("unsupported type"),
    }
}

pub fn part_1() {
    let packets = packets();

    let compared: Vec<_> = packets
        .iter()
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.take(2).collect_tuple().unwrap())
        .map(|(left, right)| compare_nodes(left, right))
        .zip(1..)
        .collect();

    println!(
        "{:?}",
        compared
            .iter()
            .filter(|(compare, _)| *compare == RightOrder)
            .map(|(_, index)| index)
            .sum::<i64>()
    );
}

#[test]
fn test_part_1() {
    part_1();
}

pub fn part_2() {
    let mut packets = packets();
    packets.push(json!([[2]]));
    packets.push(json!([[6]]));

    let ordered: Vec<_> = packets
        .iter()
        .sorted_by(|left, right| match compare_nodes(*left, *right) {
            RightOrder => std::cmp::Ordering::Less,
            WrongOrder => std::cmp::Ordering::Greater,
            Continue => std::cmp::Ordering::Equal,
        })
        .zip(1..)
        .collect();

    println!(
        "{:?}",
        ordered
            .iter()
            .filter(|(val, _)| match val.to_string().as_str() {
                "[[2]]" | "[[6]]" => true,
                _ => false,
            })
            .map(|(_, index)| index)
            .product::<i64>()
    );
}

#[test]
fn test_part_2() {
    part_2();
}
