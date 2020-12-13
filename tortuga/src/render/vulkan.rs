// Bindgen can't rustify VK_MAKE_VERSION
#[macro_use]
macro_rules! make_version {
    ($major:literal, $minor:literal, $patch:literal) => {{
        let major: u32 = $major;
        let minor: u32 = $minor;
        let patch: u32 = $patch;

        (major << 22) | (minor << 12) | patch
    }}
}

#[macro_use]
macro_rules! load_vulkan_function {
    ($context:expr, $f:expr, $type:ident) => {{
        type T = PFN_vkVoidFunction;
        type U = $type;
        let symbol_str = &concat!(stringify!($type), "\0")[4..];

        if let Some(f) = $f {
            unsafe {
                match f($context, symbol_str.as_ptr() as *const _) {
                    Some(ref f) => core::mem::transmute::<T, U>(
                        Some(*f)
                    ),
                    None => None
                }
            }
        } else {
            None
        }
    }}
}

pub mod instance;
pub mod surface;
pub mod physical_device;
pub mod device;

pub use instance::Instance;
pub use surface::Surface;
pub use physical_device::PhysicalDevice;
pub use device::Device;
