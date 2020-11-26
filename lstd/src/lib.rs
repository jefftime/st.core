#![no_std]

use c::stdlib;

pub mod io;
pub mod alloc;
pub mod container;
pub mod string;

pub fn abort() -> ! {
    unsafe { stdlib::abort() }
}

pub mod prelude {
    pub use crate::{print, println};
    pub use crate::alloc::Box;
    pub use crate::container::{Array, Rc};
    pub use crate::cstr;
}

#[macro_export]
macro_rules! cstr {
    ($string:literal) => {{
        concat!($string, "\0").as_ptr() as *const c_char
    }}
}
