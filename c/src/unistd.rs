use crate::types::*;

#[link(name="c")]
extern {
    pub fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
}
