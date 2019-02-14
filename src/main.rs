extern crate ash;
extern crate glfw;

use ash::{
    extensions::khr::{Surface, Win32Surface},
    version::*,
    *,
};
use std::ffi::CString;

use glfw::Context;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::Visible(true));

    let (mut window, _) = glfw
        .create_window(640, 480, "Vulkan Testing", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();

    assert!(glfw.vulkan_supported());

    let required_extensions = glfw.get_required_instance_extensions().unwrap_or(vec![]);
    println!("Vulkan required extensions: {:?}", required_extensions);

    // VK_KHR_surface will always be available if the previous operations were successful
    assert!(required_extensions.contains(&"VK_KHR_surface".to_string()));

    let app_name = CString::new("Hello Triangle").unwrap();
    let engine_name = CString::new("No engine").unwrap();
    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .engine_name(&engine_name)
        .application_version(vk_make_version!(1, 0, 0))
        .engine_version(0)
        .api_version(vk_make_version!(1, 0, 0));

    let extension_names = vec![Surface::name().as_ptr(), Win32Surface::name().as_ptr()];

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names);

    unsafe {
        let entry = Entry::new().unwrap();
        let instance = entry
            .create_instance(&create_info, None)
            .expect("Instance creation error");

        let extension_properties = entry.enumerate_instance_extension_properties();
        println!("{:?}", extension_properties);

        instance.destroy_instance(None);
    }
}
