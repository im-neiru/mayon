use core::{alloc::Layout, mem::MaybeUninit, ptr::NonNull};

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Allocation Error")]
pub struct AllocError;

pub(crate) type AllocResult = Result<NonNull<[u8]>, AllocError>;

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    unsafe fn allocate(&self, layout: Layout) -> AllocResult;

    unsafe fn deallocate(&self, ptr: NonNull<u8>);

    unsafe fn reallocate(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult;

    #[inline]
    unsafe fn grow(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        unsafe { self.reallocate(ptr, new_layout) }
    }

    #[inline]
    unsafe fn shrink(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        unsafe { self.reallocate(ptr, new_layout) }
    }

    #[inline]
    unsafe fn allocate_init<T>(&self, value: T) -> Result<NonNull<T>, AllocError> {
        let layout = Layout::new::<T>();

        let ptr = if layout.size() == 0 {
            NonNull::dangling()
        } else {
            let ptr: NonNull<T> = unsafe { self.allocate(layout)?.cast() };

            debug_assert!(
                ptr.addr().get().is_multiple_of(layout.align()),
                "allocator returned misaligned pointer"
            );

            ptr
        };

        unsafe { ptr.write(value) };

        Ok(ptr)
    }

    #[inline]
    unsafe fn allocate_uninit<T>(&self) -> Result<NonNull<MaybeUninit<T>>, AllocError> {
        let layout = Layout::new::<T>();

        if layout.size() == 0 {
            return Ok(NonNull::dangling());
        }

        debug_assert_eq!(layout, Layout::new::<MaybeUninit<T>>());

        let ptr = unsafe { self.allocate(layout) }?;

        debug_assert!(
            ptr.addr().get().is_multiple_of(layout.align()),
            "allocator returned misaligned pointer"
        );

        Ok(ptr.cast())
    }

    #[inline]
    unsafe fn deallocate_init<T>(&self, ptr: NonNull<T>) {
        unsafe {
            if Layout::new::<T>().size() != 0 {
                ptr.drop_in_place();
                self.deallocate(ptr.cast())
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::Allocator;
    use core::{alloc::Layout, ptr::NonNull};

    #[test]
    fn test_basic_allocation() {
        unsafe {
            let system = crate::System;

            let layout = Layout::from_size_align(64, 8).unwrap();

            let res = system.allocate(layout).expect("Allocation failed");

            let ptr: NonNull<u8> = NonNull::new(res.as_ptr() as *mut u8).expect("Returned null");

            ptr.as_ptr().write_volatile(42);
            assert_eq!(ptr.as_ptr().read_volatile(), 42);

            system.deallocate(ptr);
        }
    }

    #[test]
    fn test_grow_and_shrink() {
        unsafe {
            let system = crate::System;
            let old_layout = Layout::from_size_align(32, 8).unwrap();
            let new_layout = Layout::from_size_align(128, 8).unwrap();

            let res = system.allocate(old_layout).expect("Initial alloc failed");
            let ptr = NonNull::new(res.as_ptr() as *mut u8).unwrap();

            ptr.as_ptr().write_bytes(0xAA, 32);

            let res_grown = system.grow(ptr, new_layout).expect("Grow failed");
            let ptr_grown = NonNull::new(res_grown.as_ptr() as *mut u8).unwrap();

            assert_eq!(*ptr_grown.as_ptr(), 0xAA);
            assert_eq!(res_grown.len(), 128);

            let smaller_layout = Layout::from_size_align(16, 8).unwrap();
            let res_shrunk = system
                .shrink(ptr_grown, smaller_layout)
                .expect("Shrink failed");
            let ptr_shrunk = NonNull::new(res_shrunk.as_ptr() as *mut u8).unwrap();

            assert_eq!(*ptr_shrunk.as_ptr(), 0xAA);
            assert_eq!(res_shrunk.len(), 16);

            system.deallocate(ptr_shrunk);
        }
    }

    #[test]
    fn test_alignment_comprehensive() {
        unsafe {
            let system = crate::System;

            let alignments = [1usize, 2, 4, 8, 16, 32, 64, 128];

            for &alignment in &alignments {
                let size = 3 * alignment;

                let layout = Layout::from_size_align(size, alignment).unwrap();
                let res = system
                    .allocate(layout)
                    .unwrap_or_else(|_| panic!("alloc failed (align = {})", alignment));

                let ptr = NonNull::new(res.as_ptr() as *mut u8).unwrap();
                let addr = ptr.as_ptr() as usize;

                assert_eq!(
                    addr % alignment,
                    0,
                    "base alloc: {:p} not {}-byte aligned",
                    ptr.as_ptr(),
                    alignment
                );

                ptr.as_ptr().write_bytes(0xAB, size);
                assert_eq!(*ptr.as_ptr(), 0xAB);

                let grown_layout = Layout::from_size_align(size * 2, alignment).unwrap();
                let grown = system.grow(ptr, grown_layout).expect("grow failed");

                let grown_ptr = NonNull::new(grown.as_ptr() as *mut u8).unwrap();
                let grown_addr = grown_ptr.as_ptr() as usize;

                assert_eq!(
                    grown_addr % alignment,
                    0,
                    "grow: {:p} not {}-byte aligned",
                    grown_ptr.as_ptr(),
                    alignment
                );

                assert_eq!(*grown_ptr.as_ptr(), 0xAB);

                let shrink_layout = Layout::from_size_align(alignment, alignment).unwrap();
                let shrunk = system
                    .shrink(grown_ptr, shrink_layout)
                    .expect("shrink failed");

                let shrunk_ptr = NonNull::new(shrunk.as_ptr() as *mut u8).unwrap();
                let shrunk_addr = shrunk_ptr.as_ptr() as usize;

                assert_eq!(
                    shrunk_addr % alignment,
                    0,
                    "shrink: {:p} not {}-byte aligned",
                    shrunk_ptr.as_ptr(),
                    alignment
                );

                assert_eq!(*shrunk_ptr.as_ptr(), 0xAB);

                system.deallocate(shrunk_ptr);
            }
        }
    }
}
