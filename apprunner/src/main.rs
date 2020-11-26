#![no_std]
#![no_main]

use core::panic::PanicInfo;
use c::types::{c_int, c_char};
use lstd::{abort, println};
use tortuga::{
    window::{Window, create_window},
    render::Context
};

#[link(name = "asan")]
extern {}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    #[cfg(debug_assertions)]
    if let Some(location) = info.location() {
        println!("Panic in {}:{}", location.file(), location.line())
    }

    abort()
}

#[no_mangle]
extern fn main(_: c_int, _: *const *const c_char) -> c_int {
    let window = create_window("Test", 640, 480).unwrap();
    let context = Context::new(&window).unwrap();
    let physical_devices = context.get_physical_devices().unwrap();

    for pd in physical_devices.iter() {
        println!("device!");
    }

    'main: loop {
        if window.should_close() {
            break 'main;
        }

        window.update();
    }

    0
}
