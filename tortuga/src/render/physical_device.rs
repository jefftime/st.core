use crate::render::instance::Instance;
use core::{
    cell::RefCell,
    mem::MaybeUninit
};
use lstd::prelude::*;
use vulkan_h::*;

#[derive(Debug)]
pub enum PhysicalDeviceType {
    Integrated,
    Discrete,
    Unknown
}

pub struct PhysicalDevice {
    _instance: Rc<RefCell<Instance>>,
    pub device: VkPhysicalDevice,
    pub props: VkPhysicalDeviceProperties
}

impl PhysicalDevice {
    pub fn new(
        instance: &Rc<RefCell<Instance>>,
        device: VkPhysicalDevice
    ) -> PhysicalDevice {
        let props = unsafe {
            let mut props: MaybeUninit<VkPhysicalDeviceProperties> =
                MaybeUninit::uninit();
            if let Some(f) = instance.borrow().get_physical_device_properties {
                f(device, props.as_mut_ptr());
            }
            props.assume_init()
        };

        PhysicalDevice {
            _instance: Rc::clone(instance),
            device: device,
            props: props
        }
    }

    pub fn name(&self) -> &str {
        core::str::from_utf8(
            unsafe {
                core::mem::transmute::<&[i8], &[u8]>(
                    &self.props.deviceName
                )
            }
        ).unwrap()
    }

    pub fn kind(&self) -> PhysicalDeviceType {
        match self.props.deviceType {
            a if a == VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU =>
                PhysicalDeviceType::Integrated,
            a if a == VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU =>
                PhysicalDeviceType::Discrete,
            _ =>
                PhysicalDeviceType::Unknown
        }
    }
}
