use core::mem::MaybeUninit;

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
}
