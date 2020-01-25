use rusty_beagle2d_glfw;

fn main() {
    let init_result = rusty_beagle2d_glfw::glfw_init();

    if !init_result {
        panic!("Failed to initialize GLFW!");
    }

    let window_title = String::from("Rusyt Beagle! :D");

    let main_window = rusty_beagle2d_glfw::glfw_create_window(800, 600, window_title, None, None).unwrap();

    while !rusty_beagle2d_glfw::glfw_window_should_close(&main_window) {
        rusty_beagle2d_glfw::glfw_poll_events();
    }
}