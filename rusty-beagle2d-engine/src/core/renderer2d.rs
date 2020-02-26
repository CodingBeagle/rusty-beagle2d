use rusty_beagle2d_glfw::ogl;
use std::mem;

use crate::core::sprite;
use crate::core::shader;
use crate::core::shader_program;

pub struct Renderer2d {
    shader_program: shader_program::ShaderProgram,
    camera_position_x: f32,
    camera_position_y: f32
}

impl Renderer2d {
    pub fn set_camera_position(&mut self, position_x: f32, position_y: f32) {
        self.camera_position_x = position_x;
        self.camera_position_y = position_y;
    }

    pub fn new() -> Renderer2d {
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

        // Load OpenGl functions
        ogl::init();

        // Display OpenGl context information
        println!("{}", ogl::gl_get_string(ogl::Name::Renderer));
        println!("{}", ogl::gl_get_string(ogl::Name::Version));

        // Context Setup
        ogl::gl_enable(ogl::Capability::DebugOutput);
        ogl::gl_debug_message_callback(openg_debug_callback);

        // Enable opacity
        ogl::enable(ogl::Cap::Blend);
        ogl::blend_func(ogl::BlendFactor::SrcAlpha, ogl::BlendFactor::OneMinusSrcAlpha);

        // Bind VAO
        let vao = ogl::gen_vertex_array();
        ogl::bind_vertex_array(vao);

        // Bind indice buffer
        let ebo = ogl::gl_gen_buffer();

        ogl::gl_bind_buffer(ogl::BufferTarget::ElementArrayBuffer, ebo);
        ogl::buffer_data(ogl::BufferTarget::ElementArrayBuffer, &mut indices, ogl::Usage::StaticDraw);

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

        let offset : u32 = (3 * mem::size_of::<f32>()) as u32;

        ogl::vertex_attrib_pointer(1,
             2, 
             ogl::DataType::Float,
             false, 
             (mem::size_of::<f32>() * 5) as i32, 
             offset);
        
        ogl::enable_vertex_attrib_array(1);

        // Shader compilation
        let vertex_shader = shader::Shader::new(shader::ShaderType::VertexShader, String::from("dat/shaders/vertex.shader"));
        let fragment_shader = shader::Shader::new(shader::ShaderType::FragmentShader, String::from("dat/shaders/fragment.shader"));
        
        let shader_program = shader_program::ShaderProgram::new(vertex_shader, fragment_shader);
        shader_program.activate();

        Renderer2d {
            shader_program: shader_program,
            camera_position_x: 0.0,
            camera_position_y: 0.0 
        }
    }

    pub fn draw_sprite(&self, sprite: &sprite::Sprite) {
        // Activate the Sprite's texture for the OpenGl context
        sprite.texture.activate();

        let transform_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "transform");
        let projection_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "projection");

        // TODO yo read up on orthographic projections again!
        let mut orthographic_projection = glm::ortho(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0);
        let cam_translate = glm::vec3(self.camera_position_x, self.camera_position_y, 1.0);
        orthographic_projection = glm::translate(&orthographic_projection, &cam_translate);

        ogl::uniform_matrix_4fv(projection_location, 1, false, glm::value_ptr(&orthographic_projection).first().unwrap());

        // Transformation testing
        // TODO yo read up on matrix math again!
        let translate_vector = glm::vec3(sprite.position_x, sprite.position_y, 0.0);

        let scale_vec = glm::vec3(
            sprite.texture.get_width() as f32, 
            sprite.texture.get_height() as f32,
            1.0);

        let mut transform_matrix = glm::Mat4::identity(); // 4x4 matrix with f32 elements.
        transform_matrix = glm::translate(&transform_matrix, &translate_vector);
        transform_matrix = glm::rotate(&transform_matrix, Renderer2d::degree_to_radians(sprite.angle), &glm::vec3(0.0, 0.0, 1.0));
        transform_matrix = glm::scale(&transform_matrix, &scale_vec);

        ogl::uniform_matrix_4fv(transform_location, 1, false, glm::value_ptr(&transform_matrix).first().unwrap());

        ogl::draw_elements(ogl::DrawMode::Triangles, 6, ogl::ElementsDataType::UnsignedInt);
    }

    // TODO: Does nalgebra_glm seriously not have this? Gotta look more into this
    fn degree_to_radians(degrees: f32) -> f32 {
        (std::f32::consts::PI / 180.0) * degrees
    }
}

impl Drop for Renderer2d {
    fn drop(&mut self) {
        // TODO: Need custom drop here?
    }
}

fn openg_debug_callback(source: u32, gltype: u32, id: u32, severity: u32, length: i32, message: String) {
    println!("We received an OpenGL Error: {}", message);
}