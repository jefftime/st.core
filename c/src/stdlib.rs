use crate::types::*;

#[link(name="c")]
extern {
    pub fn abort() -> !;
    pub fn free(ptr: *mut c_void);

    #[cfg(target_os="linux")]
    pub fn posix_memalign(
        memptr: *mut *mut c_void,
        alignment: usize,
        size: usize
    ) -> c_int;
}
