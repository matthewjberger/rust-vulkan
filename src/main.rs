extern crate ash;
extern crate glfw;

use ash::extensions::khr::{Surface, Win32Surface};
use ash::*;
// use ash::util::*;
use std::ffi::CString;

use glfw::Context;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::Visible(true));

    let (mut window, _) = glfw
        .create_window(640, 480, "Vulkan Testing", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();

    assert!(glfw.vulkan_supported());
    let required_extensions = glfw.get_required_instance_extensions().unwrap_or(vec![]);
    // VK_KHR_surface will always be available if the previous operations were successful
    assert!(required_extensions.contains(&"VK_KHR_surface".to_string()));
    println!("Vulkan required extensions: {:?}", required_extensions);

    let app_name = CString::new("Hello Triangle").unwrap();
    let engine_name = CString::new("No engine").unwrap();
    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .engine_name(&engine_name)
        .application_version(vk_make_version!(1, 0, 0))
        .engine_version(0)
        .api_version(vk_make_version!(1, 0, 0));

    let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
    let layer_names_raw: Vec<*const i8> = layer_names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(&layer_names_raw)
        .enabled_extension_names(&vec![
            Surface::name().as_ptr(),
            Win32Surface::name().as_ptr(),
        ]);
}