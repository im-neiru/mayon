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
    pub fn_reallocation: FnReallocationFunction,
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
            fn_reallocation: None,
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
    ) -> *mut c_void {
        if let Ok(ptr) = allocator
            .as_ref()
            .allocate(Layout::from_size_align_unchecked(size, alignment))
        {
            let offset = Self::write_layout(ptr, size, alignment);

            ptr.as_ptr().cast::<c_void>().byte_add(offset)
        } else {
            null_mut()
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn write_layout(ptr: NonNull<[u8]>, size: usize, alignment: usize) -> usize {
        type LayoutType = [usize; 2];

        let layout: LayoutType = [size, alignment];
        const BYTE_SIZE: usize = size_of::<LayoutType>();

        let src = layout.as_ptr().cast_mut().cast::<u8>();
        let dest: *mut u8 = ptr.as_ptr().cast();

        dest.copy_from_nonoverlapping(src, BYTE_SIZE);

        BYTE_SIZE
    }
}

pub(in crate::backends::vulkan) type FnAllocationFunction<A> =
    unsafe extern "system" fn(
        allocator: NonNull<A>,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> *mut c_void;

pub(in crate::backends::vulkan) type FnReallocationFunction = Option<
    unsafe extern "system" fn(
        user_data: *mut c_void,
        original: *mut c_void,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> *mut c_void,
>;

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
