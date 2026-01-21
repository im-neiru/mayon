use core::{
    fmt::{self, Debug, Formatter},
    num::NonZeroUsize,
};

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Instance(NonZeroUsize);

impl Debug for Instance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Instance({:#x})", self.0.get())
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Surface(NonZeroUsize);

impl Debug for Surface {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Surface({:#x})", self.0.get())
    }
}
