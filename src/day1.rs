use crate::{
    file::read_lines, lines_as_numbers::StringToNumbersTrait, summed_groups::SummedGroupsTrait,
};

pub fn part_1() {
    let elves_max = read_lines("day1.txt")
        .as_numbers()
        .summed_groups()
        .max()
        .unwrap();

    println!("Heaviest elf: {:?}", elves_max);
}

pub fn part_2() {
    let mut summed_elves: Vec<i32> = read_lines("day1.txt")
        .as_numbers()
        .summed_groups()
        .collect();

    summed_elves.sort();

    let three_top = summed_elves.iter().rev().take(3).sum::<i32>();

    println!("Elves: {:?}", three_top);
}
