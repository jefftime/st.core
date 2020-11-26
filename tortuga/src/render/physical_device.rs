use crate::render::instance::Instance;
use core::{
    cell::RefCell,
};
use lstd::prelude::*;
use vulkan_h::*;

pub struct PhysicalDevice {
    instance: Rc<RefCell<Instance>>,
    device: VkPhysicalDevice,
}

impl PhysicalDevice {
    pub fn new(
        instance: &Rc<RefCell<Instance>>,
        device: VkPhysicalDevice
    ) -> PhysicalDevice {
        // let props = instance.
        PhysicalDevice {
            instance: Rc::clone(instance),
            device: device,
            // props: props
        }
    }
}
