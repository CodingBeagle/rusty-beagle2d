use rusty_beagle2d_glfw;
use rusty_beagle2d_glfw::glfw;
use rusty_beagle2d_glfw::ogl;

use linear_beaglebra::{vector2::Vector2, matrix4x4::Matrix4x4};

use std::time::{Instant, Duration};

mod core;
use crate::core::texture;
use crate::core::renderer2d;
use crate::core::sprite;

static mut cam_x: f32 = 0.0;
static mut cam_y: f32 = 0.0;

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

    // Game loop variables
    let mut t = Duration::from_millis(0);
    let dt = Duration::from_millis(1);

    let mut current_time = Instant::now();
    let mut accumulator = Duration::new(0, 0);

    // A game look typically consists of multiple different subsystems that needs "servicing" at different rates.
    // For example, rendering the scene and updating the game's physics state need not be done in synchronization, and most
    // Often is actually not.
    // Right now, my physics are updated at a rate of 1 millisecond
    // Currently, my game loop consists of the flow:
    // 1. Read I/O
    // 2. Update Game Physics
    // 3. Render scene
    while !glfw::window_should_close(main_window).expect("Failed to get window should close status.") {
        let new_time = Instant::now();
        let mut frame_time = new_time - current_time;

        // TODO: Figure out why this is done
        if frame_time > Duration::from_millis(250) {
            frame_time = Duration::from_millis(250);
        }

        current_time = new_time;
        accumulator += frame_time;

        // I/O Subsystem
        // For continuous rendering, poll_events is the best way to process pending events.
        // This is a non-blocking event processing call.
        glfw::poll_events();

        // Physics Subsystem
        while accumulator >= dt {
            unsafe {
                renderer2d.set_camera_position(cam_x, cam_y);
            }

            integrate(dt.as_secs_f32());
            t += dt;
            accumulator -= dt;
        }

        // Render Loop / Rendering Subsystem
        ogl::clear_color(
            100.0 / 255.0, 
            149.0 / 255.0,
            237.0 / 255.0, 
            1.0);

        ogl::clear(ogl::ClearMask::ColorBufferBit);

        renderer2d.draw_text("Hello, World! :D", 
            Vector2::new(0.0, 0.0), 
            1.0);

        renderer2d.draw_text("And another line!", 
            Vector2::new(0.0, 61.0), 
            1.0);

        glfw::swap_buffers(main_window).expect("Failed to swap buffers for window!");
    }

    glfw::terminate();
}

fn integrate(dt : f32) {
    // KEY RIGHT
    if is_key_down(0) {
        unsafe {
            cam_x -= 500.0 * dt;
        }
    }

    // KEY LEFT
    if is_key_down(1) {
        unsafe {
            cam_x += 500.0 * dt;
        }
    }

    // KEY UP
    if is_key_down(2) {
        unsafe {
            cam_y += 500.0 * dt;
        }
    }

    // KEY DOWN
    if is_key_down(3) {
        unsafe {
            cam_y -= 500.0 * dt;
        }
    }
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