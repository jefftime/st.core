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
                "--whitelist-type", "PFN_vkEnumerateInstanceExtensionProperties",
                "--whitelist-type", "VkResult",
                "--whitelist-type", "VkInstanceCreateInfo",
                "--whitelist-type", "VkApplicationInfo",
                "--whitelist-type", "VK_API_VERSION_1_0",
                "--whitelist-var", "VK_SUCCESS",
                "--whitelist-var", "VK_KHR_SURFACE_EXTENSION_NAME",
                "--whitelist-var", "VK_KHR_XCB_SURFACE_EXTENSION_NAME",
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
