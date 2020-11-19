use c::types::c_char;
use vulkan_h::*;
use core::ops::Drop;

pub struct Instance {
    pub instance: VkInstance
}

impl Instance {
    pub fn new(
        get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
        extensions: &[*const c_char]
    ) -> Option<Instance> {
        None
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        
    }
}
