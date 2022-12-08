pub struct GroupBy<I> {
    n: usize,
    iter: I,
}

impl<I> Iterator for GroupBy<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let vec: Self::Item = self.iter.by_ref().take(self.n).collect();
        if vec.len() > 0 {
            Some(vec)
        } else {
            None
        }
    }
}

pub trait GroupByTrait<I> {
    fn group_by(self, n: usize) -> GroupBy<I>;
}

impl<I> GroupByTrait<I> for I
where
    I: Iterator,
{
    fn group_by(self, n: usize) -> GroupBy<I> {
        GroupBy { iter: self, n }
    }
}

#[test]
fn test_group_by_trait() {
    let calories = vec![100, 200, 0, 600];
    let sums = calories.into_iter().group_by(2).collect::<Vec<Vec<i32>>>();

    assert_eq!(sums[0], vec!(100, 200));
    assert_eq!(sums[1], vec!(0, 600));
}
