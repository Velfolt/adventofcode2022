use std::io::Error;

pub struct LinesAsNumbers<I> {
    iter: I,
}

impl<I> Iterator for LinesAsNumbers<I>
where
    I: Iterator<Item = Result<String, Error>>,
{
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

pub trait StringToNumbersTrait<I> {
    fn as_numbers(self) -> LinesAsNumbers<I>;
}

impl<I> StringToNumbersTrait<I> for I
where
    I: Iterator,
{
    fn as_numbers(self) -> LinesAsNumbers<I> {
        LinesAsNumbers { iter: self }
    }
}
