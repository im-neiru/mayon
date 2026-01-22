#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bool32 {
    False = 0,
    True = 1,
}

impl From<bool> for Bool32 {
    #[inline]
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

impl From<Bool32> for bool {
    #[inline]
    fn from(value: Bool32) -> Self {
        value == Bool32::True
    }
}
