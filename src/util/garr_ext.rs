use core::{
    mem::MaybeUninit,
    ops::{Add, Sub},
    ptr,
};

use generic_array::{ArrayLength, GenericArray};
use typenum::{Const, Diff, Same, Sum, ToUInt, U};

pub(crate) trait GArrExt<T, N: ArrayLength> {
    #[must_use]
    fn join_arr<const M: usize>(&self, arr: &[T; M]) -> GenericArray<T, Sum<N, U<M>>>
    where
        T: Clone,
        Const<M>: ToUInt,
        N: Add<U<M>>,
        Sum<N, U<M>>: ArrayLength;

    #[must_use]
    fn split_arr<const M: usize>(&self) -> (GenericArray<T, Diff<N, U<M>>>, [T; M])
    where
        T: Clone,
        Const<M>: ToUInt,
        N: Sub<U<M>>,
        Diff<N, U<M>>: ArrayLength;

    fn assert_empty(&self)
    where
        N: Same<U<0>>,
    {
    }
}

impl<T, N: ArrayLength> GArrExt<T, N> for GenericArray<T, N> {
    fn join_arr<const M: usize>(&self, arr: &[T; M]) -> GenericArray<T, Sum<N, U<M>>>
    where
        T: Clone,
        Sum<N, U<M>>: ArrayLength,
        N: Add<U<M>>,
        Const<M>: ToUInt,
    {
        let mut ans = GenericArray::uninit();

        for i in 0..N::USIZE {
            ans[i].write(self[i].clone());
        }

        for i in 0..M {
            ans[N::USIZE + i].write(arr[i].clone());
        }

        unsafe { GenericArray::assume_init(ans) }
    }

    fn split_arr<const M: usize>(&self) -> (GenericArray<T, Diff<N, U<M>>>, [T; M])
    where
        T: Clone,
        Const<M>: ToUInt,
        N: Sub<U<M>>,
        Diff<N, U<M>>: ArrayLength,
    {
        let mut rest = GenericArray::uninit();
        let mut arr = [const { MaybeUninit::<T>::uninit() }; M];

        for i in 0..(N::USIZE - M) {
            rest[i].write(self[i].clone());
        }

        for i in 0..M {
            arr[i].write(self[i + N::USIZE - M].clone());
        }

        unsafe {
            (
                GenericArray::assume_init(rest),
                ptr::read(arr.as_ptr() as *const [T; M]),
            )
        }
    }
}
