use rusty_beagle2d_glfw;
use rusty_beagle2d_glfw::ogl;

fn main() {
    let vertices = [
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0
    ];

    let init_result = rusty_beagle2d_glfw::glfw_init();

    if !init_result {
        panic!("Failed to initialize GLFW!");
    }

    let window_title = String::from("Rusyt Beagle! :D");

    let main_window = rusty_beagle2d_glfw::glfw_create_window(800, 600, window_title, None, None).unwrap();

    rusty_beagle2d_glfw::glfw_make_context_current(&main_window);

    ogl::init();

    // TODO: Could make a helper method that just returns a single int... would make it much easier.
    let mut vertex_buffer: [u32;1] = [0];
    ogl::gl_gen_buffers(1, &mut vertex_buffer);

    ogl::gl_bind_buffer(ogl::BufferTarget::ArrayBuffer, vertex_buffer[0]);

    while !rusty_beagle2d_glfw::glfw_window_should_close(&main_window) {
        ogl::clear_color(
            100.0 / 255.0, 
            149.0 / 255.0, 
            237.0 / 255.0, 
            1.0);

        ogl::clear(ogl::ClearMask::ColorBufferBit);

        rusty_beagle2d_glfw::glfw_swap_buffers(&main_window);

        rusty_beagle2d_glfw::glfw_poll_events();
    }

    rusty_beagle2d_glfw::glfw_terminate();
}