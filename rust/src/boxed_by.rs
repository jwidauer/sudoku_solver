// This iterator can be used to iterate over a slice, by creating "boxes" of data.
// Assume an array like this:
//
// let arr: [u8; 16] = [ 1,  2,  3,  4,
//                       5,  6,  7,  8,
//                       9, 10, 11, 12,
//                      13, 14, 15, 16];
//
// let iter = arr.boxed_by(2);
// println!("{:?}", iter.next);  //  [1,  2,  5,  6]
// println!("{:?}", iter.next);  //  [3,  4,  7,  8]
// println!("{:?}", iter.next);  //  [9, 10, 13, 14]
// println!("{:?}", iter.next);  // [11, 12, 15, 16]
// println!("{:?}", iter.next);  // None
//
pub struct BoxedBy<'a, T> {
    buffer: &'a [T],
    box_sz: usize,
    step_sz: usize,
    n_cols: usize,
    counter: usize,
}

impl<'a, T> BoxedBy<'a, T> {
    fn new(buffer: &'a [T], dim: [usize; 2], box_size: usize) -> Self {
        assert_eq!(buffer.len(), dim.iter().product());
        BoxedBy {
            buffer,
            box_sz: box_size,
            step_sz: dim[0] / box_size,
            n_cols: dim[0],
            counter: 0,
        }
    }
}

impl<'a, T> Iterator for BoxedBy<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.step_sz {
            let start = self.counter * self.box_sz + self.counter / self.step_sz * self.n_cols;
            let res = self.buffer[start..]
                .chunks(self.box_sz)
                .step_by(self.step_sz)
                .take(self.box_sz)
                .flatten()
                .collect();

            self.counter += 1;
            Some(res)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxed_by_with_dim_8_2_with_box_size_2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let iter = BoxedBy::new(&v, [8, 2], 2);
        let res: Vec<_> = iter.collect();
        assert_eq!(
            res,
            vec![
                vec![&1, &2, &9, &10],
                vec![&3, &4, &11, &12],
                vec![&5, &6, &13, &14],
                vec![&7, &8, &15, &16]
            ]
        )
    }

    #[test]
    fn test_boxed_by_with_dim_8_2_with_box_size_4() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let iter = BoxedBy::new(&v, [8, 2], 4);
        let res: Vec<_> = iter.collect();
        assert_eq!(
            res,
            vec![
                vec![&1, &2, &3, &4, &9, &10, &11, &12],
                vec![&5, &6, &7, &8, &13, &14, &15, &16]
            ]
        )
    }

    #[test]
    fn test_boxed_by_with_dim_8_2_with_box_size_8() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let iter = BoxedBy::new(&v, [8, 2], 8);
        let res: Vec<_> = iter.collect();
        assert_eq!(
            res,
            vec![vec![
                &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12, &13, &14, &15, &16
            ]]
        )
    }

    #[test]
    fn test_boxed_by_with_dim_8_2_with_box_size_16() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let iter = BoxedBy::new(&v, [8, 2], 16);
        let res: Vec<_> = iter.collect();
        assert_eq!(
            res,
            vec![vec![
                &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12, &13, &14, &15, &16
            ]]
        )
    }
}
