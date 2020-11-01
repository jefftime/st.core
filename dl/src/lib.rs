#![no_std]

use c::types::*;
use core::{
    mem::MaybeUninit,
    ops::Drop,
    ptr::null_mut
};

pub const RTLD_NOW: c_int = 2;

#[link(name = "dl")]
extern {
    fn dlopen(file: *const c_char, mode: c_int) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

pub struct SharedLibrary {
    handle: *mut c_void
}

impl SharedLibrary {
    pub fn open(filename: *const c_char) -> Option<SharedLibrary> {
        let handle = unsafe {
            dlopen(filename, RTLD_NOW)
        };
        match handle {
            h if h == null_mut() => None,
            h => Some(SharedLibrary { handle: h })
        }
    }

    pub fn symbol(&self, symbol: *const c_char) -> Option<*mut c_void> {
        let symbol = unsafe {
            dlsym(self.handle, symbol)
        };
        match symbol {
            h if h == null_mut() => None,
            h => Some(symbol)
        }
    }
}

impl Drop for SharedLibrary {
    fn drop(&mut self) {
        unsafe {
            dlclose(self.handle);
        }
    }
}
