use crate::{
    alloc::{alloc, dealloc, Box}
};
use core::{
    cell::Cell,
    mem::ManuallyDrop,
    ops::{Deref, Drop}
};

pub struct Rc<T> {
    counter: *mut Cell<usize>,
    data: ManuallyDrop<Box<T>>
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Rc<T> {
        let counter: *mut Cell<usize> = alloc(1).unwrap();
        let data = ManuallyDrop::new(Box::new(value));

        unsafe { counter.write(Cell::new(1)); }

        Rc { counter: counter, data: data }
    }

    pub fn clone(this: &Rc<T>) -> Rc<T> {
        unsafe {
            *(*this.counter).get_mut() += 1;
        }

        let data =
            ManuallyDrop::new(Box::from_raw(Box::as_ptr(&this.data)));
        Rc {
            counter: this.counter,
            data: data,
        }
    }

    pub fn as_ptr(this: &Rc<T>) -> *mut T {
        Box::as_ptr(&this.data)
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            *(*self.counter).get_mut() -= 1;
            if (*self.counter).get() == 0 {
                dealloc(self.counter);
                ManuallyDrop::drop(&mut self.data);
            }
        }
    }
}
