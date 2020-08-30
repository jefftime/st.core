#![no_std]
#![no_main]

use core::{
    cell::RefCell,
    panic::PanicInfo
};
use c::types::{c_int, c_char};
use lstd::{abort, println};
use tortuga::window::{Window, create_window};

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! { abort() }

#[no_mangle]
extern fn main(_: c_int, _: *const *const c_char) -> c_int {
    // let x = RefCell::new(10_i32);
    let window = create_window("Test", 640, 480);

    'main: loop {
        if window.should_close() { break 'main; }
        println!("after should close");
        println!("another");
        window.update();
    }

    0
}
