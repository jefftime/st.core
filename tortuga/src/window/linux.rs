use crate::window::Window;
use lstd::{
    alloc::dealloc,
    container::Array,
    println
};
use core::{
    cell::Cell,
    mem::{MaybeUninit, transmute},
    ops::Drop,
    ptr::{null, null_mut},
    str::from_utf8_unchecked
};
use xcb_h::*;

pub fn create_window(
    title: &str,
    width: u16,
    height :u16
) -> Option<NativeWindow> {
    let title: Array<u8> = {
        let mut new_title = Array::new(b'\0', title.len() + 1);
        for (i, c) in title.as_bytes().iter().enumerate() {
            new_title[i] = *c;
        }

        new_title
    };

    NativeWindow::new(
        unsafe { from_utf8_unchecked(&title) },
        width,
        height
    )
}

pub struct NativeWindow {
    connection: *const xcb_connection_t,
    window: xcb_window_t,
    delete_atom: xcb_atom_t,
    width: Cell<u16>,
    height: Cell<u16>,
    should_close: Cell<bool>
}

impl NativeWindow {
    pub fn new(
        title: &str,
        width: u16,
        height: u16
    ) -> Option<NativeWindow> {
        let (cn, setup, screen) = NativeWindow::setup_connection()?;
        let wn = NativeWindow::setup_window(
            cn,
            setup,
            screen,
            title,
            width,
            height
        );
        let delete_atom = NativeWindow::setup_atoms(cn, wn)?;
        unsafe { xcb_flush(cn as *mut _); }

        Some(NativeWindow {
            connection: cn,
            window: wn,
            delete_atom: delete_atom,
            width: Cell::new(width),
            height: Cell::new(height),
            should_close: Cell::new(false)
        })
    }

    fn setup_connection() -> Option<(
        *const xcb_connection_t,
        *const xcb_setup_t,
        *const xcb_screen_t
    )> {
        let (cn, index) = unsafe {
            let mut index = MaybeUninit::uninit();
            let cn = xcb_connect(null_mut(), index.as_mut_ptr());
            (cn as *const _, index.assume_init())
        };
        if cn == null() { return None; }

        // Get the information for the screen that initiated the connection
        let (setup, screen) = unsafe {
            let setup = xcb_get_setup(cn as *mut _);
            let mut screen_iter =
                xcb_setup_roots_iterator(setup);
            for _ in 0..index {
                xcb_screen_next(&mut screen_iter as *mut _);
            }

            (setup, screen_iter.data)
        };

        Some((cn, setup, screen))
    }

    fn setup_window(
        cn: *const xcb_connection_t,
        setup: *const xcb_setup_t,
        screen: *const xcb_screen_t,
        title: &str,
        width: u16,
        height: u16
    ) -> xcb_window_t {
        let cn = cn as *mut _;
        unsafe {
            let wn = xcb_generate_id(cn);
            let mask = XCB_CW_EVENT_MASK;
            let values = [XCB_EVENT_MASK_STRUCTURE_NOTIFY];
            xcb_create_window(
                cn,
                XCB_COPY_FROM_PARENT as u8,
                wn,
                (*screen).root,
                0, 0,
                width, height,
                0,
                XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
                (*screen).root_visual,
                mask,
                values.as_ptr() as *mut _
            );

            // Window title
            xcb_change_property(
                cn,
                XCB_PROP_MODE_REPLACE as u8,
                wn,
                XCB_ATOM_WM_NAME,
                XCB_ATOM_STRING,
                8,
                title.as_bytes().len() as u32,
                title.as_ptr() as *const _
            );

            xcb_map_window(cn, wn);
            wn
        }
    }

    fn setup_atoms(
        cn: *const xcb_connection_t,
        wn: xcb_window_t
    ) -> Option<xcb_atom_t> {
        let cn = cn as *mut _;

        // We want to watch for the delete window event
        unsafe {
            let wm_protocols = b"WM_PROTOCOLS";
            let wm_delete_window = b"WM_DELETE_WINDOW";
            let p_ck = xcb_intern_atom(
                cn,
                0,
                wm_protocols.len() as u16,
                wm_protocols.as_ptr() as *const _
            );
            let d_ck = xcb_intern_atom(
                cn,
                0,
                wm_delete_window.len() as u16,
                wm_delete_window.as_ptr() as *const _
            );

            let protocol = xcb_intern_atom_reply(cn, p_ck, null_mut());
            let delete = xcb_intern_atom_reply(cn, d_ck, null_mut());
            if protocol == null_mut() { return None; }
            if delete == null_mut() {
                dealloc(protocol);
                return None;
            }
            xcb_change_property(
                cn,
                XCB_PROP_MODE_REPLACE as u8,
                wn,
                (*protocol).atom,
                XCB_ATOM_ATOM,
                32,
                1,
                &mut (*delete).atom as *mut u32 as *mut _
            );
            let delete_atom = (*delete).atom;
            dealloc(protocol);
            dealloc(delete);

            Some(delete_atom)
        }
    }
}

impl Window for NativeWindow {
    fn should_close(&self) -> bool {
        self.should_close.get()
    }

    fn update(&self) {
        let cn = self.connection as *mut _;
        let mut event = unsafe { xcb_poll_for_event(cn) };
        while event != null_mut() {
            match unsafe { (*event).response_type & !0x80 } {
                // Resize
                a if a == XCB_CONFIGURE_NOTIFY as u8 => {
                    type T = *mut xcb_generic_event_t;
                    type U = *mut xcb_configure_notify_event_t;
                    let e = unsafe { transmute::<T, U>(event) };
                    let (width, height) = unsafe {
                        ((*e).width, (*e).height)
                    };

                    if width != self.width.get() || height != self.height.get() {
                        self.width.set(width);
                        self.height.set(height);
                    }
                },

                // Close window
                a if a == XCB_CLIENT_MESSAGE as u8 => {
                    type T = *mut xcb_generic_event_t;
                    type U = *mut xcb_client_message_event_t;
                    let e = unsafe { transmute::<T, U>(event) };

                    let atom = unsafe { (*e).data.data32[0] };
                    if atom == self.delete_atom {
                        self.should_close.set(true);
                    }
                },

                _ => {}
            };

            dealloc(event);
            event = unsafe { xcb_poll_for_event(cn) };
        }
    }

    fn get_os_details(&self) -> (*const xcb_connection_t, xcb_window_t) {
        (self.connection, self.window)
    }
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        let cn = self.connection as *mut _;
        unsafe {
            xcb_destroy_window(cn, self.window);
            xcb_disconnect(cn);
        }
    }
}
