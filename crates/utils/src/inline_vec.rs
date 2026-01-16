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

impl<T, const CAPACITY: usize> Drop for InlineVec<T, CAPACITY> {
    fn drop(&mut self) {
        for index in 0..self.length {
            unsafe {
                self.array.get_unchecked_mut(index).assume_init_drop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn push_pop_numbers() {
        let mut vec: InlineVec<u32, 5> = InlineVec::new();

        assert!(vec.push(10).is_ok());
        assert!(vec.push(20).is_ok());
        assert_eq!(vec.length(), 2);

        assert_eq!(vec[0], 10);
        assert_eq!(vec[1], 20);

        let result = std::panic::catch_unwind(|| vec[2]);
        assert!(result.is_err());
    }

    #[test]
    fn push_pop_strings() {
        let mut vec: InlineVec<String, 3> = InlineVec::new();

        assert!(vec.push("hello".to_string()).is_ok());
        assert!(vec.push("world".to_string()).is_ok());

        assert_eq!(vec.length(), 2);
        assert_eq!(vec[0], "hello");
        assert_eq!(vec[1], "world");

        vec[1].push('!');
        assert_eq!(vec[1], "world!");
    }

    #[test]
    fn overflow_test() {
        let mut vec: InlineVec<u32, 2> = InlineVec::new();
        assert!(vec.push(1).is_ok());
        assert!(vec.push(2).is_ok());

        assert!(vec.push(3).is_err());
        assert_eq!(vec.length(), 2);
    }

    #[test]
    fn as_slice_ptr_test() {
        let mut vec: InlineVec<u32, 3> = InlineVec::new();
        vec.push(5).unwrap();
        vec.push(10).unwrap();

        let slice_ptr = vec.as_slice_ptr();
        unsafe {
            let slice: &[u32] = &*slice_ptr;
            assert_eq!(slice, &[5, 10]);
        }
    }

    #[test]

    fn index_mut_test() {
        let mut vec: InlineVec<String, 2> = InlineVec::new();
        vec.push("a".to_string()).unwrap();
        vec.push("b".to_string()).unwrap();

        vec[0] = "x".to_string();
        vec[1].push('y');

        assert_eq!(vec[0], "x");
        assert_eq!(vec[1], "by");
    }

    static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn drop_non_copy() {
        struct Tracker;

        impl Drop for Tracker {
            fn drop(&mut self) {
                DROP_COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        }

        DROP_COUNTER.store(0, Ordering::SeqCst);

        let mut vec: InlineVec<Tracker, 3> = InlineVec::new();
        vec.push(Tracker).unwrap();
        vec.push(Tracker).unwrap();

        assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), 0);

        drop(vec);

        assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), 2);
    }
}
