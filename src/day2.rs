use core::panic;
use std::io::Error;

use crate::file::read_lines;

#[derive(Debug, Clone, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Clone, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

fn line_to_hands(line: &Result<String, Error>) -> (Hand, Hand) {
    let hands: Vec<&str> = match line {
        Ok(it) => it,
        Err(_err) => panic!("should not happen"),
    }
    .split(' ')
    .collect();

    let left = match hands[0] {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissor,
        _ => panic!("should not happen"),
    };

    let right = match hands[1] {
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissor,
        _ => panic!("should not happen"),
    };

    (left, right)
}

#[test]
fn test_line_to_hands() {
    assert_eq!(
        line_to_hands(&Ok("A Y".to_string())),
        (Hand::Rock, Hand::Paper)
    );
    assert_eq!(
        line_to_hands(&Ok("B X".to_string())),
        (Hand::Paper, Hand::Rock)
    );
    assert_eq!(
        line_to_hands(&Ok("C Z".to_string())),
        (Hand::Scissor, Hand::Scissor)
    );
}

fn hands_to_outcome((other, my_hand): (Hand, Hand)) -> Outcome {
    match (my_hand, other) {
        (Hand::Rock, Hand::Scissor) => Outcome::Win,
        (Hand::Scissor, Hand::Paper) => Outcome::Win,
        (Hand::Paper, Hand::Rock) => Outcome::Win,
        (Hand::Scissor, Hand::Rock) => Outcome::Lose,
        (Hand::Paper, Hand::Scissor) => Outcome::Lose,
        (Hand::Rock, Hand::Paper) => Outcome::Lose,
        (Hand::Rock, Hand::Rock) => Outcome::Draw,
        (Hand::Paper, Hand::Paper) => Outcome::Draw,
        (Hand::Scissor, Hand::Scissor) => Outcome::Draw,
    }
}

fn hands_to_score((other, my_hand): (Hand, Hand)) -> i32 {
    let shape_score = match my_hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissor => 3,
    };

    let outcome_score = match hands_to_outcome((other, my_hand)) {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    };

    shape_score + outcome_score
}

pub fn part_1() {
    let total_score: i32 = read_lines("day2.txt")
        .map(|line| line_to_hands(&line))
        .map(hands_to_score)
        .sum();

    println!("{}", total_score);
}

fn line_to_hand_and_outcome(line: &Result<String, Error>) -> (Hand, Outcome) {
    let hand_and_outcome: Vec<&str> = match line {
        Ok(it) => it,
        Err(_err) => panic!("should not happen"),
    }
    .split(' ')
    .collect();

    let left = match hand_and_outcome[0] {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissor,
        _ => panic!("should not happen"),
    };

    let outcome = match hand_and_outcome[1] {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("should not happen"),
    };

    (left, outcome)
}

#[test]
fn test_line_to_hand_and_outcome() {
    assert_eq!(
        line_to_hand_and_outcome(&Ok("A Y".to_string())),
        (Hand::Rock, Outcome::Draw)
    );
    assert_eq!(
        line_to_hand_and_outcome(&Ok("B X".to_string())),
        (Hand::Paper, Outcome::Lose)
    );
    assert_eq!(
        line_to_hand_and_outcome(&Ok("C Z".to_string())),
        (Hand::Scissor, Outcome::Win)
    );
}

fn outcome_to_hands((other, outcome): (Hand, Outcome)) -> (Hand, Hand) {
    let my_hand = match (&other, &outcome) {
        (Hand::Rock, Outcome::Win) => Hand::Paper,
        (Hand::Rock, Outcome::Draw) => Hand::Rock,
        (Hand::Rock, Outcome::Lose) => Hand::Scissor,
        (Hand::Paper, Outcome::Win) => Hand::Scissor,
        (Hand::Paper, Outcome::Draw) => Hand::Paper,
        (Hand::Paper, Outcome::Lose) => Hand::Rock,
        (Hand::Scissor, Outcome::Win) => Hand::Rock,
        (Hand::Scissor, Outcome::Draw) => Hand::Scissor,
        (Hand::Scissor, Outcome::Lose) => Hand::Paper,
    };

    (other, my_hand)
}

pub fn part_2() {
    let total_score: i32 = read_lines("day2.txt")
        .map(|line| line_to_hand_and_outcome(&line))
        .map(outcome_to_hands)
        .map(hands_to_score)
        .sum();

    println!("{}", total_score);
}
