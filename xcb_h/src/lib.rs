#![no_std]
#![allow(non_camel_case_types)]

#[link(name = "xcb")]
extern {}

include!(concat!(env!("OUT_DIR"), "/xcb_bindings.rs"));
