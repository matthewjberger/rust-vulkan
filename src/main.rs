extern crate ash;
extern crate glfw;

use ash::{
    extensions::{
        ext::DebugUtils,
        khr::{Surface, Win32Surface},
    },
    version::*,
    *,
};
use core::ffi::c_void;
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

    let required_extensions = dbg!(glfw.get_required_instance_extensions()).unwrap_or(vec![]);
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

    let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
    let layer_names_raw: Vec<*const i8> = layer_names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();

    let extension_names = vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ];

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(&layer_names_raw)
        .enabled_extension_names(&extension_names);

    unsafe {
        let entry = Entry::new().unwrap();
        let instance = entry
            .create_instance(&create_info, None)
            .expect("Instance creation error");

        let _extension_properties = dbg!(entry.enumerate_instance_extension_properties());

        let debug_messenger = {
            let ext = DebugUtils::new(&entry, &instance);
            let info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
                .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
                .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
                .pfn_user_callback(Some(vulkan_debug_callback));

            let handle = ext.create_debug_utils_messenger(&info, None).unwrap();
            (ext, handle)
        };

        debug_messenger
            .0
            .destroy_debug_utils_messenger(debug_messenger.1, None);
        instance.destroy_instance(None);
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugUtilsMessageSeverityFlagsEXT,
    _: vk::DebugUtilsMessageTypeFlagsEXT,
    _: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    vk::FALSE
}
