use core::{
    fmt::{Debug, Display},
    panic::Location,
};

pub trait HasErrorKind
where
    Self::ErrorKind: Copy + Clone + Debug + Display,
{
    type ErrorKind;

    fn kind(&self) -> Self::ErrorKind;
}

pub trait HasErrorLocation {
    fn location(&self) -> &'static Location<'static>;
}
