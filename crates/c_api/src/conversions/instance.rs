use core::{
    mem::transmute,
    ops::{Deref, DerefMut},
};

use crate::rs;

impl From<rs::Instance> for crate::MynInstance {
    #[inline(always)]
    fn from(value: rs::Instance) -> Self {
        unsafe { transmute::<rs::Instance, Self>(value) }
    }
}

impl Deref for crate::MynInstance {
    type Target = rs::Instance;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        let ptr: *const Self::Target = (self as *const Self).cast();

        unsafe { ptr.as_ref().unwrap_unchecked() }
    }
}

impl DerefMut for crate::MynInstance {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr: *mut Self::Target = (self as *mut Self).cast();

        unsafe { ptr.as_mut().unwrap_unchecked() }
    }
}
