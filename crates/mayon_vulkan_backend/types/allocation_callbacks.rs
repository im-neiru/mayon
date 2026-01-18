use core::{alloc::Layout, ffi::c_void, marker::PhantomData, mem::transmute, ptr::NonNull};

use allocator::Allocator;

#[repr(C)]
pub(crate) struct AllocationCallbacks<'a, A> {
    pub allocator: NonNull<A>,
    pub fn_allocation: FnAllocationFunction<A>,
    pub fn_reallocation: FnReallocationFunction<A>,
    pub fn_free: FnFreeFunction<A>,
    pub fn_internal_allocation: FnInternalAllocationNotification,
    pub fn_internal_free: FnInternalFreeNotification,
    pub _marker: PhantomData<&'a A>,
}

pub(crate) type AllocationCallbacksRef<'a> = NonNull<AllocationCallbacks<'a, ()>>;

impl<'a, A> AllocationCallbacks<'a, A>
where
    A: Allocator + 'static,
{
    pub(crate) fn new(allocator: NonNull<A>) -> Self {
        Self {
            allocator,
            fn_allocation: Self::handle_allocation,
            fn_reallocation: Self::handle_reallocation,
            fn_free: Self::handle_free,
            fn_internal_allocation: None,
            fn_internal_free: None,
            _marker: Default::default(),
        }
    }

    #[inline]
    #[allow(unsafe_op_in_unsafe_fn)]
    pub(crate) const unsafe fn alloc_ref(&self) -> AllocationCallbacksRef<'a> {
        let self_ptr = self as *const Self;

        transmute::<*const Self, AllocationCallbacksRef>(self_ptr)
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "system" fn handle_allocation(
        allocator: NonNull<A>,
        size: usize,
        alignment: usize,
        _: SystemAllocationScope,
    ) -> Option<NonNull<c_void>> {
        let allocator = allocator.as_ref();

        let Ok(ptr) = allocator.allocate(Layout::from_size_align_unchecked(size, alignment)) else {
            return None;
        };

        Some(ptr.cast())
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

        let Ok(layout) = Layout::from_size_align(size, alignment) else {
            return None;
        };

        let Ok(ptr) = allocator.reallocate(
            transmute::<Option<NonNull<c_void>>, NonNull<u8>>(original),
            layout,
        ) else {
            return None;
        };

        Some(ptr.cast())
    }

    /// Frees memory previously allocated by the given allocator using the allocator's stored layout.
    ///
    /// The caller must ensure `allocator` points to the allocator instance that originally allocated
    /// `memory`, and that `memory` was returned by that allocator; otherwise the behavior is undefined.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::ptr::NonNull;
    /// use std::ffi::c_void;
    ///
    /// // SAFETY: `allocator_ptr` and `memory_ptr` must be valid and come from the same allocator.
    /// unsafe {
    ///     handle_free(allocator_ptr, memory_ptr);
    /// }
    /// ```
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "system" fn handle_free(allocator: NonNull<A>, memory: NonNull<c_void>) {
        let allocator = allocator.as_ref();

        allocator.deallocate(memory.cast());
    }
}

pub(crate) type FnAllocationFunction<A> = unsafe extern "system" fn(
    allocator: NonNull<A>,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> Option<NonNull<c_void>>;

pub(crate) type FnReallocationFunction<A> = unsafe extern "system" fn(
    allocator: NonNull<A>,
    original: Option<NonNull<c_void>>,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> Option<NonNull<c_void>>;

pub type FnFreeFunction<A> =
    unsafe extern "system" fn(allocator: NonNull<A>, memory: NonNull<c_void>);

pub(crate) type FnInternalAllocationNotification = Option<
    unsafe extern "system" fn(
        user_data: *mut c_void,
        size: usize,
        allocation_type: InternalAllocationType,
        allocation_scope: SystemAllocationScope,
    ),
>;

pub(crate) type FnInternalFreeNotification = Option<
    unsafe extern "system" fn(
        user_data: *mut c_void,
        size: usize,
        allocation_type: InternalAllocationType,
        allocation_scope: SystemAllocationScope,
    ),
>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct SystemAllocationScope(pub(crate) i32);

#[allow(unused)]
impl SystemAllocationScope {
    pub(crate) const COMMAND: Self = Self(0);
    pub(crate) const OBJECT: Self = Self(1);
    pub(crate) const CACHE: Self = Self(2);
    pub(crate) const DEVICE: Self = Self(3);
    pub(crate) const INSTANCE: Self = Self(4);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct InternalAllocationType(pub(crate) i32);

impl InternalAllocationType {
    #[allow(unused)]
    pub(crate) const EXECUTABLE: Self = Self(0);
}
