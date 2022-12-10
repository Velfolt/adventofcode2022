use std::fmt::Debug;

use crate::file::read_lines;

#[derive(Debug, Copy, Clone)]
struct CPU {
    x: i64,
}

#[derive(Debug)]
enum Instruction {
    AddX(i64),
    Noop,
}

impl Instruction {
    fn new(instruction: Vec<String>) -> Self {
        match instruction[0].as_str() {
            "addx" => Instruction::AddX(instruction[1].parse().unwrap()),
            "noop" => Instruction::Noop,
            _ => panic!("unknown instruction {:?}", instruction[0]),
        }
    }

    fn to_cycles(&self) -> Vec<Effect> {
        match self {
            Instruction::AddX(v) => vec![Effect::Noop, Effect::AddX(*v)],
            Instruction::Noop => vec![Effect::Noop],
        }
    }
}

#[derive(Debug)]
enum Effect {
    AddX(i64),
    Noop,
}

impl Effect {
    fn apply(&self, cpu: &mut CPU) {
        match self {
            Effect::AddX(v) => cpu.x += v,
            Effect::Noop => {}
        }
    }
}

fn instructions() -> Vec<Instruction> {
    read_lines("day10.txt")
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        })
        .map(Instruction::new)
        .collect()
}

fn cycles(instructions: Vec<Instruction>, mut cpu: CPU) -> Vec<(usize, CPU)> {
    let mut last_effect: Option<Effect> = None;

    instructions
        .iter()
        .map(|instr| instr.to_cycles())
        .flatten()
        .map(|effect| {
            if let Some(effect) = &last_effect {
                effect.apply(&mut cpu);
            }

            last_effect = Some(effect);
            cpu
        })
        .enumerate()
        .collect::<Vec<_>>()
}

pub fn part_1() {
    let cpu = CPU { x: 1 };
    let instructions = instructions();
    let cycles = cycles(instructions, cpu);

    let signal_strength: Vec<_> = (20..=220)
        .step_by(40)
        .map(|cycle| cycles[cycle - 1].1.x * cycle as i64)
        .collect();

    println!("{:?}", signal_strength.iter().sum::<i64>());
}

#[test]
fn test_part_1() {
    part_1()
}

pub fn part_2() {
    let cpu = CPU { x: 1 };
    let instructions = instructions();
    let cycles = cycles(instructions, cpu);

    for (cycle, cpu) in cycles {
        let sprite = (cpu.x - 1)..=(cpu.x + 1);
        let crt_pos = cycle as i64 % 40;

        if sprite.contains(&crt_pos) {
            print!("#")
        } else {
            print!(".")
        }

        if cycle % 40 == 39 {
            println!("");
        }
    }
}

#[test]
fn test_part_2() {
    part_2()
}
