use crate::types::*;

#[cfg(target_os="linux")] pub const STDIN: c_int = 0;
#[cfg(target_os="linux")] pub const STDOUT: c_int = 1;
#[cfg(target_os="linux")] pub const STDERR: c_int = 2;

#[link(name="c")]
extern {
    pub fn write(fd: c_int, buf: *const c_void, count: usize) -> c_int;
    pub fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut c_void;
    pub fn fread(
        ptr: *mut c_void,
        size: usize,
        nmemb: usize,
        stream :*mut c_void
    ) -> usize;
    pub fn fwrite(
        ptr: *const c_void,
        size: usize,
        nmemb: usize,
        stream: *mut c_void
    ) -> usize;
    pub fn fclose(stream: *mut c_void) -> c_int;
}

