use crate::render::physical_device::PhysicalDevice;
use c::types::*;
use dl::SharedLibrary;
use lstd::prelude::*;
use vulkan_h::*;
use core::{
    mem::{MaybeUninit, transmute},
    ops::Drop,
    ptr::{null, null_mut}
};

pub struct Instance {
    pub instance: VkInstance,
    pub get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    pub create_instance: PFN_vkCreateInstance,
    pub enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties,
    pub enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
    pub destroy_instance: PFN_vkDestroyInstance,
    pub create_xcb_surface: PFN_vkCreateXcbSurfaceKHR,
    pub destroy_surface: PFN_vkDestroySurfaceKHR,
    pub get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties
}

// Bindgen can't rustify VK_MAKE_VERSION
macro_rules! make_version {
    ($major:literal, $minor:literal, $patch:literal) => {{
        let major: u32 = $major;
        let minor: u32 = $minor;
        let patch: u32 = $patch;

        (major << 22) | (minor << 12) | patch
    }}
}

macro_rules! load_vulkan_function {
    ($instance:expr, $f:expr, $type:ident) => {{
        let f = $f?;
        let instance: VkInstance = $instance;

        type T = PFN_vkVoidFunction;
        type U = $type;
        let symbol_str = &concat!(stringify!($type), "\0")[4..];

        unsafe {
            match f(instance, symbol_str.as_ptr() as *const _) {
                Some(ref f) => transmute::<T, U>(Some(*f)),
                None => None
            }
        }
    }}
}


impl Instance {
    pub fn new(
        get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
        extensions: &[*const c_char]
    ) -> Option<Instance> {
        let extensions = [
            VK_KHR_SURFACE_EXTENSION_NAME.as_ptr() as *const c_char,
            VK_KHR_XCB_SURFACE_EXTENSION_NAME.as_ptr() as *const c_char
        ];

        let libvulkan = SharedLibrary::open(cstr!("libvulkan.so"))?;
        let symbol = libvulkan.symbol(cstr!("vkGetInstanceProcAddr"))?;
        let get_instance_proc_addr = unsafe {
            type T = PFN_vkGetInstanceProcAddr;
            transmute::<*mut c_void, T>(symbol)
        };

        let (
            vk_create_instance,
            vk_enumerate_instance_extension_properties
        ) = load_preinstance_functions(get_instance_proc_addr)?;

        let instance = create_instance(
            vk_create_instance,
            &extensions
        )?;

        let (
            vk_destroy_instance,
            vk_create_xcb_surface,
            vk_destroy_surface,
            vk_enumerate_physical_devices,
            vk_get_physical_device_properties
        ) = load_instance_functions(
            &instance,
            get_instance_proc_addr
        )?;

        Some(Instance {
            instance: instance,
            get_instance_proc_addr: get_instance_proc_addr,
            create_instance: vk_create_instance,
            destroy_instance: vk_destroy_instance,
            enumerate_instance_extension_properties: None,
            enumerate_physical_devices: vk_enumerate_physical_devices,
            create_xcb_surface: vk_create_xcb_surface,
            destroy_surface: vk_destroy_surface,
            get_physical_device_properties: vk_get_physical_device_properties
        })
    }

    // pub fn devices() -> Array<PhysicalDevice> {

    // }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if let Some(f) = self.destroy_instance {
            unsafe {
                f(
                    self.instance,
                    null_mut()
                );
            }
        }
    }
}

fn create_instance(
    vk_create_instance: PFN_vkCreateInstance,
    extensions: &[*const c_char]
) -> Option<VkInstance> {
    let layers = {
        #[cfg(not(debug_assertions))]
        let layers = [];

        #[cfg(debug_assertions)]
        let layers = [
            cstr!("VK_LAYER_LUNARG_standard_validation")
        ];

        layers
    };

    let app_name = cstr!("Tortuga");
    let engine_name = cstr!("Tortuga");
    let app_info = VkApplicationInfo {
        sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: null(),
        pApplicationName: app_name,
        applicationVersion: make_version!(1, 0 ,0),
        pEngineName: engine_name,
        engineVersion: make_version!(1, 0, 0),
        apiVersion: make_version!(1, 0, 0)
    };
    let create_info = VkInstanceCreateInfo {
        sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: null(),
        flags: 0,
        pApplicationInfo: &app_info as *const _,
        enabledExtensionCount: extensions.len() as u32,
        ppEnabledExtensionNames: if extensions.len() > 0 {
            extensions.as_ptr()
        } else {
            null()
        },
        enabledLayerCount: layers.len() as u32,
        ppEnabledLayerNames: if layers.len() > 0 {
            layers.as_ptr()
        } else {
            null()
        }
    };

    Some(unsafe {
        let mut instance: MaybeUninit<VkInstance> = MaybeUninit::uninit();
        let result = vk_create_instance?(
            &create_info as *const _,
            null_mut(),
            instance.as_mut_ptr()
        );
        if result != VK_SUCCESS { return None; }
        instance.assume_init()
    })
}

fn load_preinstance_functions(
    f: PFN_vkGetInstanceProcAddr
) -> Option<(
    PFN_vkCreateInstance,
    PFN_vkEnumerateInstanceExtensionProperties
)> {
    macro_rules! load {
        ($type:ident) => {{
            match load_vulkan_function!(null_mut(), f, $type) {
                Some(ref f) => Some(*f),
                None => return None
            }
        }}
    }

    Some((
        load!(PFN_vkCreateInstance),
        load!(PFN_vkEnumerateInstanceExtensionProperties)
    ))
}

fn load_instance_functions(
    instance: &VkInstance,
    f: PFN_vkGetInstanceProcAddr
) -> Option<(
    PFN_vkDestroyInstance,
    PFN_vkCreateXcbSurfaceKHR,
    PFN_vkDestroySurfaceKHR,
    PFN_vkEnumeratePhysicalDevices,
    PFN_vkGetPhysicalDeviceProperties
)> {
    macro_rules! load {
        ($type:ident) => {{
            match load_vulkan_function!(*instance, f, $type) {
                Some(ref f) => Some(*f),
                None => return None
            }
        }}
    }

    Some((
        load!(PFN_vkDestroyInstance),
        load!(PFN_vkCreateXcbSurfaceKHR),
        load!(PFN_vkDestroySurfaceKHR),
        load!(PFN_vkEnumeratePhysicalDevices),
        load!(PFN_vkGetPhysicalDeviceProperties)
    ))
}

