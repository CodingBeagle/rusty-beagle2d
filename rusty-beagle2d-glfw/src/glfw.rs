/*
    I'm attempting to make the GLFW wrapper with the following design considerations in mind:
    -- Fail Fast
    ---- Any errors should be caught as early as possible and reported to the client
    -- Fail With Description
    ---- Any error should be given as much description as possible
    -- Minimal Unsafe Potential
    ---- The wrapper interface should expose the underlying GLFW interface in a fashion that makes it
    ---- As difficult to use incorrectly as possible
*/
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings.rs"));

use std::ffi::CString;
use core::ffi::c_void;

// LEARN - REPR(u32)
// By using repr(u32) I specify the size to make the fieldless enum.
// A fieldless enum is simply an enum that has no data in its variants
// I do this because all the GLFW_* constants are of type u32.
#[repr(u32)]
pub enum WindowHint {
    Resizable = GLFW_RESIZABLE,
    OpenGlProfile = GLFW_OPENGL_PROFILE,
    ContextVersionMajor = GLFW_CONTEXT_VERSION_MAJOR,
    ContextVersionMinor = GLFW_CONTEXT_VERSION_MINOR,
    OpenGlDebugContext = GLFW_OPENGL_DEBUG_CONTEXT
}

#[repr(u32)]
pub enum GlfwBoolean {
    True = GLFW_TRUE,
    False = GLFW_FALSE
}

#[repr(u32)]
pub enum WindowHintValue {
    OpenGlCoreProfile = GLFW_OPENGL_CORE_PROFILE,
}

// LEARN - Result<(), ...>
// In Rust, "()" is the "void" type, also called the Unit type.
// For Results, this is the idiomatic way to indicate a function in which
// You want the work of the function to be performed, but that it will not have any return value
// On success.
pub fn init() -> Result<(), String> {
    unsafe {
        if glfwInit() == 1 {
            Ok(())
        } else {
            Err(String::from("Failed to initialize GLFW."))
        }
    }
}

pub fn swap_interval(interval: i32) {
    unsafe {
        glfwSwapInterval(interval);
    }
}

pub fn window_hint(windowHint: u32, windowHintValue: u32) {
    unsafe {
        glfwWindowHint(windowHint as i32, windowHintValue as i32);
    }
}

pub fn set_key_callback(window: *mut GLFWwindow, callback: GLFWkeyfun) {
    unsafe {
        glfwSetKeyCallback(window, callback);

        // TODO: Check to see if GLFW had any error here.
        // GLFW can actually return Null / None from glfwSetKeyCallback if GLFW has not yet
        // Been initialized. But it will also return Null / None if everything went OK but
        // Not callback had been previously set!
    }
}

pub fn create_window(width: i32, 
                    height: i32, 
                    title: String, 
                    monitor: Option<*mut GLFWmonitor>, 
                    share: Option<*mut GLFWwindow>) -> Result<*mut GLFWwindow, String>
{
    unsafe {
        // LEARN - CString
        // The CString type represents an owned, C-compatible, nul-terminated string with no
        // nul bytes in the middle.
        // It's used to generate C-compatible strings from interface with Rust to C libraries.
        let title_c_string = match CString::new(title) {
            Ok(cstring) => cstring,
            Err(e) => return Err(String::from("Failed to create CString from title parameter."))
        };

        let created_window = glfwCreateWindow(width, height, title_c_string.as_ptr(),
            match monitor {
                Some(monitor) => monitor,
                None => std::ptr::null_mut::<GLFWmonitor>(),
            },
            match share {
                Some(share) => share,
                None => std::ptr::null_mut::<GLFWwindow>()
            });

        if created_window.is_null() {
            return Err(String::from("Failed to create GLFW window."))
        }

        Ok(created_window)
    }
}

pub fn make_context_current(window: *mut GLFWwindow) {
    unsafe {
        // Notice, a null pointer is a valid value in order to detach the current context.
        glfwMakeContextCurrent(window);
    }
}

pub fn window_should_close(window: *mut GLFWwindow) -> Result<bool, String> {
    // Cannot find anything in documentation about what happens if null is provided.
    // However, can't see why you'd ever do that or what a useful result should look like
    // So I'm setting a constraint in this wrapper.
    if window.is_null() {
        return Err(String::from("The window cannot be null."));
    }

    unsafe {
        if glfwWindowShouldClose(window) == 1 {Ok(true)} else {Ok(false)}
    }
}

pub fn swap_buffers(window: *mut GLFWwindow) -> Result<(), String> {
    // Not sure what documentation says about null window being provided
    // But can't see how that would make sense, so setting restriction
    if window.is_null() {
        return Err(String::from("Window cannot be null."));
    }

    unsafe {
        glfwSwapBuffers(window);
        Ok(())
    }
}

pub fn poll_events() {
    unsafe {
        glfwPollEvents();
    }
}

pub fn terminate() {
    unsafe {
        glfwTerminate();
    }
}

pub fn get_proc_address(procname: &'static str) -> *const c_void {
    let procname_c_string =  CString::new(procname).expect("Failed to create title string!");

    unsafe {
        match glfwGetProcAddress(procname_c_string.as_ptr()) {
            Some(function_pointer) => function_pointer as *const c_void,
            None => std::ptr::null()
        }
    }
}