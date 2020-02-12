use rusty_beagle2d_glfw;
use rusty_beagle2d_glfw::ogl;
use std::fs;
use std::mem;
use std::path::Path;
use stb_image::image;

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

    let init_result = rusty_beagle2d_glfw::glfw_init();

    if !init_result {
        panic!("Failed to initialize GLFW!");
    }

    let window_title = String::from("Rusyt Beagle! :D");

    // TODO: yeah.. this enum casting ain't great son
    rusty_beagle2d_glfw::glfw_window_hint(rusty_beagle2d_glfw::WindowHint::Resizable, rusty_beagle2d_glfw::GlfwBool::False as i32);
    rusty_beagle2d_glfw::glfw_window_hint(rusty_beagle2d_glfw::WindowHint::OpenGlProfile, rusty_beagle2d_glfw::OpenGlProfile::CoreProfile as i32);
    rusty_beagle2d_glfw::glfw_window_hint(rusty_beagle2d_glfw::WindowHint::ContextVersionMajor, 3);
    rusty_beagle2d_glfw::glfw_window_hint(rusty_beagle2d_glfw::WindowHint::ContextVersionMinor, 3);
    rusty_beagle2d_glfw::glfw_window_hint(rusty_beagle2d_glfw::WindowHint::OpenGlDebugContext, rusty_beagle2d_glfw::GlfwBool::True as i32);

    let main_window = rusty_beagle2d_glfw::glfw_create_window(800, 600, window_title, None, None).unwrap();

    rusty_beagle2d_glfw::glfw_make_context_current(&main_window);

    ogl::init();

    println!("{}", ogl::gl_get_string(ogl::Name::RENDERER));
    println!("{}", ogl::gl_get_string(ogl::Name::VERSION));

    // Context Setup
    ogl::gl_enable(ogl::Capability::DebugOutput);
    ogl::gl_debug_message_callback(openg_debug_callback);

    // Bind VAO
    let vao = ogl::gen_vertex_array();
    ogl::bind_vertex_array(vao);

    // Bind indice buffer
    let ebo = ogl::gl_gen_buffer();

    ogl::gl_bind_buffer(ogl::BufferTarget::ElementArrayBuffer, ebo);
    ogl::BufferData(ogl::BufferTarget::ElementArrayBuffer, &mut indices, ogl::Usage::StaticDraw);

    // TODO: Could make a helper method that just returns a single int... would make it much easier.
    let mut vertex_buffer: [u32;1] = [0];
    ogl::gl_gen_buffers(1, &mut vertex_buffer);

    ogl::gl_bind_buffer(ogl::BufferTarget::ArrayBuffer, vertex_buffer[0]);
    ogl::BufferData(ogl::BufferTarget::ArrayBuffer, &mut vertices, ogl::Usage::StaticDraw);

    ogl::vertex_attrib_pointer_no_offset(
        0, 
        3, 
        ogl::DataType::Float,
        false, 
        mem::size_of::<f32>() as i32 * 5);

    ogl::enable_vertex_attrib_array(0);

    // Shader compilation
    let vertexShaderCode = fs::read_to_string("dat\\shaders\\vertex.shader").unwrap();

    let vertexShader = ogl::create_shader(ogl::ShaderType::Vertex);
    ogl::shader_source(vertexShader, 1, &vec![&vertexShaderCode]);
    ogl::compile_shader(vertexShader);

    let vertex_shader_compilation_result = ogl::get_shader(vertexShader, ogl::Parameter::CompileStatus);

    if vertex_shader_compilation_result != 1 {
        let compilationReport = ogl::get_shader_info_log(vertexShader);

        println!("{}", compilationReport);
        panic!("Failed to compile vertex shader!");
    }

    let fragment_shader_code = fs::read_to_string("dat\\shaders\\fragment.shader").unwrap();

    let fragment_shader = ogl::create_shader(ogl::ShaderType::Fragment);
    ogl::shader_source(fragment_shader, 1, &vec![&fragment_shader_code]);
    ogl::compile_shader(fragment_shader);

    if ogl::get_shader(fragment_shader, ogl::Parameter::CompileStatus) != 1 {
        panic!("Failed to compile fragment shader! {}", ogl::get_shader_info_log(fragment_shader));
    }

    let shader_program = ogl::create_program();

    ogl::attach_shader(shader_program, vertexShader);   
    ogl::attach_shader(shader_program, fragment_shader);

    ogl::link_program(shader_program);

    let link_status = ogl::get_programiv(shader_program, ogl::ProgramParameter::LinkStatus);

    if link_status != 1 {
        panic!("Failed to link shader program! {}", ogl::get_program_info_log(shader_program));
    }

    // Shader cleanup
    ogl::delete_shader(vertexShader);
    ogl::delete_shader(fragment_shader);

    ogl::use_program(shader_program);

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

    while !rusty_beagle2d_glfw::glfw_window_should_close(&main_window) {
        ogl::clear_color(
            100.0 / 255.0, 
            149.0 / 255.0, 
            237.0 / 255.0, 
            1.0);

        ogl::clear(ogl::ClearMask::ColorBufferBit);

        ogl::draw_elements(ogl::DrawMode::Triangles, 6, ogl::ElementsDataType::UnsignedInt);

        rusty_beagle2d_glfw::glfw_swap_buffers(&main_window);
        rusty_beagle2d_glfw::glfw_poll_events();
    }

    rusty_beagle2d_glfw::glfw_terminate();
}

fn openg_debug_callback(source: u32, gltype: u32, id: u32, severity: u32, length: i32, message: String) {
    println!("We received an OpenGL Error: {}", message);
}