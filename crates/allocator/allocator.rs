use core::{alloc::Layout, ptr::NonNull};

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
}
