pub trait Backend {}

pub trait CreateBackend<'s>
where
    Self: Backend,
{
    type Error;
    type Params;

    fn create(params: Self::Params) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
