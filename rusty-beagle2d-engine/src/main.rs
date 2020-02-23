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

    let renderer2d = renderer2d::Renderer2d::new();

    // Image Loading
    let beagle_texture = texture::Texture::new(String::from("dat/textures/beagle.jpg"));
    let mut beagle_sprite = sprite::Sprite::new(&beagle_texture);

    while !glfw::window_should_close(main_window).expect("Failed to get window should close status.") {
        ogl::clear_color(
            100.0 / 255.0, 
            149.0 / 255.0, 
            237.0 / 255.0, 
            1.0);

        ogl::clear(ogl::ClearMask::ColorBufferBit);

        renderer2d.draw_sprite(&beagle_sprite);

        glfw::swap_buffers(main_window).expect("Failed to swap buffers for window!");
        glfw::poll_events();
    }

    glfw::terminate();
}
 
fn openg_debug_callback(source: u32, gltype: u32, id: u32, severity: u32, length: i32, message: String) {
    println!("We received an OpenGL Error: {}", message);
}