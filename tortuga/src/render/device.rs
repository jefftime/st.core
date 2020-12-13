use crate::render::vulkan::Device as VulkanDevice;
use core::{
    cell::RefCell
};
use lstd::prelude::*;

pub struct Device {
    device: Rc<RefCell<VulkanDevice>>
}

impl Device {
    fn new(device: VulkanDevice) -> Device {
        Device {
            device: Rc::new(RefCell::new(device))
        }
    }
}
