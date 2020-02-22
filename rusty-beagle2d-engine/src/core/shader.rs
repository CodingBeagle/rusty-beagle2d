use std::fs;
use rusty_beagle2d_glfw::ogl;

// LEARN: Copy and Clone traits
// You can only derive the Copy trait on types that also implement the Clone trait.
// The Copy trait allows you to duplicate a value by only copying bits stored on the stack.
// No arbritrary code is required.
// The "PartialEq" trait used with enums means that each variant is equal to itself and not equal to other variants-
// Deriving from this Trati means that you can use the enum in if statements like "ShaderTypeA == ShaderTypeB"
#[derive(Copy, Clone, PartialEq)]
pub enum ShaderType {
    VertexShader,
    FragmentShader
}

pub struct Shader {
    source_code: String,
    shader_type: ShaderType,
    opengl_object_id: u32
}

// LEARN: impl blocks
// To define methods (functions within the context of a Struct), we use an implementation (impl) block.
// 
impl Shader {
    // LEARN: Associated functions
    // Functions within an impl block that do NOT take "self" as a parameter are called Associated Functions.
    // They are associated with the struct type and not specific instances of the Struct.
    // They are like static methods in languages like C#
    // In Rust, they are typically used for convinience functions, like a constructor, a way to create a new instance of the associated struct.
    pub fn new(shader_type: ShaderType, source_code_file: String) -> Shader {
        let source_code = fs::read_to_string(source_code_file).expect("Failed to read from provided source code file.");

        let shader_object = ogl::create_shader(match shader_type {
            ShaderType::FragmentShader => ogl::ShaderType::Fragment,
            ShaderType::VertexShader => ogl::ShaderType::Vertex
        });
        ogl::shader_source(shader_object, 1, &vec![&source_code]);
        ogl::compile_shader(shader_object);

        let shader_compilation_result = ogl::get_shader(shader_object, ogl::Parameter::CompileStatus);
        if shader_compilation_result != 1 {
            let compilation_report = ogl::get_shader_info_log(shader_object);
            panic!("Failed to compile vertex shader: {}", compilation_report);
        }

        Shader { source_code: String::from(source_code), opengl_object_id: shader_object, shader_type }
    }

    pub fn get_shader_type(&self) -> ShaderType {
        self.shader_type
    }

    pub fn get_opengl_object_id(&self) -> u32 {
        self.opengl_object_id
    }
}

// LEARN: Drop
// Drop is like a destructor, in that it lets you customize what should happen when a value goes out of scope.
// In this case we make sure to do OpenGl resource cleanup that was allocated during construction of the struct.
impl Drop for Shader {
    fn drop(&mut self) {
        println!("Dropping shader! :D");
        ogl::delete_shader(self.opengl_object_id);
    }
}