#![no_std]

use c::{
    stdlib
};

pub mod io;
pub mod alloc;

pub fn abort() -> ! {
    unsafe { stdlib::abort() }
}

