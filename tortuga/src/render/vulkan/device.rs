use crate::render::vulkan::{
    instance::Instance,
    surface::Surface,
    physical_device::PhysicalDevice
};
use c::types::*;
use core::{
    cell::RefCell,
    mem::MaybeUninit,
    ops::Drop,
    ptr::null_mut
};
use lstd::prelude::*;
use vulkan_h::*;

pub struct Device {
    instance: Rc<RefCell<Instance>>,
    pub device: VkDevice,
    pub graphics_index: u32,
    pub present_index: u32,
    pub get_device_queue: PFN_vkGetDeviceQueue,
    pub create_semaphore: PFN_vkCreateSemaphore,
    pub destroy_semaphore: PFN_vkDestroySemaphore,
    pub create_pipeline_layout: PFN_vkCreatePipelineLayout,
    pub destroy_pipeline_layout: PFN_vkDestroyPipelineLayout,
    pub create_shader_module: PFN_vkCreateShaderModule,
    pub destroy_shader_module: PFN_vkDestroyShaderModule,
    pub create_render_pass: PFN_vkCreateRenderPass,
    pub destroy_render_pass: PFN_vkDestroyRenderPass,
    pub create_graphics_pipelines: PFN_vkCreateGraphicsPipelines,
    pub destroy_pipeline: PFN_vkDestroyPipeline,
    pub create_framebuffer: PFN_vkCreateFramebuffer,
    pub destroy_framebuffer: PFN_vkDestroyFramebuffer,
    pub create_image_view: PFN_vkCreateImageView,
    pub destroy_image_view: PFN_vkDestroyImageView,
    pub create_command_pool: PFN_vkCreateCommandPool,
    pub destroy_command_pool: PFN_vkDestroyCommandPool,
    pub allocate_command_buffers: PFN_vkAllocateCommandBuffers,
    pub free_command_buffers: PFN_vkFreeCommandBuffers,
    pub begin_command_buffer: PFN_vkBeginCommandBuffer,
    pub end_command_buffer: PFN_vkEndCommandBuffer,
    pub cmd_begin_render_pass: PFN_vkCmdBeginRenderPass,
    pub cmd_end_render_pass: PFN_vkCmdEndRenderPass,
    pub cmd_bind_pipeline: PFN_vkCmdBindPipeline,
    pub cmd_bind_vertex_buffers: PFN_vkCmdBindVertexBuffers,
    pub cmd_bind_index_buffer: PFN_vkCmdBindIndexBuffer,
    pub cmd_draw_indexed: PFN_vkCmdDrawIndexed
}

impl Device {
    pub fn new(
        instance: &Rc<RefCell<Instance>>,
        surface: &Rc<RefCell<Surface>>,
        physical_device: &PhysicalDevice
    ) -> Option<Device> {
        let extensions = [
            VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr() as *const c_char
        ];

        let (graphics_index, present_index) =
            get_queue_information(
                instance,
                surface,
                physical_device
            )?;

        let device = create_device(
            &instance.borrow(),
            physical_device,
            &extensions,
            graphics_index,
            present_index
        )?;

        let (
            vk_get_device_queue,
            vk_create_semaphore,
            vk_destroy_semaphore,
            vk_create_pipeline_layout,
            vk_destroy_pipeline_layout,
            vk_create_shader_module,
            vk_destroy_shader_module,
            vk_create_render_pass,
            vk_destroy_render_pass,
            vk_create_graphics_pipelines,
            vk_destroy_pipeline,
            vk_create_framebuffer,
            vk_destroy_framebuffer,
            vk_create_image_view,
            vk_destroy_image_view,
            vk_create_command_pool,
            vk_destroy_command_pool,
            vk_allocate_command_buffers,
            vk_free_command_buffers,
            vk_begin_command_buffer,
            vk_end_command_buffer,
            vk_cmd_begin_render_pass,
            vk_cmd_end_render_pass,
            vk_cmd_bind_pipeline,
            vk_cmd_bind_vertex_buffers,
            vk_cmd_bind_index_buffer,
            vk_cmd_draw_indexed
        ) = load_device_functions(&instance.borrow(), &device)?;

        Some(Device {
            instance: Rc::clone(instance),
            device: device,
            graphics_index: graphics_index,
            present_index: present_index,
            get_device_queue: vk_get_device_queue,
            create_semaphore: vk_create_semaphore,
            destroy_semaphore: vk_destroy_semaphore,
            create_pipeline_layout: vk_create_pipeline_layout,
            destroy_pipeline_layout: vk_destroy_pipeline_layout,
            create_shader_module: vk_create_shader_module,
            destroy_shader_module: vk_destroy_shader_module,
            create_render_pass: vk_create_render_pass,
            destroy_render_pass: vk_destroy_render_pass,
            create_graphics_pipelines: vk_create_graphics_pipelines,
            destroy_pipeline: vk_destroy_pipeline,
            create_framebuffer: vk_create_framebuffer,
            destroy_framebuffer: vk_destroy_framebuffer,
            create_image_view: vk_create_image_view,
            destroy_image_view: vk_destroy_image_view,
            create_command_pool: vk_create_command_pool,
            destroy_command_pool: vk_destroy_command_pool,
            allocate_command_buffers: vk_allocate_command_buffers,
            free_command_buffers: vk_free_command_buffers,
            begin_command_buffer: vk_begin_command_buffer,
            end_command_buffer: vk_end_command_buffer,
            cmd_begin_render_pass: vk_cmd_begin_render_pass,
            cmd_end_render_pass: vk_cmd_end_render_pass,
            cmd_bind_pipeline: vk_cmd_bind_pipeline,
            cmd_bind_vertex_buffers: vk_cmd_bind_vertex_buffers,
            cmd_bind_index_buffer: vk_cmd_bind_index_buffer,
            cmd_draw_indexed: vk_cmd_draw_indexed
        })
    }

}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            if let Some(f) = self.instance.borrow().destroy_device {
                unsafe {
                    f(self.device, null_mut());
                }
            }
        }
    }
}

fn create_device(
    instance: &Instance,
    physical_device: &PhysicalDevice,
    extensions: &[*const c_char],
    graphics_index: u32,
    present_index: u32,
) -> Option<VkDevice> {
    const priority: f32 = 1.0;
    let queue_infos = [
        VkDeviceQueueCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            flags: 0,
            pNext: null_mut(),
            queueCount: 1,
            pQueuePriorities: &priority as *const _,
            queueFamilyIndex: graphics_index
        },
        VkDeviceQueueCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            flags: 0,
            pNext: null_mut(),
            queueCount: 1,
            pQueuePriorities: &priority as *const _,
            queueFamilyIndex: present_index
        }
    ];

    let n_queues = if graphics_index != present_index {
        2
    } else {
        1
    };

    let features = unsafe { MaybeUninit::zeroed().assume_init() };

    let create_info = VkDeviceCreateInfo {
        sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        flags: 0,
        pNext: null_mut(),
        enabledLayerCount: 0,
        ppEnabledLayerNames: null_mut(),
        pEnabledFeatures: &features as *const _,
        enabledExtensionCount: extensions.len() as _,
        ppEnabledExtensionNames: extensions.as_ptr(),
        queueCreateInfoCount: n_queues,
        pQueueCreateInfos: queue_infos.as_ptr()
    };

    let device = unsafe {
        let mut device = MaybeUninit::uninit();
        let result = instance.create_device?(
            physical_device.device,
            &create_info as *const _,
            null_mut(),
            device.as_mut_ptr()
        );
        if result != VK_SUCCESS { return None; }

        device.assume_init()
    };

    Some(device)
}

fn get_queue_information(
    instance: &Rc<RefCell<Instance>>,
    surface: &Rc<RefCell<Surface>>,
    device: &PhysicalDevice
) -> Option<(u32, u32)> {
    let mut graphics_set = false;
    let mut present_set = false;
    let mut graphics_index = 0_u32;
    let mut present_index = 0_u32;

    let n_props = unsafe {
        let mut n_props = MaybeUninit::uninit();
        instance.borrow().get_physical_device_queue_family_properties?(
            device.device as *const _ as *mut _,
            n_props.as_mut_ptr(),
            null_mut()
        );
        n_props.assume_init()
    };
    if n_props == 0 { return None; }
    let mut props: Array<VkQueueFamilyProperties> =
        Array::new(n_props as usize);
    unsafe {
        instance.borrow().get_physical_device_queue_family_properties?(
            device.device as *const _ as *mut _,
            &n_props as *const _ as *mut _,
            props.as_mut_ptr()
        );
        props.assume_init(n_props as isize);
    }

    for (i, prop) in props.iter().enumerate() {
        let is_graphics = prop.queueCount > 0
            && (prop.queueFlags & VK_QUEUE_GRAPHICS_BIT != 0);
        if is_graphics {
            graphics_set = true;
            graphics_index = i as u32;
        }

        let mut present_support = 0_u32;
        let result = unsafe {
            instance.borrow().get_physical_device_surface_support?(
                device.device,
                i as u32,
                surface.borrow().surface,
                &mut present_support as *mut _
            )
        };
        if result != VK_SUCCESS { return None; }

        if prop.queueCount > 0 && present_support != 0 {
            present_set = true;
            present_index = i as u32;
        }
    }

    if !graphics_set || !present_set {
        None
    } else {
        Some((graphics_index, present_index))
    }
}

fn load_device_functions(
    instance: &Instance,
    device: &VkDevice
) -> Option<(
    PFN_vkGetDeviceQueue,
    PFN_vkCreateSemaphore,
    PFN_vkDestroySemaphore,
    PFN_vkCreatePipelineLayout,
    PFN_vkDestroyPipelineLayout,
    PFN_vkCreateShaderModule,
    PFN_vkDestroyShaderModule,
    PFN_vkCreateRenderPass,
    PFN_vkDestroyRenderPass,
    PFN_vkCreateGraphicsPipelines,
    PFN_vkDestroyPipeline,
    PFN_vkCreateFramebuffer,
    PFN_vkDestroyFramebuffer,
    PFN_vkCreateImageView,
    PFN_vkDestroyImageView,
    PFN_vkCreateCommandPool,
    PFN_vkDestroyCommandPool,
    PFN_vkAllocateCommandBuffers,
    PFN_vkFreeCommandBuffers,
    PFN_vkBeginCommandBuffer,
    PFN_vkEndCommandBuffer,
    PFN_vkCmdBeginRenderPass,
    PFN_vkCmdEndRenderPass,
    PFN_vkCmdBindPipeline,
    PFN_vkCmdBindVertexBuffers,
    PFN_vkCmdBindIndexBuffer,
    PFN_vkCmdDrawIndexed
)> {
    macro_rules! load {
        ($type:ident) => {{
            match load_vulkan_function!(
                *device,
                instance.get_device_proc_addr,
                $type
            ) {
                Some(ref f) => Some(*f),
                None => return None
            }
        }}
    }

    Some((
        load!(PFN_vkGetDeviceQueue),
        load!(PFN_vkCreateSemaphore),
        load!(PFN_vkDestroySemaphore),
        load!(PFN_vkCreatePipelineLayout),
        load!(PFN_vkDestroyPipelineLayout),
        load!(PFN_vkCreateShaderModule),
        load!(PFN_vkDestroyShaderModule),
        load!(PFN_vkCreateRenderPass),
        load!(PFN_vkDestroyRenderPass),
        load!(PFN_vkCreateGraphicsPipelines),
        load!(PFN_vkDestroyPipeline),
        load!(PFN_vkCreateFramebuffer),
        load!(PFN_vkDestroyFramebuffer),
        load!(PFN_vkCreateImageView),
        load!(PFN_vkDestroyImageView),
        load!(PFN_vkCreateCommandPool),
        load!(PFN_vkDestroyCommandPool),
        load!(PFN_vkAllocateCommandBuffers),
        load!(PFN_vkFreeCommandBuffers),
        load!(PFN_vkBeginCommandBuffer),
        load!(PFN_vkEndCommandBuffer),
        load!(PFN_vkCmdBeginRenderPass),
        load!(PFN_vkCmdEndRenderPass),
        load!(PFN_vkCmdBindPipeline),
        load!(PFN_vkCmdBindVertexBuffers),
        load!(PFN_vkCmdBindIndexBuffer),
        load!(PFN_vkCmdDrawIndexed)
    ))
}
