use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
    ptr::slice_from_raw_parts,
};

pub struct InlineVec<T, const CAPACITY: usize> {
    array: [MaybeUninit<T>; CAPACITY],
    length: usize,
}

impl<T, const CAPACITY: usize> InlineVec<T, CAPACITY> {
    #[inline]
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            array: [const { MaybeUninit::uninit() }; CAPACITY],
            length: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) -> Result<(), crate::BufferOverflowError> {
        if self.length == CAPACITY {
            drop(value);

            return Err(crate::BufferOverflowError);
        }

        self.array[self.length] = MaybeUninit::new(value);

        self.length += 1;

        Ok(())
    }

    #[inline]
    pub const fn length(&self) -> usize {
        self.length
    }

    pub const fn as_slice_ptr(&self) -> *const [T] {
        slice_from_raw_parts(self.array.as_ptr().cast(), self.length)
    }
}

impl<T, const CAPACITY: usize> Index<usize> for InlineVec<T, CAPACITY> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.length {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.length, index
            );
        }

        unsafe {
            let element = self.array.get_unchecked(index);

            element.assume_init_ref()
        }
    }
}

impl<T, const CAPACITY: usize> IndexMut<usize> for InlineVec<T, CAPACITY> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.length {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.length, index
            );
        }

        unsafe {
            let element = self.array.get_unchecked_mut(index);

            element.assume_init_mut()
        }
    }
}
