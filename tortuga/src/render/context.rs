use crate::{
    render::{
        instance::Instance,
        surface::Surface,
        physical_device::PhysicalDevice
    },
    window::Window
};
use c::types::*;
use core::{
    cell::RefCell,
    mem::{MaybeUninit, ManuallyDrop, transmute},
    ops::Drop,
    ptr::null_mut
};
use dl::SharedLibrary;
use lstd::prelude::*;
use vulkan_h::*;

pub struct Context {
    libvulkan: SharedLibrary,
    instance: ManuallyDrop<Rc<RefCell<Instance>>>,
    surface: ManuallyDrop<Surface>
}

impl Context {
    pub fn new(window: &dyn Window) -> Option<Context> {
        let extensions = [
            VK_KHR_SURFACE_EXTENSION_NAME.as_ptr() as *const c_char,
            VK_KHR_XCB_SURFACE_EXTENSION_NAME.as_ptr() as *const c_char,
        ];

        let libvulkan = SharedLibrary::open(cstr!("libvulkan.so"))?;
        let symbol = libvulkan.symbol(cstr!("vkGetInstanceProcAddr"))?;
        let get_instance_proc_addr = unsafe {
            type T = PFN_vkGetInstanceProcAddr;
            transmute::<*mut c_void, T>(symbol)
        };

        let instance = Instance::new(
            get_instance_proc_addr,
            &extensions
        )?;
        let instance = Rc::new(RefCell::new(instance));
        let instance = ManuallyDrop::new(instance);

        let surface = Surface::new(&instance, window)?;
        let surface = ManuallyDrop::new(surface);
        Some(Context {
            libvulkan: libvulkan,
            instance: instance,
            surface: surface,
        })
    }

    pub fn get_physical_devices(&self) -> Option<Array<PhysicalDevice>> {
        let enumerate_devices =
            self.instance.borrow().enumerate_physical_devices;

        let mut n_devices = unsafe {
            let mut n_devices = MaybeUninit::uninit();
            let result = enumerate_devices?(
                self.instance.borrow().instance,
                n_devices.as_mut_ptr(),
                null_mut()
            );
            if result != VK_SUCCESS { return None; }

            let n_devices = n_devices.assume_init();
            if n_devices == 0 { return None; }
            n_devices
        };

        let mut vk_devices = Array::new(n_devices as usize);
        let result = unsafe {
            enumerate_devices?(
                self.instance.borrow().instance,
                &mut n_devices as *mut _,
                vk_devices.as_mut_ptr()
            )
        };
        if result != VK_SUCCESS { return None; }

        let mut physical_devices = Array::new(n_devices as usize);
        for (i, device) in vk_devices.iter().enumerate() {
            physical_devices.push(PhysicalDevice::new(
                &self.instance,
                *device
            ));
        }

        Some(physical_devices)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.surface);
            ManuallyDrop::drop(&mut self.instance);
        }
    }
}

