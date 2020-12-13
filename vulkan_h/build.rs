use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    process::Command
};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("vulkan_bindings.rs");
    match File::open(out_dir.as_path()) {
        Ok(_) => {
            println!("cargo:warning=Do not generate vulkan_bindings.rs; file already exists");
        },
        _ => {
            println!("cargo:warning=Generating vulkan_bindings.rs");
            let mut bw = BufWriter::new(
                File::create(out_dir.as_path()).unwrap()
            );
            let args = [
                "/usr/include/vulkan/vulkan.h",
                "--no-doc-comments",
                "--no-layout-tests",
                "--no-prepend-enum-name",
                "--use-core",
                "--ctypes-prefix", "c::types",
                "--generate-inline-functions",
                "--whitelist-type", "VK_MAKE_VERSION",
                "--whitelist-type", "VkInstance",
                "--whitelist-type", "PFN_vkGetInstanceProcAddr",
                "--whitelist-type", "PFN_vkVoidFunction",
                "--whitelist-type", "PFN_vkCreateInstance",
                "--whitelist-type", "PFN_vkDestroyInstance",
                "--whitelist-type", "PFN_vkEnumeratePhysicalDevices",
                "--whitelist-type", "PFN_vkCreateXcbSurfaceKHR",
                "--whitelist-type", "PFN_vkDestroySurfaceKHR",
                "--whitelist-type", "PFN_vkGetPhysicalDeviceProperties",
                "--whitelist-type", "PFN_vkGetPhysicalDeviceQueueFamilyProperties",
                "--whitelist-type", "PFN_vkGetPhysicalDeviceSurfaceSupportKHR",
                "--whitelist-type", "PFN_vkCreateDevice",
                "--whitelist-type", "PFN_vkDestroyDevice",
                "--whitelist-type", "PFN_vkGetDeviceProcAddr",
                "--whitelist-type", "PFN_vkGetDeviceQueue",
                "--whitelist-type", "PFN_vkCreateSemaphore",
                "--whitelist-type", "PFN_vkDestroySemaphore",
                "--whitelist-type", "PFN_vkCreatePipelineLayout",
                "--whitelist-type", "PFN_vkDestroyPipelineLayout",
                "--whitelist-type", "PFN_vkCreateShaderModule",
                "--whitelist-type", "PFN_vkDestroyShaderModule",
                "--whitelist-type", "PFN_vkCreateRenderPass",
                "--whitelist-type", "PFN_vkDestroyRenderPass",
                "--whitelist-type", "PFN_vkCreateGraphicsPipelines",
                "--whitelist-type", "PFN_vkDestroyPipeline",
                "--whitelist-type", "PFN_vkCreateFramebuffer",
                "--whitelist-type", "PFN_vkDestroyFramebuffer",
                "--whitelist-type", "PFN_vkCreateImageView",
                "--whitelist-type", "PFN_vkDestroyImageView",
                "--whitelist-type", "PFN_vkCreateCommandPool",
                "--whitelist-type", "PFN_vkDestroyCommandPool",
                "--whitelist-type", "PFN_vkAllocateCommandBuffers",
                "--whitelist-type", "PFN_vkFreeCommandBuffers",
                "--whitelist-type", "PFN_vkBeginCommandBuffer",
                "--whitelist-type", "PFN_vkEndCommandBuffer",
                "--whitelist-type", "PFN_vkCmdBeginRenderPass",
                "--whitelist-type", "PFN_vkCmdEndRenderPass",
                "--whitelist-type", "PFN_vkCmdBindPipeline",
                "--whitelist-type", "PFN_vkCmdBindVertexBuffers",
                "--whitelist-type", "PFN_vkCmdBindIndexBuffer",
                "--whitelist-type", "PFN_vkCmdDrawIndexed",
                "--whitelist-type", "VkResult",
                "--whitelist-type", "VkInstanceCreateInfo",
                "--whitelist-type", "VkApplicationInfo",
                "--whitelist-type", "VK_API_VERSION_1_0",
                "--whitelist-type", "VkCreateXcbSurfaceKHR",
                "--whitelist-type", "VkSurfaceKHR",
                "--whitelist-type", "VkPhysicalDevice",
                "--whitelist-type", "VkPhysicalDeviceProperties",
                "--whitelist-type", "VkQueueFlagBits",
                "--whitelist-type", "VkQueueFamilyProperties",
                "--whitelist-type", "VkDevice",
                "--whitelist-type", "VkDeviceQueueCreatInfo",
                "--whitelist-type", "VkDeviceCreateInfo",
                "--whitelist-type", "VkPhysicalDeviceFeatures",
                "--whitelist-var", "VK_SUCCESS",
                "--whitelist-var", "VK_KHR_SURFACE_EXTENSION_NAME",
                "--whitelist-var", "VK_KHR_XCB_SURFACE_EXTENSION_NAME",
                "--whitelist-var", "VK_KHR_SWAPCHAIN_EXTENSION_NAME",
                "--whitelist-var", "VK_MAX_PHYSICAL_DEVICE_NAME_SIZE",
                "--",
                "-DVK_USE_PLATFORM_XCB_KHR"
            ];
            let output = Command::new("bindgen")
                .args(args.iter())
                .output()
                .expect("Could not generate bindings");
            bw.write(&output.stdout).unwrap();
            bw.flush().unwrap();
        }
    }
}
