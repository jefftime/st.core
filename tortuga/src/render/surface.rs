use crate::{
    render::instance::Instance,
    window::Window
};
use core::{
    cell::RefCell,
    mem::MaybeUninit,
    ops::Drop,
    ptr::null_mut
};
use lstd::prelude::*;
use vulkan_h::*;

pub struct Surface {
    instance: Rc<RefCell<Instance>>,
    vk_destroy_surface: PFN_vkDestroySurfaceKHR,
    pub surface: VkSurfaceKHR
}

impl Surface {
    pub fn new(
        instance: &Rc<RefCell<Instance>>,
        window: &dyn Window,
        vk_create_surface: PFN_vkCreateXcbSurfaceKHR,
        vk_destroy_surface: PFN_vkDestroySurfaceKHR
    ) -> Option<Surface> {
        let (connection, window) = window.get_os_details();
        let create_info = VkXcbSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            flags: 0,
            pNext: null_mut(),
            connection: connection as *mut _,
            window: window
        };

        Some(Surface {
            instance: Rc::clone(instance),
            vk_destroy_surface: vk_destroy_surface,
            surface: unsafe {
                let mut surface: MaybeUninit<VkSurfaceKHR> =
                    MaybeUninit::uninit();
                let result = vk_create_surface?(
                    instance.borrow().instance,
                    &create_info as *const _ as *mut _,
                    null_mut(),
                    surface.as_mut_ptr()
                );
                if result != VK_SUCCESS { return None; }
                surface.assume_init()
            }
        })
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        if let Some(f) = self.vk_destroy_surface {
            unsafe {
                f(
                    self.instance.borrow().instance,
                    self.surface,
                    null_mut()
                );
            }
        }
    }
}
