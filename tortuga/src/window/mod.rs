#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

// #[cfg(target_os = "linux")]
// use xcb_h::{xcb_connection_t, xcb_window_t};

pub fn create_window(title: &str, width: u16, height: u16) -> impl Window {
    native::create_window(title, width, height)
}

pub trait Window {
    fn should_close(&self) -> bool;
    fn show(&self);
    fn update(&self);

    // #[cfg(target_os = "linux")]
    // fn get_os_details(&self) -> (xcb_connection_t, xcb_window_t);
}
