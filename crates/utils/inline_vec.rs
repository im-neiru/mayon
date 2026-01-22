use core::{
    fmt,
    mem::MaybeUninit,
    ops::{Index, IndexMut},
    slice,
};

use crate::BufferOverflowError;

pub struct InlineVec<T, const CAPACITY: usize> {
    array: [MaybeUninit<T>; CAPACITY],
    length: usize,
}

impl<T, const CAPACITY: usize> InlineVec<T, CAPACITY> {
    /// Creates an empty InlineVec where no elements are initialized.
    ///
    /// # Examples
    ///
    /// ```
    /// # use utils::InlineVec;
    ///
    /// let v: InlineVec<i32, 4> = InlineVec::new();
    /// assert_eq!(v.length(), 0);
    /// assert_eq!(v.as_slice(), &[]);
    /// ```
    #[inline]
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            array: [const { MaybeUninit::uninit() }; CAPACITY],
            length: 0,
        }
    }

    /// Creates an InlineVec containing the elements of `value` in order.
    ///
    /// The returned container has `length == SIZE`. The remaining capacity (if any)
    /// is left uninitialized.
    ///
    /// # Examples
    ///
    /// ```
    /// # use utils::InlineVec;
    ///
    /// let v = InlineVec::<i32, 4>::from_array([10, 20]).unwrap();
    /// assert_eq!(v.length(), 2);
    /// assert_eq!(v.as_slice(), &[10, 20]);
    /// ```
    #[inline]
    pub fn from_array<const SIZE: usize>(value: [T; SIZE]) -> Result<Self, BufferOverflowError> {
        if SIZE > CAPACITY {
            return Err(BufferOverflowError);
        }

        let mut array = [const { MaybeUninit::uninit() }; CAPACITY];

        for (index, element) in value.into_iter().enumerate() {
            let dest = unsafe { array.get_unchecked_mut(index) };
            *dest = MaybeUninit::new(element);
        }

        Ok(Self {
            array,
            length: SIZE,
        })
    }

    /// Appends `value` to the vector if there is remaining capacity.
    ///
    /// If the inline buffer is full, `value` is dropped and a `crate::BufferOverflowError` is returned.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, `Err(crate::BufferOverflowError)` if the vector is at full capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use utils::InlineVec;
    ///
    /// let mut v = InlineVec::<i32, 2>::new();
    /// v.push(10).unwrap();
    /// v.push(20).unwrap();
    /// assert_eq!(v.length(), 2);
    /// assert!(v.push(30).is_err()); // value is dropped and an error is returned
    /// ```
    #[inline]
    pub fn push(&mut self, value: T) -> Result<(), crate::BufferOverflowError> {
        if self.length == CAPACITY {
            return Err(crate::BufferOverflowError);
        }

        self.array[self.length] = MaybeUninit::new(value);

        self.length += 1;

        Ok(())
    }

    /// Number of initialized elements stored in the `InlineVec`.
    ///
    /// # Returns
    ///
    /// The current number of initialized elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use utils::InlineVec;
    ///
    /// let mut v: InlineVec<i32, 4> = InlineVec::new();
    /// assert_eq!(v.length(), 0);
    /// v.push(10).unwrap();
    /// assert_eq!(v.length(), 1);
    /// ```
    #[inline]
    pub const fn length(&self) -> usize {
        self.length
    }

    /// Returns `true` if the `InlineVec` contains no initialized elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use utils::InlineVec;
    ///
    /// let v: InlineVec<i32, 4> = InlineVec::new();
    /// assert!(v.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Provides a slice of the initialized elements in this `InlineVec`.
    ///
    /// The returned slice borrows the elements at indices `0..self.length`, reflecting only the initialized portion of the internal storage.
    ///
    /// # Examples
    ///
    /// ```
    /// # use utils::InlineVec;
    ///
    /// let v = InlineVec::<u32, 3>::from_array([1, 2, 3]).unwrap();
    /// assert_eq!(v.as_slice(), &[1, 2, 3]);
    /// ```
    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.array.as_ptr().cast(), self.length) }
    }

    /// Forcefully sets the length of the vector to `length`.
    ///
    /// If `length` exceeds the `CAPACITY`, the length is capped at `CAPACITY` and the amount that
    /// exceeded the capacity is returned as "excess". If `length <= CAPACITY`, the excess is 0.
    ///
    /// This is particularly useful for FFI patterns (like Vulkan) where a function reports the
    /// total number of available elements, allowing the caller to know if their buffer was too small.
    ///
    /// # Returns
    ///
    /// The number of elements that could not fit (zero if all fit).
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - All elements in the range `0..min(length, CAPACITY)` are properly initialized.
    /// - If the new length is smaller than the previous length, the caller is responsible for
    ///   dropping the "removed" elements if they are not trivially droppable.
    ///
    /// # Examples
    ///
    /// ```
    /// # use utils::InlineVec;
    ///
    /// let mut v: InlineVec<u32, 2> = InlineVec::new();
    /// // Logic: we want to set length to 5, but capacity is 2.
    /// let excess = unsafe { v.set_length(5) };
    /// assert_eq!(v.length(), 2);
    /// assert_eq!(excess, 3); // 5 - 2 = 3
    /// ```
    #[inline]
    pub unsafe fn set_length(&mut self, length: usize) -> usize {
        let excess = length.saturating_sub(CAPACITY);

        self.length = length.min(CAPACITY);

        excess
    }
}

impl<T, const CAPACITY: usize> Index<usize> for InlineVec<T, CAPACITY> {
    type Output = T;

    /// Accesses the element at the given index and returns a reference to it.
    ///
    /// Returns a reference to the initialized element at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index >= self.length`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use utils::InlineVec;
    ///
    /// let v = InlineVec::<i32, 3>::from_array([10, 20, 30]).unwrap();
    /// assert_eq!(v[1], 20);
    /// ```
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
    /// Accesses the element at the given index for mutation, panicking if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use utils::InlineVec;
    ///
    /// let mut v = InlineVec::<i32, 3>::from_array([1, 2, 3]).unwrap();
    /// v[1] += 10;
    /// assert_eq!(v.as_slice(), &[1, 12, 3]);
    /// ```
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
    /// Drops all initialized elements held by the `InlineVec`.
    ///
    /// This destructor only drops the elements that have been initialized (indices `0..length`).
    fn drop(&mut self) {
        for index in 0..self.length {
            unsafe {
                self.array.get_unchecked_mut(index).assume_init_drop();
            }
        }
    }
}

impl<T, const CAPACITY: usize> fmt::Debug for InlineVec<T, CAPACITY>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();

        for index in 0..self.length {
            list.entry(unsafe { self.array[index].assume_init_ref() });
        }

        list.finish()
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
    fn as_slice_test() {
        let mut vec: InlineVec<u32, 3> = InlineVec::new();
        vec.push(5).unwrap();
        vec.push(10).unwrap();

        let slice = vec.as_slice();

        assert_eq!(slice, &[5, 10]);
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
            /// Increments the global drop counter when a Tracker value is dropped.
            ///
            /// # Examples
            ///
            /// ```
            /// use std::sync::atomic::{AtomicUsize, Ordering};
            ///
            /// static COUNTER: AtomicUsize = AtomicUsize::new(0);
            ///
            /// struct Tracker;
            ///
            /// impl Drop for Tracker {
            ///     fn drop(&mut self) {
            ///         COUNTER.fetch_add(1, Ordering::SeqCst);
            ///     }
            /// }
            ///
            /// {
            ///     let _t = Tracker;
            /// } // `_t` is dropped here
            ///
            /// assert_eq!(COUNTER.load(Ordering::SeqCst), 1);
            /// ```
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
