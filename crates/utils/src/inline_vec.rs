use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

pub struct InlineVec<T, const CAPACITY: usize> {
    array: [MaybeUninit<T>; CAPACITY],
    length: usize,
}

impl<T, const CAPACITY: usize> InlineVec<T, CAPACITY> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            array: [const { MaybeUninit::uninit() }; CAPACITY],
            length: 0,
        }
    }

    #[inline]
    pub const fn push(&mut self, value: T) {
        self.array[self.length] = MaybeUninit::new(value);

        self.length += 1;
    }

    #[inline]
    pub const fn length(&self) -> usize {
        self.length
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
