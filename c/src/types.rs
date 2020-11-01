// TODO: Put these behind cfg(target_os) gates

#![allow(non_camel_case_types)]

pub type c_char      = i8;
pub type c_short     = i16;
pub type c_int       = i32;
pub type c_long      = i64;
pub type c_longlong  = i128;
pub type c_uchar     = u8;
pub type c_ushort    = u16;
pub type c_uint      = u32;
pub type c_ulong     = u64;
pub type c_ulonglong = u128;
pub type c_void      = core::ffi::c_void;
