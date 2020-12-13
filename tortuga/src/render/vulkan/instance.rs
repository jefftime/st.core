use c::types::*;
use dl::SharedLibrary;
use lstd::prelude::*;
use vulkan_h::*;
use core::{
    cell::RefCell,
    mem::{MaybeUninit, transmute},
    ops::Drop,
    ptr::{null, null_mut}
};

pub struct Instance {
    pub libvulkan: Rc<RefCell<SharedLibrary>>,
    pub instance: VkInstance,
    pub get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    pub create_instance: PFN_vkCreateInstance,
    pub enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
    pub destroy_instance: PFN_vkDestroyInstance,
    pub create_xcb_surface: PFN_vkCreateXcbSurfaceKHR,
    pub destroy_surface: PFN_vkDestroySurfaceKHR,
    pub get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties,
    pub get_physical_device_queue_family_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    pub get_physical_device_surface_support: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    pub create_device: PFN_vkCreateDevice,
    pub destroy_device: PFN_vkDestroyDevice,
    pub get_device_proc_addr: PFN_vkGetDeviceProcAddr,
}

impl Instance {
    pub fn new(extensions: &[*const c_char]) -> Option<Instance> {
        let libvulkan = SharedLibrary::open(cstr!("libvulkan.so"))?;
        let symbol = libvulkan.symbol(cstr!("vkGetInstanceProcAddr"))?;
        let get_instance_proc_addr = unsafe {
            type T = PFN_vkGetInstanceProcAddr;
            transmute::<*mut c_void, T>(symbol)
        };

        let vk_create_instance =
            load_preinstance_functions(get_instance_proc_addr)?;

        let instance = create_instance(
            vk_create_instance,
            &extensions
        )?;

        let (
            vk_destroy_instance,
            vk_create_xcb_surface,
            vk_destroy_surface,
            vk_enumerate_physical_devices,
            vk_get_physical_device_properties,
            vk_get_physical_device_queue_family_properties,
            vk_get_physical_device_surface_support,
            vk_create_device,
            vk_destroy_device,
            vk_get_device_proc_addr
        ) = load_instance_functions(
            &instance,
            get_instance_proc_addr
        )?;

        Some(Instance {
            libvulkan: Rc::new(RefCell::new(libvulkan)),
            instance: instance,
            get_instance_proc_addr: get_instance_proc_addr,
            create_instance: vk_create_instance,
            destroy_instance: vk_destroy_instance,
            enumerate_physical_devices: vk_enumerate_physical_devices,
            create_xcb_surface: vk_create_xcb_surface,
            destroy_surface: vk_destroy_surface,
            get_physical_device_properties: vk_get_physical_device_properties,
            get_physical_device_queue_family_properties: vk_get_physical_device_queue_family_properties,
            get_physical_device_surface_support: vk_get_physical_device_surface_support,
            create_device: vk_create_device,
            destroy_device: vk_destroy_device,
            get_device_proc_addr: vk_get_device_proc_addr
        })
    }
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
) -> Option<PFN_vkCreateInstance> {
    macro_rules! load {
        ($type:ident) => {{
            match load_vulkan_function!(null_mut(), f, $type) {
                Some(ref f) => Some(*f),
                None => return None
            }
        }}
    }

    Some(
        load!(PFN_vkCreateInstance)
    )
}

fn load_instance_functions(
    instance: &VkInstance,
    f: PFN_vkGetInstanceProcAddr
) -> Option<(
    PFN_vkDestroyInstance,
    PFN_vkCreateXcbSurfaceKHR,
    PFN_vkDestroySurfaceKHR,
    PFN_vkEnumeratePhysicalDevices,
    PFN_vkGetPhysicalDeviceProperties,
    PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    PFN_vkCreateDevice,
    PFN_vkDestroyDevice,
    PFN_vkGetDeviceProcAddr
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
        load!(PFN_vkGetPhysicalDeviceProperties),
        load!(PFN_vkGetPhysicalDeviceQueueFamilyProperties),
        load!(PFN_vkGetPhysicalDeviceSurfaceSupportKHR),
        load!(PFN_vkCreateDevice),
        load!(PFN_vkDestroyDevice),
        load!(PFN_vkGetDeviceProcAddr)
    ))
}

