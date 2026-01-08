use std::{alloc::Allocator, alloc::Global, sync::Arc};

use crate::backend::{Backend, CreateBackend};

pub struct Instance<A: Allocator = Global>
where
    A: Allocator,
{
    backend: Arc<dyn Backend + 'static, A>,
}

pub(crate) struct Inner {
    backend: dyn Backend + Send + Sync,
}

impl<A: Allocator> Instance<A> {
    pub fn new_in<'s, B>(params: B::Params, allocator: A) -> Result<Self, B::Error>
    where
        B: Backend + CreateBackend<'s> + 'static,
    {
        let backend = B::create(params)?;

        Ok(Instance {
            backend: Arc::new_in(backend, allocator),
        })
    }
}

impl Instance<Global> {
    pub fn new<'s, B>(params: B::Params) -> Result<Self, B::Error>
    where
        B: Backend + CreateBackend<'s> + 'static,
    {
        let backend = B::create(params)?;

        Ok(Instance {
            backend: Arc::new(backend),
        })
    }
}
