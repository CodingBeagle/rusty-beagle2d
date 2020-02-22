use crate::core::texture;

// LEARN Lifetime types in Structs
// The "texture" field of Sprite is a reference to an already existing Texture instance
// That exists in another part of the program.
// Because we have a reference in a struct field, we must explicitly state a generic lifetime
// parameter, that can be used by the Rust borrow checker to validate that an instance of a
// Sprite does not outlive the Texture it references. It if did, we could hit a case of having
// A dangling reference to a texture no longer existing in the program.
pub struct Sprite<'a> {
    pub position_x: u32,
    pub position_y: u32,
    pub angle: f32,
    pub uniform_scale: f32,
    pub texture: &'a texture::Texture
}

// LEARN: Lifetime annotations in Method Definitions
// When we implement methods on a struct with lifetimes,
// We use the same syntax as that of generic type parameters.
// Lifetime names for struct fields always need to be declared after the "impl"
// keyword and then used after the struct's name, because those lifetimes are part of
// the struct's type.
impl<'a> Sprite<'a> {
    pub fn new(sprite_texture: &'a texture::Texture) -> Sprite<'a> {
        Sprite { 
            position_x: 0,
            position_y: 0,
            angle: 0.0,
            uniform_scale: 1.0,
            texture: sprite_texture
        }
    }
}