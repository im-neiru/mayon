mod alloc;
mod inner;

use std::{alloc::Allocator, alloc::Global, sync::Arc};

use crate::backends::{Backend, CreateBackend};

use inner::Inner;

pub struct Instance<A: Allocator = Global>
where
    A: Allocator,
{
    backend: Arc<dyn Backend + 'static, A>,
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
