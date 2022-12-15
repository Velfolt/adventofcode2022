use std::{collections::HashMap, fmt::Display, ops::RangeInclusive};

pub struct Grid<T> {
    pub data: HashMap<(i64, i64), T>,
}

impl<T> Grid<T>
where
    T: PartialEq,
{
    pub fn min_y(&self) -> i64 {
        self.data.keys().map(|(_, y)| *y).min().unwrap()
    }

    pub fn max_y(&self) -> i64 {
        self.data.keys().map(|(_, y)| *y).max().unwrap()
    }

    pub fn x_range(&self) -> RangeInclusive<i64> {
        let keys = self.data.keys().map(|(x, _)| *x);

        keys.clone().min().unwrap()..=keys.clone().max().unwrap()
    }

    pub fn count(&self, t: T) -> usize {
        self.data.values().filter(|value| **value == t).count()
    }
}

impl<T> Display for Grid<T>
where
    T: Display + Default + PartialEq + Clone + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_range = self.x_range();
        let y_range = 0..=self.max_y();

        for y in y_range.clone() {
            for x in x_range.clone() {
                write!(f, "{}", self.data.get(&(x, y)).map_or(T::default(), |e| *e))?;
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}

pub fn range_inclusive(a: i64, b: i64) -> impl Iterator<Item = i64> {
    let x: Box<dyn Iterator<Item = i64>>;
    if b > a {
        x = Box::new(a..=b)
    } else {
        x = Box::new((b..=a).rev())
    }
    x
}
