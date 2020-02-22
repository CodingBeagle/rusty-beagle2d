use rusty_beagle2d_glfw::ogl;
use crate::core::shader;

pub struct ShaderProgram {
    opengl_object_id: u32
}

impl ShaderProgram {
    pub fn new(vertex_shader: shader::Shader, fragment_shader: shader::Shader) -> ShaderProgram {
        let shader_program = ogl::create_program();

        if vertex_shader.get_shader_type() == fragment_shader.get_shader_type()
        {
            panic!("Vertex Shader and Fragment Shader cannot be the same type!");
        }

        ogl::attach_shader(shader_program, vertex_shader.get_opengl_object_id());
        ogl::attach_shader(shader_program, fragment_shader.get_opengl_object_id());

        ogl::link_program(shader_program);

        let link_status = ogl::get_programiv(shader_program, ogl::ProgramParameter::LinkStatus);
        if link_status != 1 {
            panic!("Failed to link shader program! {}", ogl::get_program_info_log(shader_program));
        }

        ShaderProgram { opengl_object_id: shader_program }
    }

    pub fn activate(&self) {
        ogl::use_program(self.opengl_object_id);
    }

    pub fn get_opengl_object_id(&self) -> u32 {
        self.opengl_object_id
    }
}