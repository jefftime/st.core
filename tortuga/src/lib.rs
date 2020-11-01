#![no_std]

pub mod window;

#[cfg_attr(feature = "vulkan", path = "render/linux.rs")]
pub mod render;
