use rusty_beagle2d_glfw;
use rusty_beagle2d_glfw::glfw;
use rusty_beagle2d_glfw::ogl;

// "extern crate" indicates that you want to link against an external library, and brings the top-level
// crate name into scope.
extern crate nalgebra_glm as glm;

mod core;
use crate::core::texture;
use crate::core::renderer2d;
use crate::core::sprite;

static mut cam_x: f32 = 0.0;
static mut cam_y: f32 = 0.0;
static mut last_state: u32 = 0;

static mut button_states: u32 = 0;

fn main() {
    glfw::init().expect("Failed to initialize GLFW!");

    glfw::window_hint(glfw::WindowHint::Resizable as u32, glfw::GlfwBoolean::False as u32);
    glfw::window_hint(glfw::WindowHint::OpenGlProfile as u32, glfw::WindowHintValue::OpenGlCoreProfile as u32);
    glfw::window_hint(glfw::WindowHint::ContextVersionMajor as u32, 3);
    glfw::window_hint(glfw::WindowHint::ContextVersionMinor as u32, 3);
    glfw::window_hint(glfw::WindowHint::OpenGlDebugContext as u32, glfw::GlfwBoolean::True as u32);

    let main_window = 
        glfw::create_window(1024, 768, String::from("Rusty Beagle! :D"), None, None).expect("Failed to create main window!");

    glfw::make_context_current(main_window);

    glfw::set_key_callback(main_window, Some(glfw_key_callback));

    let mut renderer2d = renderer2d::Renderer2d::new();

    // Image Loading
    let grid_texture = texture::Texture::new(String::from("dat/textures/grid.png"));
    let grid_sprite = sprite::Sprite::new(&grid_texture);

    while !glfw::window_should_close(main_window).expect("Failed to get window should close status.") {
        ogl::clear_color(
            100.0 / 255.0, 
            149.0 / 255.0, 
            237.0 / 255.0, 
            1.0);

        ogl::clear(ogl::ClearMask::ColorBufferBit);

        unsafe {
            renderer2d.set_camera_position(cam_x, cam_y);
        }

        // KEY RIGHT
        if is_key_down(0) {
            unsafe {
                cam_x -= 5.0;
            }
        }

        // KEY LEFT
        if is_key_down(1) {
            unsafe {
                cam_x += 5.0;
            }
        }

        // KEY UP
        if is_key_down(2) {
            unsafe {
                cam_y += 5.0;
            }
        }

        // KEY DOWN
        if is_key_down(3) {
            unsafe {
                cam_y -= 5.0;
            }
        }

        renderer2d.draw_sprite(&grid_sprite);

        glfw::swap_buffers(main_window).expect("Failed to swap buffers for window!");

        // For continuous rendering, poll_events is the best way to process pending events.
        // This is a non-blocking event processing call.
        glfw::poll_events();
    }

    glfw::terminate();
}

fn is_key_down(button_id: i32) -> bool {
    unsafe {
        (button_states << button_id) & (1 << 31) != 0
    }
}

// TODO: Find a good pattern for dispencing key press events to the rest of a game engine
extern "C" fn glfw_key_callback(window: *mut glfw::GLFWwindow, key: i32, scancode: i32, action: i32, mods: i32) {
    unsafe {
        let mut button_id = -1;

        if key == glfw::GLFW_KEY_RIGHT as i32 {
            button_id = 0;
        }

        if key == glfw::GLFW_KEY_LEFT as i32 {
            button_id = 1;
        }

        if key == glfw::GLFW_KEY_UP as i32 {
            button_id = 2;
        }

        if key == glfw::GLFW_KEY_DOWN as i32 {
            button_id = 3;
        }

        if button_id >= 0 {
            if action == glfw::GLFW_PRESS as i32 {
                button_states = button_states | (1 << (31 - button_id));
            }
    
            if action == glfw::GLFW_RELEASE as i32 {
                button_states = button_states ^ (1 << (31 - button_id));
            }
        }
    }   
}
 
// 

fn openg_debug_callback(source: u32, gltype: u32, id: u32, severity: u32, length: i32, message: String) {
    println!("We received an OpenGL Error: {}", message);
}