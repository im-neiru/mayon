use core::{
    alloc::{Allocator, Layout},
    ffi::c_void,
    marker::PhantomData,
    mem::size_of,
    ptr::{NonNull, null_mut},
};

#[repr(C)]
pub(in crate::backends::vulkan) struct AllocationCallbacks<'a, A> {
    pub allocator: NonNull<A>,
    pub fn_allocation: FnAllocationFunction<A>,
    pub fn_reallocation: FnReallocationFunction<A>,
    pub fn_free: FnFreeFunction,
    pub fn_internal_allocation: FnInternalAllocationNotification,
    pub fn_internal_free: FnInternalFreeNotification,
    pub _marker: PhantomData<&'a A>,
}

impl<'a, A> AllocationCallbacks<'a, A>
where
    A: Allocator + 'static,
{
    pub(crate) fn new(allocator: NonNull<A>) -> Self {
        Self {
            allocator,
            fn_allocation: Self::handle_allocation,
            fn_reallocation: Self::handle_reallocation,
            fn_free: None,
            fn_internal_allocation: None,
            fn_internal_free: None,
            _marker: Default::default(),
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "system" fn handle_allocation(
        allocator: NonNull<A>,
        size: usize,
        alignment: usize,
        _: SystemAllocationScope,
    ) -> Option<NonNull<c_void>> {
        if let Ok(ptr) = allocator
            .as_ref()
            .allocate(Layout::from_size_align_unchecked(size, alignment))
        {
            write_layout(ptr, size, alignment);

            Some(ptr.cast::<c_void>().byte_add(LAYOUT_SIZE))
        } else {
            None
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "system" fn handle_reallocation(
        allocator: NonNull<A>,
        original: Option<NonNull<c_void>>,
        size: usize,
        alignment: usize,
        _: SystemAllocationScope,
    ) -> Option<NonNull<c_void>> {
        let allocator = allocator.as_ref();

        let (old_layout, true_ptr) = read_layout(original?.cast());

        if old_layout.size() == size && old_layout.align() == alignment {
            return original;
        }

        let new_layout = Layout::from_size_align_unchecked(size, alignment);

        let result = if old_layout.size() > size {
            allocator.shrink(true_ptr, old_layout, new_layout)
        } else {
            allocator.grow(true_ptr, old_layout, new_layout)
        };

        if let Ok(ptr) = result {
            write_layout(ptr, size, alignment);

            Some(ptr.cast::<c_void>().byte_add(LAYOUT_SIZE))
        } else {
            None
        }
    }
}

pub(in crate::backends::vulkan) type FnAllocationFunction<A> =
    unsafe extern "system" fn(
        allocator: NonNull<A>,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> Option<NonNull<c_void>>;

pub(in crate::backends::vulkan) type FnReallocationFunction<A> =
    unsafe extern "system" fn(
        allocator: NonNull<A>,
        original: Option<NonNull<c_void>>,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> Option<NonNull<c_void>>;

pub type FnFreeFunction =
    Option<unsafe extern "system" fn(p_user_data: *mut c_void, p_memory: *mut c_void)>;

pub(in crate::backends::vulkan) type FnInternalAllocationNotification = Option<
    unsafe extern "system" fn(
        user_data: *mut c_void,
        size: usize,
        allocation_type: InternalAllocationType,
        allocation_scope: SystemAllocationScope,
    ),
>;

pub(in crate::backends::vulkan) type FnInternalFreeNotification = Option<
    unsafe extern "system" fn(
        user_data: *mut c_void,
        size: usize,
        allocation_type: InternalAllocationType,
        allocation_scope: SystemAllocationScope,
    ),
>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(in crate::backends::vulkan) struct SystemAllocationScope(pub(crate) i32);

impl SystemAllocationScope {
    pub(in crate::backends::vulkan) const COMMAND: Self = Self(0);
    pub(in crate::backends::vulkan) const OBJECT: Self = Self(1);
    pub(in crate::backends::vulkan) const CACHE: Self = Self(2);
    pub(in crate::backends::vulkan) const DEVICE: Self = Self(3);
    pub(in crate::backends::vulkan) const INSTANCE: Self = Self(4);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(in crate::backends::vulkan) struct InternalAllocationType(pub(crate) i32);

impl InternalAllocationType {
    pub(in crate::backends::vulkan) const EXECUTABLE: Self = Self(0);
}

const LAYOUT_LENGTH: usize = 2;
const LAYOUT_SIZE: usize = size_of::<LayoutType>();
type LayoutType = [usize; LAYOUT_LENGTH];

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn write_layout(ptr: NonNull<[u8]>, size: usize, alignment: usize) {
    let layout: LayoutType = [size, alignment];

    let src = layout.as_ptr().cast_mut().cast::<u8>();
    let dest: *mut u8 = ptr.as_ptr().cast();

    dest.copy_from_nonoverlapping(src, LAYOUT_SIZE);
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn read_layout(ptr: NonNull<u8>) -> (Layout, NonNull<u8>) {
    let true_ptr = ptr.byte_sub(LAYOUT_SIZE);
    let [size, alignment] =
        *NonNull::slice_from_raw_parts(ptr.cast::<usize>(), LAYOUT_LENGTH).as_ref()
    else {
        unreachable!("This slice have is [usize; 2]")
    };

    let layout = Layout::from_size_align_unchecked(size, alignment);

    (layout, true_ptr)
}
