use crate::backend::{
    vulkan::{backend::FnTable, Error, VulkanBackend},
    CreateBackend,
};

impl<'s> CreateBackend<'s> for VulkanBackend {
    type Error = Error;
    type Params = VulkanBackendParams<'s>;

    fn create(_: Self::Params) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let _fns = FnTable::global()?;

        Ok(Self {})
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct VulkanBackendParams<'s> {
    pub application_name: Option<&'s str>,
    pub engine_name: Option<&'s str>,
}

impl<'s> VulkanBackendParams<'s> {
    #[inline]
    pub fn with_application_name(mut self, application_name: impl Into<&'s str>) -> Self {
        self.application_name = Some(application_name.into());
        self
    }

    #[inline]
    pub fn with_engine_name(mut self, engine_name: impl Into<&'s str>) -> Self {
        self.engine_name = Some(engine_name.into());
        self
    }
}
