use crate::alloc::{alloc, dealloc};
use core::{
    cell::Cell,
    ops::{Deref, DerefMut, Drop},
    ptr::drop_in_place,
    slice::{from_raw_parts, from_raw_parts_mut}
};

pub struct Array<T: Copy> {
    data: *mut T,
    len: usize
}

impl<T: Copy> Array<T> {
    pub fn new(initial_value: T, count: usize) -> Array<T> {
        let data: *mut T = alloc(count).unwrap();
        for x in 0..count {
            unsafe { data.offset(x as isize).write(initial_value); }
        }

        Array {
            data: data,
            len: count
        }
    }

    pub fn from_slice(array: &[T]) -> Array<T> {
        let data: *mut T = alloc(array.len()).unwrap();
        for (i, x) in array.iter().enumerate() {
            unsafe { data.offset(i as isize).write(*x); }
        }

        Array {
            data: data,
            len: array.len()
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Copy> Drop for Array<T> {
    fn drop(&mut self) {
        for x in 0..self.len {
            unsafe { drop_in_place(self.data.offset(x as isize)); }
        }

        dealloc(self.data);
    }
}

impl<T: Copy> Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { from_raw_parts(self.data, self.len) }
    }
}

impl<T: Copy> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { from_raw_parts_mut(self.data, self.len) }
    }
}

pub struct Rc<T> {
    counter: *mut usize,
    data: *mut T
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Option<Rc<T>> {
        let counter = alloc::<usize>(1)?;
        unsafe { *counter = 1; }

        let data = alloc(1)?;
        unsafe { data.write(value); }
        Some(Rc { counter: counter, data: data })
    }

    pub fn clone(this: &Rc<T>) -> Rc<T> {
        unsafe { *this.counter += 1; }

        Rc {
            counter: this.counter,
            data: this.data
        }
    }

    pub fn as_ptr(this: &Rc<T>) -> *mut T {
        this.data
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe { *self.counter -= 1; }

        if unsafe { *self.counter } == 0 {
            dealloc(self.counter);
            dealloc(self.data);
        }
    }
}
