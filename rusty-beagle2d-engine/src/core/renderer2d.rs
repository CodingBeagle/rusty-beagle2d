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
    camera_position_y: f32,
    characters: HashMap<u8, Character>
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

        let mut characters: HashMap<u8, Character> = HashMap::new();

        // FreeType Loading
        // LEARN: Read up on FreeType in general and the theory behind what it does and why
        unsafe {
            // Initialize FreeType
            let mut ft: freetype::FT_Library = ptr::null_mut();

            let init_result = freetype::FT_Init_FreeType(&mut ft);
            if init_result != 0 {
                panic!("Failed to initialize FreeType!");
            }

            // Initialize face
            let mut ft_face: freetype::FT_Face = ptr::null_mut();

            let path = CString::new("test-dat/fonts/arial.ttf").expect("Failed to create CString");
            let ft_face_loading_result = freetype::FT_New_Face(ft, path.as_ptr(), 0, &mut ft_face);

            if ft_face_loading_result != 0 {
                panic!("Failed to load font!");
            }

            // Define font size
            let ft_set_size_result = freetype::FT_Set_Pixel_Sizes(ft_face, 0, 128);
            if ft_set_size_result != 0 {
                panic!("Failed to set size of font!");
            }

            // OpenGl requires that textures all have a 4-byte alignment.
            // e.g: Their size is always a multiple of 4 bytes.
            // Normally this won't be a problem since most textures have a width that is a multiple of 4 and/or
            // use 4 bytes per pixel.
            // However, since we now only use a single byte per pixel they can have any possible width. By Setting
            // its unpack alignment equal to 1, we ensure there are no alignment issues (which can cause segmentation faults).
            ogl::pixel_storei(ogl::AlignmentParameter::UnpackAlignment, 1);

            for x in 0..128 {
                // Load character glyph
                let ft_load_char_result = freetype::FT_Load_Char(ft_face, x, freetype::FT_LOAD_RENDER as i32);

                if ft_load_char_result != 0 {
                    panic!("Failed to load character glyph!");
                }

                // Generate Texture
                let texture_id: u32 = ogl::gen_texture();
                ogl::bind_texture(ogl::TextureTarget::Texture2d, texture_id);

                // LEARN: Read up more on glypgh's and their info in FreeType library
                let glyph_info = *(*ft_face).glyph;
                let bitmap_info = glyph_info.bitmap;
                
                ogl::tex_image_2d_from_raw(
                    ogl::TextureTarget::Texture2d,
                    0,
                    ogl::TextureInternalFormat::Red,
                    bitmap_info.width as i32,
                    bitmap_info.rows as i32, 
                        0, 
                        ogl::TextureFormat::Red,
                        ogl::ElementsDataType::UnsignedByte,  
                        bitmap_info.buffer as *const c_void);

                // Set texture options
                ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureWrapS, ogl::TextureParameter::ClampToEdge);
                ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureWrapT, ogl::TextureParameter::ClampToEdge);
                ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureMinFilter, ogl::TextureParameter::Linear);
                ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureMagFilter, ogl::TextureParameter::Linear);

                // Store character for later use
                // TODO: Now would be a nice time to have generic Vectors in my linear algebra library :) (so I can store as ints intead of f32's)
                let new_character = Character {
                    TextureId: texture_id,
                    Size: Vector2::new(bitmap_info.width as f32, bitmap_info.rows as f32),
                    Bearing: Vector2::new(glyph_info.bitmap_left as f32, glyph_info.bitmap_top as f32),
                    Advance: glyph_info.advance.x as u32,
                    // LEARN: What is "font units" in FreeType???
                    MaxHeight: ((*ft_face).height >> 6) as u32 // The vertical distance between two consecutive baseline, expressed in font units.
                };

                characters.insert(x as u8, new_character);
            }

            // Clean up FreeType library and memory
            // LEARN: Read up more on what these function calls do.
            freetype::FT_Done_Face(ft_face);
            freetype::FT_Done_FreeType(ft);
        }

        Renderer2d {
            shader_program: shader_program,
            camera_position_x: 0.0,
            camera_position_y: 0.0,
            characters
        }
    }

    pub fn draw_sprite(&self, sprite: &sprite::Sprite) {
        // Activate the Sprite's texture for the OpenGl context
        sprite.texture.activate();

        let transform_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "transform");
        let projection_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "projection");

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
            sprite.texture.get_width() as f32,
            sprite.texture.get_height() as f32,
            1.0
        );

        ogl::uniform_matrix_4fv(transform_location, 1, false, homemade_sprite_matrix.first());

        ogl::draw_elements(ogl::DrawMode::Triangles, 6, ogl::ElementsDataType::UnsignedInt);
    }

    // LEARN: Difference between String and string slice
    pub fn draw_text(&self, position: Vector2, text: &str) {
        // Get Uniform Locations
        let projection_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "projection");
        let model_matrix_location = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "transform");
        let is_text_uniform = ogl::get_uniform_location(self.shader_program.get_opengl_object_id(), "isText");

        // Orthographic Projection
        let mut homemade_orthographic_projection = matrix4x4::Matrix4x4::orthographic(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0);
        let homemade_camera_translate = Vector2::new(self.camera_position_x, self.camera_position_y);
        homemade_orthographic_projection = homemade_orthographic_projection.translate(homemade_camera_translate);

        ogl::uniform_matrix_4fv(projection_location, 1, false, homemade_orthographic_projection.first());

        // Enable text rendering in shader
        ogl::uniform_1i(is_text_uniform, 1);

        let mut pen_point = position.x;

        let scale: f32 = 0.25;

        // TODO: Simply iterating over bytes in a UTF-8 string is to get ASCII chars is prolly not great...
        for character in text.bytes() {
            let character_info = self.characters.get(&character).expect("Failed to retrieve character code!");

            // Calculate font position
            let character_x_pos = pen_point as f32 + (character_info.Bearing.x * (scale as f32));

            // TODO: Perhaps there's a nicer way of making sure that pos (0, 0) will be a string nicely printed at the top left corner of the screen....
            // Right now I simply align each character nicely with the origin and then offset each character by what is 1 line height...
            let character_y_pos = (position.y - character_info.Bearing.y * (scale as f32)) + ((character_info.MaxHeight * (scale as u32)) as f32);
            
            let mut character_matrix = matrix4x4::Matrix4x4::identity();
            character_matrix = character_matrix.translate(Vector2::new(character_x_pos, character_y_pos));
            character_matrix = character_matrix.scale(character_info.Size.x * scale as f32, character_info.Size.y * scale as f32, 1.0);

            // Bind character glyph image
            ogl::bind_texture(ogl::TextureTarget::Texture2d, character_info.TextureId);

            // Set model matrix
            ogl::uniform_matrix_4fv(model_matrix_location, 1, false, character_matrix.first());

            // Character image draw call
            ogl::draw_elements(ogl::DrawMode::Triangles, 6, ogl::ElementsDataType::UnsignedInt);

            // Unbind character glyph image
            ogl::bind_texture(ogl::TextureTarget::Texture2d, 0);

            // Advance pen point position
            // LEARN: Why do you need to bitshift with 6 in FreeType when advancing pen point?
            pen_point += ((character_info.Advance >> 6) as f32) * scale;
        }

        // Disable text rendering in shader
        ogl::uniform_1i(is_text_uniform, 0);
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