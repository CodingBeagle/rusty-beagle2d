use rusty_beagle2d_glfw;
use rusty_beagle2d_glfw::glfw;
use rusty_beagle2d_glfw::ogl;
use std::mem;
use std::path::Path;
use stb_image::image;

// "extern crate" indicates that you want to link against an external library, and brings the top-level
// crate name into scope.
extern crate nalgebra_glm as glm;

mod core;
use crate::core::shader;
use crate::core::shader_program;

fn main() {
    let mut vertices: Vec<f32> = vec![
        // Positions       // Texture Coords
         0.5,  0.5, 0.0,   1.0, 1.0,         // Top Right
         0.5, -0.5, 0.0,   1.0, 0.0,         // Bottom Right
        -0.5, -0.5, 0.0,   0.0, 0.0,         // Bottom Left
        -0.5,  0.5, 0.0,   0.0, 1.0,         // Top Left
    ];

    let mut indices: Vec<u32> = vec![
        0, 1, 3, // First Triangle
        1, 2, 3  // Second Triangle
    ];

    glfw::init().expect("Failed to initialize GLFW!");

    glfw::window_hint(glfw::WindowHint::Resizable as u32, glfw::GlfwBoolean::False as u32);
    glfw::window_hint(glfw::WindowHint::OpenGlProfile as u32, glfw::WindowHintValue::OpenGlCoreProfile as u32);
    glfw::window_hint(glfw::WindowHint::ContextVersionMajor as u32, 3);
    glfw::window_hint(glfw::WindowHint::ContextVersionMinor as u32, 3);
    glfw::window_hint(glfw::WindowHint::OpenGlDebugContext as u32, glfw::GlfwBoolean::True as u32);

    let main_window = 
        glfw::create_window(800, 600, String::from("Rusty Beagle! :D"), None, None).expect("Failed to create main window!");

    glfw::make_context_current(main_window);

    ogl::init();

    println!("{}", ogl::gl_get_string(ogl::Name::Renderer));
    println!("{}", ogl::gl_get_string(ogl::Name::Version));

    // Context Setup
    ogl::gl_enable(ogl::Capability::DebugOutput);
    ogl::gl_debug_message_callback(openg_debug_callback);

    // Bind VAO
    let vao = ogl::gen_vertex_array();
    ogl::bind_vertex_array(vao);

    // Bind indice buffer
    let ebo = ogl::gl_gen_buffer();

    ogl::gl_bind_buffer(ogl::BufferTarget::ElementArrayBuffer, ebo);
    ogl::buffer_data(ogl::BufferTarget::ElementArrayBuffer, &mut indices, ogl::Usage::StaticDraw);

    // TODO: Could make a helper method that just returns a single int... would make it much easier.
    let vertex_buffer = ogl::gl_gen_buffer();
    ogl::gl_bind_buffer(ogl::BufferTarget::ArrayBuffer, vertex_buffer);
    ogl::buffer_data(ogl::BufferTarget::ArrayBuffer, &mut vertices, ogl::Usage::StaticDraw);

    ogl::vertex_attrib_pointer(
        0, 
        3, 
        ogl::DataType::Float,
        false, 
        mem::size_of::<f32>() as i32 * 5,
        0);

    ogl::enable_vertex_attrib_array(0);

    // Shader compilation
    let vertex_shader = shader::Shader::new(shader::ShaderType::VertexShader, String::from("dat/shaders/vertex.shader"));
    let fragment_shader = shader::Shader::new(shader::ShaderType::FragmentShader, String::from("dat/shaders/fragment.shader"));
    
    let shader_program = shader_program::ShaderProgram::new(vertex_shader, fragment_shader);
    shader_program.activate();

    // Image Loading
    // TODO: Make wrapper for this
    unsafe {
        stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
    }    

    let imagePath = Path::new("dat/textures/beagle.jpg");
    let loadResult = image::load(imagePath);

    let imageData: image::Image::<u8> = match loadResult {
        image::LoadResult::Error(message) => panic!("Failed to load image: {}", message),
        image::LoadResult::ImageU8(imageu8) => {
            println!("Image width: {}", imageu8.width);
            println!("Image height: {}", imageu8.height);
            println!("Image depth: {}", imageu8.depth);
            imageu8
        },
        image::LoadResult::ImageF32(imagef32) => panic!("Loaded image f32! Not supported yet..."),
    };

    let texture_object = ogl::gen_texture();
    ogl::bind_texture(ogl::TextureTarget::Texture2d, texture_object);

    ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureWrapS, ogl::TextureParameter::Repeat);
    ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureWrapT, ogl::TextureParameter::Repeat);
    ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureMinFilter, ogl::TextureParameter::Linear);
    ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureMagFilter, ogl::TextureParameter::Linear);

    ogl::tex_image_2d::<u8>(ogl::TextureTarget::Texture2d, 0, ogl::TextureInternalFormat::Rgb, imageData.width as i32, imageData.height as i32, 0, ogl::TextureFormat::Rgb, ogl::ElementsDataType::UnsignedByte, imageData.data);
    ogl::generate_mipmap(ogl::TextureTarget::Texture2d);

    // TODO: Free image data after having uploaded it to OpenGL
    let offset : u32 = (3 * mem::size_of::<f32>()) as u32;
    ogl::vertex_attrib_pointer(1, 2, ogl::DataType::Float, false, (mem::size_of::<f32>() * 5) as i32, offset);
    ogl::enable_vertex_attrib_array(1);

    // Transformation testing
    // TODO yo read up on matrix math again!
    let translateVector = glm::vec3(400.0, 300.0, 0.0);
    let rotationAxis = glm::vec3(0.0, 0.0, 1.0);
    let scaleVec = glm::vec3(imageData.width as f32, imageData.height as f32, 1.0);

    let transform_location = ogl::get_uniform_location(shader_program.get_opengl_object_id(), "transform");
    let projection_location = ogl::get_uniform_location(shader_program.get_opengl_object_id(), "projection");

    let mut moving_degrees: f32 = 0.0;

    // TODO yo read up on orthographic projections again!
    let orthographic_projection = glm::ortho(0.0, 800.0, 600.0, 0.0, -1.0, 1.0);
    ogl::uniform_matrix_4fv(projection_location, 1, false, glm::value_ptr(&orthographic_projection).first().unwrap());

    while !glfw::window_should_close(main_window).expect("Failed to get window should close status.") {
        ogl::clear_color(
            100.0 / 255.0, 
            149.0 / 255.0, 
            237.0 / 255.0, 
            1.0);

        ogl::clear(ogl::ClearMask::ColorBufferBit);

        moving_degrees += 0.5;

        let mut transform_matrix = glm::Mat4::identity(); // 4x4 matrix with f32 elements.
        transform_matrix = glm::translate(&transform_matrix, &translateVector);
        transform_matrix = glm::rotate(&transform_matrix, degree_to_radians(moving_degrees), &rotationAxis);
        transform_matrix = glm::scale(&transform_matrix, &scaleVec);

        ogl::uniform_matrix_4fv(transform_location, 1, false, glm::value_ptr(&transform_matrix).first().unwrap());

        ogl::draw_elements(ogl::DrawMode::Triangles, 6, ogl::ElementsDataType::UnsignedInt);

        glfw::swap_buffers(main_window).expect("Failed to swap buffers for window!");
        glfw::poll_events();
    }

    glfw::terminate();
}

// TODO: Does nalgebra_glm seriously not have this? Gotta look more into this
fn degree_to_radians(degrees: f32) -> f32 {
    (std::f32::consts::PI / 180.0) * degrees
}
 
fn openg_debug_callback(source: u32, gltype: u32, id: u32, severity: u32, length: i32, message: String) {
    println!("We received an OpenGL Error: {}", message);
}