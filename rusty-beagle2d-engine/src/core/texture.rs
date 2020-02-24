use rusty_beagle2d_glfw::ogl;
use stb_image::image;
use std::path;

pub struct Texture {
    opengl_object_id: u32,
    width: usize,
    height: usize,
    depth: usize
}

impl Texture {
    pub fn new(image_filename: String) -> Texture {
        let image_path = path::Path::new(&image_filename);
        let load_result = image::load(image_path);

        // TODO: Support images with multiple depth... right now I'll just expect PNG with depth of 4 (RGBA).
        let image_data = match load_result {
            image::LoadResult::Error(error_message) => panic!("Failed to load image: {}", error_message),
            image::LoadResult::ImageF32(imagef32) => panic!("Loaded image f32, not supported right now"),
            image::LoadResult::ImageU8(imageu8) => imageu8
        };

        let texture_object = ogl::gen_texture();
        ogl::bind_texture(ogl::TextureTarget::Texture2d, texture_object);

        ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureWrapS, ogl::TextureParameter::Repeat);
        ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureWrapT, ogl::TextureParameter::Repeat);
        ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureMinFilter, ogl::TextureParameter::Linear);
        ogl::tex_parameteri(ogl::TextureTarget::Texture2d, ogl::TextureParameterName::TextureMagFilter, ogl::TextureParameter::Linear);
    
        ogl::tex_image_2d::<u8>(ogl::TextureTarget::Texture2d,
             0,
            ogl::TextureInternalFormat::Rgba8,
            image_data.width as i32,
            image_data.height as i32,
            0, 
            ogl::TextureFormat::Rgba,
            ogl::ElementsDataType::UnsignedByte,
            image_data.data);

        ogl::generate_mipmap(ogl::TextureTarget::Texture2d);

        Texture { 
            opengl_object_id: texture_object, 
            width: image_data.width, 
            height: image_data.height, 
            depth: image_data.depth }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn activate(&self) {
        ogl::bind_texture(ogl::TextureTarget::Texture2d, self.opengl_object_id);
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        println!("Dropping Texture! :D");
    }
}