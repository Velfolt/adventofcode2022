pub struct SummedGroups<I> {
    iter: I,
}

impl<I> Iterator for SummedGroups<I>
where
    I: Iterator<Item = i32>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let sum = self.iter.by_ref().take_while(|x| *x != 0).sum();
        if sum > 0 {
            Some(sum)
        } else {
            None
        }
    }
}

pub trait SummedGroupsTrait<I> {
    fn summed_groups(self) -> SummedGroups<I>;
}

impl<I> SummedGroupsTrait<I> for I
where
    I: Iterator<Item = i32>,
{
    fn summed_groups(self) -> SummedGroups<I> {
        SummedGroups { iter: self }
    }
}

#[test]
fn test_summed_groups_trait() {
    let calories = vec![100, 200, 0, 600];
    let sums = calories.into_iter().summed_groups().collect::<Vec<i32>>();

    assert_eq!(sums[0], 300);
    assert_eq!(sums[1], 600);
}
