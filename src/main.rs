extern crate ash;
extern crate glfw;
extern crate winapi;

use ash::{
    extensions::{
        ext::DebugUtils,
        khr::{Surface, Win32Surface},
    },
    version::*,
    vk::DeviceQueueCreateInfoBuilder,
    *,
};
use core::ffi::c_void;
use std::{ffi::CString, ptr};

use glfw::Context;

use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;

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

        let window_create_info = vk::Win32SurfaceCreateInfoKHR {
            s_type: vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: Default::default(),
            hinstance: GetModuleHandleW(ptr::null()) as *const c_void,
            hwnd: window.get_win32_window() as HWND as *const c_void,
        };
        let win32_surface_loader = Win32Surface::new(&entry, &instance);
        let surface = win32_surface_loader
            .create_win32_surface(&window_create_info, None)
            .unwrap();
        let surface_loader = Surface::new(&entry, &instance);

        let devices = instance
            .enumerate_physical_devices()
            .expect("Physical device error");

        if devices.len() == 0 {
            println!("No physical devices are available.");
            instance.destroy_instance(None);
            return;
        }

        let (physical_device, queue_family_index) = devices
            .iter()
            .map(|physical_device| {
                instance
                    .get_physical_device_queue_family_properties(*physical_device)
                    .iter()
                    .enumerate()
                    .filter_map(|(index, ref info)| {
                        let supports_graphic_and_surface =
                            info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                                && surface_loader.get_physical_device_surface_support(
                                    *physical_device,
                                    index as u32,
                                    surface,
                                );
                        match supports_graphic_and_surface {
                            true => Some((*physical_device, index as u32)),
                            _ => None,
                        }
                    })
                    .nth(0)
            })
            .filter_map(|v| v)
            .nth(0)
            .expect("Couldn't find a suitable device.");

        let priorities = [1.0];
        let queue_create_info = [vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .queue_priorities(&priorities)
            .build()];

        surface_loader.destroy_surface(surface, None);
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
