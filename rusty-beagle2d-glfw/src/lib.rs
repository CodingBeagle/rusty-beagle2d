extern crate libc;
use libc::c_int;
use libc::c_char;
use libc::c_void;
use std::ffi::CString;
use std::ptr;

// TODO: Read up on repr(C) meaning
#[repr(C)] pub struct GLFWmonitor { _private: [u8; 0] }
#[repr(C)] pub struct GLFWwindow { _private: [u8; 0] }

// By default, everything in Rust is private.
// Except in two cases:
// - Associated items in a "pub" Trait are public by default.
// - Enum variants in a "pub" enum are also public by default.
// Visibility in Rust works on a per-module basis.
// Defining Window as "pub" means that it can be accessed from an external module.
// If an item is private (like window_ptr field in Window), it can only be accessed by the current module.
// This can be used to create module hierarchies exposing public APIs while hiding internal implementation details.
// TODO: Read up on Modules! There's important things to be understood in regards to visibility in Rust
pub struct Window {
    window_ptr: *mut GLFWwindow
}

pub struct Monitor {
    monitor_ptr: *mut GLFWmonitor
}

#[link(name = "glfw3-x64-debug", kind="dylib")]
// The "extern" block is used to facilitate creation and use of an FFI (Foreign Function Interface)
// An FFI is a way for a programming language to define functions and enable a different (foreign)
// Programming language to call those functions.
extern {
    // TODO: I suppose a more idiomatic rust way would be to return a Result?
    fn glfwInit() -> c_int;

    // Rust has two raw pointer types we can use:
    // Mutable (*mut T) and immutable ones (*const T)
    // Actually, you CAN create raw pointers in safe code. You just can't dereference raw pointers outside safe blocks.
    fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor:  *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;

    fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;

    fn glfwMakeContextCurrent(window: *mut GLFWwindow);

    fn glfwSwapBuffers(window: *mut GLFWwindow);

    // TODO: Is this really correct way to return function pointer?
    fn glfwGetProcAddress(procname: *const c_char) -> *const c_void;

    fn glfwTerminate();

    fn glfwPollEvents();
}

pub fn glfw_get_proc_address(procname: &'static str) -> *const c_void {
    // TODO: Is this c string lifetime right?
    let title_c_string =  CString::new(procname).expect("Failed to create title string!");

    unsafe {
        glfwGetProcAddress(title_c_string.as_ptr()) as *const c_void
    }
}

pub fn glfw_poll_events() {
    unsafe {
        glfwPollEvents();
    }
}

pub fn glfw_init() -> bool {
    unsafe {
        glfwInit() == 1
    }
}

pub fn glfw_terminate() {
    unsafe {
        glfwTerminate();
    }
}

pub fn glfw_swap_buffers(window: &Window) {
    unsafe {
        glfwSwapBuffers(window.window_ptr);
    }
}

pub fn glfw_window_should_close(window: &Window) -> bool {
    unsafe {
        glfwWindowShouldClose(window.window_ptr) == 1
    }
}

pub fn glfw_make_context_current(window: &Window) {
    unsafe {
        glfwMakeContextCurrent(window.window_ptr);
    }
}

// TODO: Read up on Options: https://doc.rust-lang.org/std/option/
// TODO: Remember to read up on CString some more here: https://doc.rust-lang.org/std/ffi/struct.CString.html
// TODO: Maybe return some type of result object here? With either a window, or an error in case something went wrong!
pub fn glfw_create_window(width: i32, height: i32, title: String, monitor: Option<&Monitor>, share: Option<&Window>) -> Option<Window> {
    let title_c_string =  CString::new(title).expect("Failed to create title string!");

    let result = unsafe {
        glfwCreateWindow(
            width, 
            height, 
            title_c_string.as_ptr(), 
            match monitor {
                Some(x) => x.monitor_ptr as *mut GLFWmonitor,
                None => ptr::null_mut() as *mut GLFWmonitor
            }, 
            match share {
                Some(x) => x.window_ptr as *mut GLFWwindow,
                None => ptr::null_mut() as *mut GLFWwindow
            })
        };

    if result.is_null() {
        return None;
    }
    
    let created_window = Window {
        window_ptr: result
    };

    Some(created_window)
}

// TODO: Figure out how to split this module out into separate file in this crate
pub mod ogl {
    pub enum ClearMask {
        ColorBufferBit
    }

    pub fn init() {
        // TODO: Read up on this funky syntax
        gl::load_with(|s| crate::glfw_get_proc_address(s));
    }

    pub fn clear(mask: ClearMask) {
        // TODO: Maybe I don't need match for this... (could just assign value in enum definition)
        let mask = match mask {
            ClearMask::ColorBufferBit => gl::COLOR_BUFFER_BIT
        };

        unsafe {
            gl::Clear(mask);
        }
    }

    pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }
}