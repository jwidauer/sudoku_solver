pub struct SpacedBy<'a, T> {
    v: &'a [T],
    n: usize,
    counter: usize,
}

impl<'a, T> SpacedBy<'a, T> {
    fn new(slice: &'a [T], offset: usize) -> Self {
        SpacedBy {
            v: slice,
            n: offset,
            counter: 0,
        }
    }
}

impl<'a, T> Iterator for SpacedBy<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.n {
            let res = self.v
                                .iter()
                                .skip(self.counter)
                                .step_by(self.n)
                                .collect();
            self.counter += 1;
            Some(res)
        } else {
            None
        }
    }
}

pub trait SliceExt<'a, T> {
    fn spaced_by(&'a self, offset: usize) -> SpacedBy<'a, T>;
}

impl<'a, T> SliceExt<'a, T> for [T] {
    fn spaced_by(&'a self, offset: usize) -> SpacedBy<'a, T> {
        SpacedBy::new(&self, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spaced_by_2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let res: Vec<_> = v.spaced_by(2).collect();
        assert_eq!(res, vec![vec![&1, &3, &5, &7, &9], vec![&2, &4, &6, &8]]);
    }

    #[test]
    fn test_spaced_by_3() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let res: Vec<_> = v.spaced_by(3).collect();
        assert_eq!(
            res,
            vec![vec![&1, &4, &7], vec![&2, &5, &8], vec![&3, &6, &9]]
        );
    }
}
