use heapless::Vec;

pub(crate) trait VecExt<T, const CAP: usize> {
    fn pop_slice<const N: usize>(&mut self) -> [T; N]
    where
        T: Clone;

    fn assert_empty(&self);
}

impl<T, const CAP: usize> VecExt<T, CAP> for Vec<T, CAP> {
    fn pop_slice<const N: usize>(&mut self) -> [T; N]
    where
        T: Clone,
    {
        if self.len() < N {
            panic!("expected >= {N} items, got {} only", self.len())
        }

        let ans = <&[T; N]>::try_from(&self[self.len() - N..])
            .expect("correct size")
            .clone();

        self.truncate(self.len() - N);

        ans.clone()
    }

    fn assert_empty(&self) {
        assert!(self.is_empty());
    }
}
