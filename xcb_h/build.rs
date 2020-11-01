use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    process::Command
};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("xcb_bindings.rs");
    match File::open(out_dir.as_path()) {
        Ok(_) => {
            println!("cargo:warning=Do not generate xcb_bindings.rs; file already exists");
        },
        _ => {
            println!("cargo:warning=Generating xcb_bindings.rs");
            let mut bw = BufWriter::new(
                File::create(out_dir.as_path()).unwrap()
            );
            let args = [
                "/usr/include/xcb/xcb.h",
                "--no-doc-comments",
                "--no-layout-tests",
                "--no-prepend-enum-name",
                "--use-core",
                "--ctypes-prefix", "c::types",
                "--whitelist-function", "xcb_connect",
                "--whitelist-function", "xcb_get_setup",
                "--whitelist-function", "xcb_setup_roots_iterator",
                "--whitelist-function", "xcb_screen_next",
                "--whitelist-function", "xcb_generate_id",
                "--whitelist-function", "xcb_create_window",
                "--whitelist-function", "xcb_change_property",
                "--whitelist-function", "xcb_intern_atom",
                "--whitelist-function", "xcb_intern_atom_reply",
                "--whitelist-function", "xcb_map_window",
                "--whitelist-function", "xcb_create_gc",
                "--whitelist-function", "xcb_flush",
                "--whitelist-function", "xcb_poll_for_event",
                "--whitelist-function", "xcb_destroy_window",
                "--whitelist-function", "xcb_disconnect",
                "--whitelist-type", "xcb_screen_iterator_t",
                "--whitelist-type", "xcb_atom_t",
                "--whitelist-type", "xcb_connection_t",
                "--whitelist-type", "xcb_window_t",
                "--whitelist-type", "xcb_cw_t",
                "--whitelist-type", "xcb_event_mask_t",
                "--whitelist-type", "xcb_window_class_t",
                "--whitelist-type", "xcb_prop_mode_t",
                "--whitelist-type", "xcb_atom_enum_t",
                "--whitelist-type", "xcb_client_message_event_t",
                "--whitelist-type", "xcb_configure_notify_event_t",
                "--whitelist-var", "XCB_COPY_FROM_PARENT",
                "--whitelist-var", "XCB_CONFIGURE_NOTIFY",
                "--whitelist-var", "XCB_CLIENT_MESSAGE"
            ];
            let output = Command::new("bindgen")
                .args(args.iter())
                .output()
                .unwrap();
            bw.write(&output.stdout).unwrap();
            bw.flush().unwrap();
        }
    }
}
