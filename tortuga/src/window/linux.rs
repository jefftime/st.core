use core::cell::Cell;
use crate::window::Window;

pub fn create_window(
    title: &str,
    width: u16,
    height :u16
) -> NativeWindow {
    NativeWindow::new(title)
}

pub struct NativeWindow {
    should_close: Cell<bool>
}

impl NativeWindow {
    pub fn new(title: &str) -> NativeWindow {
        NativeWindow {
            should_close: Cell::new(false)
        }
    }
}

impl Window for NativeWindow {
    fn should_close(&self) -> bool {
        self.should_close.get()
    }

    fn show(&self) {}

    fn update(&self) {
        self.should_close.set(true);
    }
}
