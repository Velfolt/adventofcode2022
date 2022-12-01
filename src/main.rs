use std::{
    fs::File,
    io::{self, BufRead},
};

mod lines_as_numbers;
mod summed_groups;

use crate::lines_as_numbers::StringToNumbersTrait;
use crate::summed_groups::summed_groups;

fn december1_part_1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let calories = io::BufReader::new(file).lines().as_numbers().collect();
    let elves_max = summed_groups(&calories).max();

    println!("Heaviest elf: {:?}", elves_max);

    Ok(())
}

fn december1_part_2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let calories = io::BufReader::new(file).lines().as_numbers().collect();
    let mut summed_elves = summed_groups(&calories).collect::<Vec<i32>>();

    summed_elves.sort();

    let three_top = summed_elves.iter().rev().take(3).sum::<i32>();

    println!("Elves: {:?}", three_top);

    Ok(())
}

fn main() -> io::Result<()> {
    december1_part_1()?;
    december1_part_2()
}
