use std::slice::Iter;

pub struct SummedGroups<'a> {
    iter: Iter<'a, i32>,
}

impl<'a> Iterator for SummedGroups<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let sum = self.iter.by_ref().take_while(|x| **x != 0).sum();
        if sum > 0 {
            Some(sum)
        } else {
            None
        }
    }
}

pub fn summed_groups(numbers: &Vec<i32>) -> SummedGroups {
    SummedGroups {
        iter: numbers.iter(),
    }
}

#[test]
fn test_summed_groups() {
    let calories = vec![100, 200, 0, 600];
    let sums: Vec<i32> = summed_groups(&calories).collect();

    assert_eq!(sums[0], 300);
    assert_eq!(sums[1], 600);
}
