use rusty_beagle2d_glfw::ogl;
use linear_beaglebra::{matrix4x4, vector2::Vector2};
use std::mem;
use std::collections::{HashMap};

use crate::core::sprite;
use crate::core::shader;
use crate::{core::shader_program};

use rusty_beagle2d_freetype::freetype;

use std::ffi::{c_void, CString};
use std::ptr;

struct Character {
    TextureId: u32, // ID Handle of the glyph texture
    Size: Vector2, // Size of glyph
    Bearing: Vector2, // Offset from baseline to left/top of glyph
    Advance: u32, // Offset to advance to next glyph
    MaxHeight: u32
}

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
        // TODO: Read up on the OpenGL coords... clearly don't understand that, or how it works with orthographic cameras...
        let mut vertices: Vec<f32> = vec![
            // Positions       // Texture Coords
             1.0,  1.0, 0.0,   1.0, 1.0,         // Top Right
             1.0,  0.0, 0.0,   1.0, 0.0,         // Bottom Right
             0.0,  0.0, 0.0,   0.0, 0.0,         // Bottom Left
             0.0,  1.0, 0.0,   0.0, 1.0,         // Top Left
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
        ogl::buffer_data(ogl::BufferTarget::ElementArrayBuffer, &mut indices, ogl::Usage::DynamicDraw);

        let vertex_buffer = ogl::gl_gen_buffer();
        ogl::gl_bind_buffer(ogl::BufferTarget::ArrayBuffer, vertex_buffer);
        ogl::buffer_data(ogl::BufferTarget::ArrayBuffer, &mut vertices, ogl::Usage::DynamicDraw);

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
        let texture_bounding_box_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "bounding_box");

        // Set texture bounding box
        // TODO: Make vector 4 in linear-beaglebra
        let bounding_box_array = [sprite.texture_x, sprite.texture_y, sprite.texture_width, sprite.texture_height];
        ogl::uniform4fv(texture_bounding_box_location, 1, bounding_box_array.first().expect("Failed to read bounding box values"));

        // TODO yo read up on orthographic projections again!
        let mut homemade_orthographic_projection = matrix4x4::Matrix4x4::orthographic(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0);
        let homemade_camera_translate = Vector2::new(self.camera_position_x, self.camera_position_y);
        homemade_orthographic_projection = homemade_orthographic_projection.translate(homemade_camera_translate);

        ogl::uniform_matrix_4fv(projection_location, 1, false, homemade_orthographic_projection.first());

        // Transformation testing
        // TODO yo read up on matrix math again!
        let homemade_sprite_translation_matrix = Vector2::new(sprite.position_x, sprite.position_y);

        let mut homemade_sprite_matrix = matrix4x4::Matrix4x4::identity();
        homemade_sprite_matrix = homemade_sprite_matrix.translate(homemade_sprite_translation_matrix);
        homemade_sprite_matrix = homemade_sprite_matrix.rotate(0.0, 0.0, sprite.angle);
        homemade_sprite_matrix = homemade_sprite_matrix.scale(
            sprite.texture_width as f32,
            sprite.texture_height as f32,
            1.0
        );

        ogl::uniform_matrix_4fv(transform_location, 1, false, homemade_sprite_matrix.first());

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