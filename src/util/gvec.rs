#![allow(non_camel_case_types)] // for ArrayLength types

use core::{fmt, mem::MaybeUninit, ops::Deref, slice};

use generic_array::{ArrayLength, GenericArray};

pub(crate) struct GVec<T, CAP: ArrayLength> {
    data: GenericArray<MaybeUninit<T>, CAP>,
    len: usize,
}

impl<T, CAP: ArrayLength> Default for GVec<T, CAP> {
    fn default() -> Self {
        Self {
            data: GenericArray::uninit(),
            len: 0,
        }
    }
}

impl<T, CAP: ArrayLength> GVec<T, CAP> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_extend(&mut self, arr: impl AsRef<[T]>) -> Result<(), CapacityError>
    where
        T: Clone,
    {
        let arr = arr.as_ref();

        if self.len + arr.len() >= CAP::USIZE {
            return Err(CapacityError);
        }

        for it in arr {
            self.data[self.len].write(it.clone());
            self.len += 1;
        }

        Ok(())
    }

    pub fn push(&mut self, item: T) {
        assert!(!self.is_full(), "GVec is full");

        self.data[self.len].write(item);
        self.len += 1;
    }

    pub fn pop(&mut self) -> T {
        assert!(!self.is_full(), "GVec is empty");

        self.len -= 1;
        unsafe { self.data[self.len].assume_init_read() }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == CAP::USIZE
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<CAP: ArrayLength> minicbor::encode::Write for GVec<u8, CAP> {
    type Error = CapacityError;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.try_extend(buf)
    }
}

impl<T, CAP: ArrayLength> Deref for GVec<T, CAP> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        let ptr = self.data[..self.len()].as_ptr() as *const T;
        unsafe { slice::from_raw_parts(ptr, self.len()) }
    }
}

// -------------------- Capacity Error --------------------

#[derive(Debug)]
pub struct CapacityError;

impl fmt::Display for CapacityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl core::error::Error for CapacityError {}
