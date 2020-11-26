mod rc;

pub use rc::*;

use crate::alloc::{alloc, dealloc};
use core::{
    cell::Cell,
    mem::transmute_copy,
    ops::{Deref, DerefMut, Drop},
    ptr::drop_in_place,
    slice::{from_raw_parts, from_raw_parts_mut}
};

pub struct Array<T> {
    data: *mut T,
    len: Cell<isize>,
    cap: usize
}

impl<T> Array<T> {
    pub fn new(cap: usize) -> Array<T> {
        let data: *mut T = alloc(cap).unwrap();
        let len = Cell::new(0);
        Array {
            data: data,
            len: len,
            cap: cap
        }
    }

    /// This allows one to initialize an array from foreign code
    pub unsafe fn assume_init(&self, len: isize) {
        self.len.set(len);
    }

    pub fn push(&mut self, value: T) -> bool {
        if self.len.get() == self.cap as isize { return false; }

        unsafe { self.data.offset(self.len.get()).write(value); }
        self.len.set(self.len.get() + 1);
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len.get() > 0 {
            self.len.set(self.len.get() - 1);
            unsafe {
                // The transmute_copy() is used to get around the fact
                // that T is not a Copy type
                let result = &*self.data.offset(self.len.get());
                Some(transmute_copy(result))
            }
        } else {
            None
        }
    }

    pub fn from_slice(array: &[T]) -> Array<T> {
        let data: *mut T = alloc(array.len()).unwrap();
        for (i, x) in array.iter().enumerate() {
            unsafe {
                data
                    .offset(i as isize)
                    .write(transmute_copy(&*x));
            }
        }

        Array {
            data: data,
            len: Cell::new(array.len() as isize),
            cap: array.len()
        }
    }

    pub fn len(&self) -> isize {
        self.len.get()
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        for x in 0..self.len.get() {
            unsafe { drop_in_place(self.data.offset(x as isize)); }
        }

        dealloc(self.data);
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { from_raw_parts(self.data, self.len.get() as usize) }
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { from_raw_parts_mut(self.data, self.len.get() as usize) }
    }
}

