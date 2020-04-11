use rusty_beagle2d_glfw::ogl;
use linear_beaglebra::{matrix4x4, vector2::Vector2};
use std::mem;
use std::collections::{HashMap};

use crate::core::sprite;
use crate::core::shader;
use crate::{core::shader_program};

use crate::core::texture;

use std::boxed;

use std::fs::{File};
use std::path::{Path};

use std::ffi::{c_void, CString};
use std::ptr;

struct Character {
    TextureId: u32, // ID Handle of the glyph texture,
    TexturePosition: Vector2,
    Size: Vector2, // Size of glyph
    Bearing: Vector2, // Offset from baseline to left/top of glyph
    Advance: u32, // Offset to advance to next glyph
    MaxHeight: u32
}

pub struct Renderer2d {
    shader_program: shader_program::ShaderProgram,
    camera_position_x: f32,
    camera_position_y: f32,
    text_sprite_atlas: sprite::Sprite,
    character_info: HashMap<u8, Character>
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

        // Font Texture Setup
        let text_atlas = std::boxed::Box::new(texture::Texture::new(String::from("test-dat/fonts/bitmap-fonts/verdana-signed.png")));

        // LEARN: Read up on Rust iterators
        let mut characters: HashMap<u8, Character> = HashMap::new();

        let character_atlas_info = std::fs::read_to_string(Path::new("test-dat/fonts/bitmap-fonts/verdana-signed.fnt"))
            .expect("Failed to load character atlas info");

        let line_iterator: Vec<&str> = character_atlas_info.lines().collect();

        let max_height_line: Vec<&str> = line_iterator[1]
            .split_whitespace()
            .skip(1)
            .collect();

        let max_height = Renderer2d::parse_value(max_height_line[0]);

        // TODO: When using Hiero to export .fnt, padding is encoded oddly...
        // Left padding is subtracted from the original xoffset value, so you have to add padding to xoffset to value you need for text rendering
        // Left + Right Padding is added to xadvance, so these have to be subtracted from xadvance to get the value you need for text rendering
        // Up padding is substracted from yoffset value, so you have to add it back to get original yoffset value. 
        let padding_line: Vec<&str> = line_iterator[0].split_whitespace().collect();
        let padding_text: Vec<&str> = padding_line[11].rsplit('=').collect();
        let padding_values: Vec<&str> = padding_text[0].split(',').collect();
        
        let padding_up = i32::from_str_radix( padding_values[0], 10).expect("Failed to convert padding to int value.");
        let padding_right = i32::from_str_radix( padding_values[1], 10).expect("Failed to convert padding to int value.");
        let padding_down = i32::from_str_radix( padding_values[2], 10).expect("Failed to convert padding to int value.");
        let padding_left = i32::from_str_radix( padding_values[3], 10).expect("Failed to convert padding to int value.");

        for character_line in &line_iterator[4..line_iterator.len()-1] {
            let line_words: Vec<&str> = character_line
                .split_whitespace()
                .skip(1)
                .collect();

            let character_id = Renderer2d::parse_value(line_words[0]);
            let character_texture_position_x = Renderer2d::parse_value(line_words[1]);
            let character_texture_position_y = Renderer2d::parse_value(line_words[2]);
            let character_width = Renderer2d::parse_value(line_words[3]);
            let character_height = Renderer2d::parse_value(line_words[4]);
            let character_x_offset = Renderer2d::parse_value(line_words[5]) + padding_left;
            let character_y_offset = Renderer2d::parse_value(line_words[6]);
            let character_x_advance = Renderer2d::parse_value(line_words[7]) - (padding_left + padding_right);

            characters.insert(character_id as u8, Character {
                TextureId: text_atlas.get_opengl_texture_id(),
                TexturePosition: Vector2::new(character_texture_position_x as f32, character_texture_position_y as f32),
                Size: Vector2::new(character_width as f32, character_height as f32),
                Bearing: Vector2::new(character_x_offset as f32, character_y_offset as f32),
                Advance: character_x_advance as u32,
                MaxHeight: max_height as u32
            });
        }

        let text_sprite = sprite::Sprite::new(text_atlas);

        Renderer2d {
            shader_program: shader_program,
            camera_position_x: 0.0,
            camera_position_y: 0.0,
            text_sprite_atlas: text_sprite,
            character_info: characters
        }
    }

    fn parse_value(value_pair: &str) -> i32 {
        let split_values: Vec<&str> = value_pair.rsplit('=').collect();
        i32::from_str_radix(split_values[0], 10).expect("Failed to parse value.")
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

    pub fn draw_text(&mut self, text: &str, position: Vector2, scale: f32) {
        // Check if string is purely ASCII
        if text.is_ascii() == false {
            panic!("The provided text is not ASCII!");
        }

        self.text_sprite_atlas.texture.activate();

        // Get shader uniforms
        let transform_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "transform");
        let projection_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "projection");
        let texture_bounding_box_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "bounding_box");
        let is_text_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "isText");

        // Activate font rendering in shader
        ogl::uniform_1i(is_text_location, 1);

        // Text render variables
        let mut pen_point = position;

        for my_char in text.bytes() {
            let current_character = self.character_info.get(&my_char).expect("Failed to find character.");

            self.text_sprite_atlas.position_x = pen_point.x + ((current_character.Bearing.x) * scale);
            self.text_sprite_atlas.position_y = pen_point.y + ((current_character.Bearing.y) * scale);

            self.text_sprite_atlas.set_render_view(
                current_character.TexturePosition.x, 
                current_character.TexturePosition.y, 
                current_character.Size.x, 
                current_character.Size.y);

            // Set texture bounding box
            // TODO: Make vector 4 in linear-beaglebra
            let bounding_box_array = [
                current_character.TexturePosition.x,
                current_character.TexturePosition.y, 
                current_character.Size.x,
                current_character.Size.y];
            
            ogl::uniform4fv(
                texture_bounding_box_location,
                1,
                bounding_box_array.first().expect("Failed to read bounding box values"));

            // TODO yo read up on orthographic projections again!
            let mut homemade_orthographic_projection = matrix4x4::Matrix4x4::orthographic(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0);
            let homemade_camera_translate = Vector2::new(self.camera_position_x, self.camera_position_y);
            homemade_orthographic_projection = homemade_orthographic_projection.translate(homemade_camera_translate);

            ogl::uniform_matrix_4fv(projection_location, 1, false, homemade_orthographic_projection.first());

            // Transformation testing
            // TODO yo read up on matrix math again!
            let homemade_sprite_translation_matrix = 
                Vector2::new(
                    self.text_sprite_atlas.position_x, 
                    self.text_sprite_atlas.position_y);

            let mut homemade_sprite_matrix = matrix4x4::Matrix4x4::identity();
            homemade_sprite_matrix = homemade_sprite_matrix.translate(homemade_sprite_translation_matrix);
            homemade_sprite_matrix = homemade_sprite_matrix.rotate(0.0, 0.0, self.text_sprite_atlas.angle);
            homemade_sprite_matrix = homemade_sprite_matrix.scale(
                self.text_sprite_atlas.texture_width as f32 * scale,
                self.text_sprite_atlas.texture_height as f32 * scale,
                1.0
            );

            ogl::uniform_matrix_4fv(transform_location, 1, false, homemade_sprite_matrix.first());

            ogl::draw_elements(ogl::DrawMode::Triangles, 6, ogl::ElementsDataType::UnsignedInt);

            pen_point.x += (current_character.Advance as f32) * scale;
        }

        // Disable font rendering in fragment shader
        ogl::uniform_1i(is_text_location, 0);

        // Unbind font texture atlas
        ogl::bind_texture(ogl::TextureTarget::Texture2d, 0);
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