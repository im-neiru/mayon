use core::{alloc::Allocator, ffi::c_void, marker::PhantomData};

#[repr(C)]
pub(in crate::backends::vulkan) struct AllocationCallbacks<'a, A> {
    pub allocator: *const A,
    pub fn_allocation: FnAllocationFunction,
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
    pub(crate) fn new(allocator: *const A) -> Self {
        Self {
            // TODO: redirect allocations to allocator
            allocator,
            fn_allocation: None,
            fn_reallocation: None,
            fn_free: None,
            fn_internal_allocation: None,
            fn_internal_free: None,
            _marker: Default::default(),
        }
    }
}

pub(in crate::backends::vulkan) type FnAllocationFunction = Option<
    unsafe extern "system" fn(
        user_data: *mut c_void,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> *mut c_void,
>;

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
