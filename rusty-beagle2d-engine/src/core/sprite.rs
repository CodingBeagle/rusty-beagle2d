use crate::core::texture;

use std::boxed::{Box};

// LEARN Lifetime types in Structs
// The "texture" field of Sprite is a reference to an already existing Texture instance
// That exists in another part of the program.
// Because we have a reference in a struct field, we must explicitly state a generic lifetime
// parameter, that can be used by the Rust borrow checker to validate that an instance of a
// Sprite does not outlive the Texture it references. It if did, we could hit a case of having
// A dangling reference to a texture no longer existing in the program.
pub struct Sprite {
    pub position_x: f32,
    pub position_y: f32,
    pub texture_x: f32,
    pub texture_y: f32,
    pub texture_width: f32,
    pub texture_height: f32,
    pub angle: f32,
    pub uniform_scale: f32,
    pub texture: Box<texture::Texture> // LEARN: Read up on Rust Box's
}

// LEARN: Lifetime annotations in Method Definitions
// When we implement methods on a struct with lifetimes,
// We use the same syntax as that of generic type parameters.
// Lifetime names for struct fields always need to be declared after the "impl"
// keyword and then used after the struct's name, because those lifetimes are part of
// the struct's type.
impl Sprite {
    pub fn new(sprite_texture: Box<texture::Texture>) -> Sprite {
        Sprite { 
            position_x: 0.0,
            position_y: 0.0,
            texture_x: 0.0,
            texture_y: 0.0,
            texture_width: sprite_texture.get_width() as f32,
            texture_height: sprite_texture.get_height() as f32,
            angle: 0.0,
            uniform_scale: 1.0,
            texture: sprite_texture
        }
    }

    pub fn set_render_view(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.texture_x = x;
        self.texture_y = y;
        self.texture_width = width;
        self.texture_height = height;
    }
}