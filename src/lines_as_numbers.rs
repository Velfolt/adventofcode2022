use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub struct LinesAsNumbers {
    iter: Lines<BufReader<File>>,
}

impl Iterator for LinesAsNumbers {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.iter.next() {
            if let Ok(line) = line {
                return Some(line.parse::<Self::Item>().map_or_else(|_| 0, |x| x));
            }
        }

        None
    }
}

pub trait StringToNumbersTrait {
    fn as_numbers(self) -> LinesAsNumbers;
}

impl StringToNumbersTrait for Lines<BufReader<File>> {
    fn as_numbers(self) -> LinesAsNumbers {
        LinesAsNumbers { iter: self }
    }
}
