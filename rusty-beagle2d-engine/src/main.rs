use rusty_beagle2d_glfw;
use rusty_beagle2d_glfw::ogl;

fn main() {
    let mut vertices = vec![
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0
    ];

    let init_result = rusty_beagle2d_glfw::glfw_init();

    if !init_result {
        panic!("Failed to initialize GLFW!");
    }

    let window_title = String::from("Rusyt Beagle! :D");

    // TODO: yeah.. this enum casting ain't great son
    rusty_beagle2d_glfw::glfw_window_hint(rusty_beagle2d_glfw::WindowHint::Resizable, rusty_beagle2d_glfw::GlfwBool::False as i32);
    rusty_beagle2d_glfw::glfw_window_hint(rusty_beagle2d_glfw::WindowHint::OpenGlProfile, rusty_beagle2d_glfw::OpenGlProfile::CoreProfile as i32);
    rusty_beagle2d_glfw::glfw_window_hint(rusty_beagle2d_glfw::WindowHint::ContextVersionMajor, 4);
    rusty_beagle2d_glfw::glfw_window_hint(rusty_beagle2d_glfw::WindowHint::ContextVersionMinor, 6);
    rusty_beagle2d_glfw::glfw_window_hint(rusty_beagle2d_glfw::WindowHint::OpenGlDebugContext, rusty_beagle2d_glfw::GlfwBool::True as i32);

    let main_window = rusty_beagle2d_glfw::glfw_create_window(800, 600, window_title, None, None).unwrap();

    rusty_beagle2d_glfw::glfw_make_context_current(&main_window);

    ogl::init();

    // Context Setup
    ogl::gl_enable(ogl::Capability::DebugOutput);
    ogl::gl_debug_message_callback(openg_debug_callback);

    // TODO: Could make a helper method that just returns a single int... would make it much easier.
    let mut vertex_buffer: [u32;1] = [0];
    ogl::gl_gen_buffers(1, &mut vertex_buffer);

    ogl::gl_bind_buffer(ogl::BufferTarget::ArrayBuffer, vertex_buffer[0]);

    ogl::BufferData(ogl::BufferTarget::ArrayBuffer, &mut vertices, ogl::Usage::StaticDraw);

    println!("{}", ogl::gl_get_string(ogl::Name::RENDERER));
    println!("{}", ogl::gl_get_string(ogl::Name::VERSION));

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

fn openg_debug_callback(source: u32, gltype: u32, id: u32, severity: u32, length: i32, message: String) {
    println!("We received an OpenGL Error: {}", message);
}