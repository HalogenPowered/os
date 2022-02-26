use core::cell::UnsafeCell;
use core::fmt::{Debug, Display, Formatter, Error as FormatError};
use core::ops::Deref;

pub struct LateInit<T>(UnsafeCell<Option<T>>);

impl<T> LateInit<T> {
    #[inline]
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    pub fn init(&self, value: T) {
        assert!(self.option().is_none(), "LateInit.init called more than once!");
        unsafe { *self.0.get() = Some(value) }
    }

    #[inline]
    pub fn get(&self) -> &T {
        self.option().as_ref().expect("LateInit.get called before initialization!")
    }

    #[inline]
    pub fn get_mut(&self) -> &mut T {
        self.option().as_mut().expect("LateInit.get_mut called before initialization!")
    }

    #[inline]
    fn option(&self) -> &mut Option<T> {
        unsafe { &mut *self.0.get() }
    }
}

unsafe impl<T> Sync for LateInit<T> {}

impl<T> Deref for LateInit<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> AsRef<T> for LateInit<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

macro_rules! impl_debug_display {
    ($T:ident) => {
        impl<T: $T> $T for LateInit<T> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
                match self.option() {
                    Some(ref x) => { x.fmt(f) },
                    None => { write!(f, "<UNINITIALIZED>") },
                }
            }
        }
    }
}

impl_debug_display!(Debug);
impl_debug_display!(Display);
