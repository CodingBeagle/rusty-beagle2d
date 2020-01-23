use rusty_beagle2d_glfw;

fn main() {
    let mut init_result = rusty_beagle2d_glfw::glfw_init();

    if !init_result {
        panic!("Failed to initialize GLFW!");
    }

    let windowTitle = String::from("Rusyt Beagle! :!");

    let mainWindow = rusty_beagle2d_glfw::glfw_create_window(800, 600, windowTitle, None, None).unwrap();

    while !rusty_beagle2d_glfw::glfw_window_should_close(mainWindow) {
        rusty_beagle2d_glfw::glfw_poll_events();
    }
}