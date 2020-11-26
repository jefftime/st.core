use crate::stdlib::{free, posix_memalign};
use core::{
    alloc::Layout,
    mem::MaybeUninit,
    ops::{Deref, DerefMut, Drop}
};

pub struct Box<T> {
    data: *mut T
}

impl<T> Box<T> {
    pub fn new(value: T) -> Box<T> {
        let data = alloc::<T>(1).unwrap();
        unsafe {
            data.write(value);
        }

        Box { data: data }
    }

    pub fn try_new(value: T) -> Option<Box<T>> {
        match alloc::<T>(1) {
            None => None,
            Some(data) => {
                unsafe {
                    data.write(value);
                    Some(Box { data: data })
                }
            }
        }
    }

    pub fn from_raw(ptr: *mut T) -> Box<T> {
        Box { data: ptr }
    }

    pub fn into_raw(this: Box<T>) -> *mut T {
        let result = this.data;
        core::mem::forget(this);
        result
    }

    pub fn as_ptr(this: &Box<T>) -> *mut T {
        this.data
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        dealloc(self.data);
    }
}

impl<T> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.data
        }
    }
}

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.data
        }
    }
}

#[cfg(target_os="linux")]
pub fn alloc<T>(count: usize) -> Option<*mut T> {
    let mut ptr: MaybeUninit<*mut T> = MaybeUninit::uninit();
    unsafe {
        let layout = Layout::new::<T>();
        let align = if layout.align() < 8 { 8 } else { layout.align() };
        let size = count * layout.size();

        let rc = posix_memalign(ptr.as_mut_ptr() as *mut *mut _, align, size);
        if rc != 0 {
            return None;
        }
    }

    Some(unsafe { ptr.assume_init() })
}

pub fn dealloc<T>(ptr: *mut T) {
    unsafe {
        free(ptr as *mut _);
    }
}
