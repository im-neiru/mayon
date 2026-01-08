use std::{alloc::Allocator, alloc::Global, sync::Arc};

use crate::backend::Backend;

pub struct Instance<A: Allocator = Global>
where
    A: Allocator,
{
    backend: Arc<dyn Backend, A>,
}
