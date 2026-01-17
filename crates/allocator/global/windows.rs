use core::{alloc::Layout, ffi::c_void, ptr::NonNull};

use crate::{AllocError, allocator::AllocResult};

#[link(name = "kernel32")]
unsafe extern "system" {

    /// Retrieves a handle to the default heap of the calling process.
    /// This handle can then be used in subsequent calls to the heap functions.
    #[link_name = "GetProcessHeap"]
    unsafe fn get_process_heap() -> *mut c_void;

    /// Allocates a block of memory from a heap. The allocated memory is not movable.
    #[link_name = "HeapAlloc"]
    unsafe fn heap_alloc(heap: *mut c_void, flags: u32, size: usize) -> *mut c_void;

    /// Frees a memory block allocated from a heap by the HeapAlloc or HeapReAlloc function.
    #[link_name = "HeapFree"]
    unsafe fn heap_free(heap: *mut c_void, flags: u32, ptr: *mut c_void) -> i32;

    /// Reallocates a block of memory from a heap.
    /// This function enables you to resize a memory block and change other memory block properties.
    /// The allocated memory is not movable.
    #[link_name = "HeapReAlloc"]
    unsafe fn heap_realloc(
        heap: *mut c_void,
        flags: u32,
        ptr: *mut c_void,
        new_size: usize,
    ) -> *mut c_void;
}

const HEAP_ZERO_MEMORY: u32 = 0x00000008;

#[inline]
unsafe fn allocate(layout: Layout, flags: u32) -> AllocResult {
    let ptr = unsafe { heap_alloc(get_process_heap(), flags, layout.size()) as *mut u8 };

    let Some(ptr) = NonNull::new(ptr) else {
        return Err(AllocError);
    };

    Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
}

#[inline]
unsafe fn rellocate(ptr: NonNull<u8>, new_layout: Layout, flags: u32) -> AllocResult {
    let ptr = unsafe {
        heap_realloc(
            get_process_heap(),
            flags,
            ptr.as_ptr().cast(),
            new_layout.size(),
        ) as *mut u8
    };

    let Some(ptr) = NonNull::new(ptr) else {
        return Err(AllocError);
    };

    Ok(NonNull::slice_from_raw_parts(ptr, new_layout.size()))
}

unsafe impl crate::Allocator for super::System {
    #[inline]
    unsafe fn allocate(&self, layout: Layout) -> AllocResult {
        unsafe { allocate(layout, 0) }
    }

    #[inline]
    unsafe fn allocate_zeroed(&self, layout: Layout) -> AllocResult {
        unsafe { allocate(layout, HEAP_ZERO_MEMORY) }
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>) {
        unsafe { heap_free(get_process_heap(), 0, ptr.as_ptr() as *mut _) };
    }

    #[inline]
    unsafe fn grow(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        unsafe { rellocate(ptr, new_layout, 0) }
    }

    #[inline]
    unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        unsafe { rellocate(ptr, new_layout, HEAP_ZERO_MEMORY) }
    }

    #[inline]
    unsafe fn shrink(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        unsafe { rellocate(ptr, new_layout, 0) }
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
    fn test_allocate_zeroed() {
        unsafe {
            let system = crate::System;
            let size = 1024;
            let layout = Layout::from_size_align(size, 8).unwrap();

            let res = system
                .allocate_zeroed(layout)
                .expect("Zeroed allocation failed");

            let ptr = res.as_ptr() as *mut u8;

            for i in 0..size {
                assert_eq!(*ptr.add(i), 0, "Memory at offset {} was not zeroed", i);
            }

            system.deallocate(NonNull::new(ptr).unwrap());
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
