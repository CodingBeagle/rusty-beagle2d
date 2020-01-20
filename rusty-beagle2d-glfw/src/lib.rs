extern crate libc;
use libc::c_int;

#[link(name = "glfw3", kind="dylib")]
extern {
    fn glfwInit() -> c_int;
}

pub fn glfw_init() -> bool {
    unsafe {
        glfwInit() == 1
    }
}