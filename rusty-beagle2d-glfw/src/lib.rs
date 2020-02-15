// I declare public modules of this crate here
pub mod glfw;

extern crate libc;

#[link(name = "glfw3-x64-debug", kind="dylib")]
// The "extern" block is used to facilitate creation and use of an FFI (Foreign Function Interface)
// An FFI is a way for a programming language to define functions and enable a different (foreign)
// Programming language to call those functions.
extern {
}

// TODO: Figure out how to split this module out into separate file in this crate
pub mod ogl {
    // TODO: Read up on mods and use statements (seems like they aren't inherited from parent module)
    use std::ffi;
    use libc::c_void;
    use std::ptr;
    use gl;
    use std::ffi::CStr;
    use std::mem;

    // type GLboolean = c_uchar;
    // type GLenum = c_uint;
    // type GLuint = c_uint;
    // type GLsizei = c_int;
    // type GLubyte = c_uchar;
    // type GLint = c_int;
    // type GLchar = c_char
    // type c_uint = u32;
    // type c_int = i32;
    // type c_char = i8
    // type c_uchar = u8

    pub enum ClearMask {
        ColorBufferBit
    }

    pub enum BufferTarget {
        ArrayBuffer,
        ElementArrayBuffer
    }

    pub enum Name {
        // TODO: Should be camel casing naming convention...
        VERSION,
        RENDERER
    }

    pub enum Capability {
        DebugOutput
    }

    pub enum Usage {
        StaticDraw
    }

    pub enum ShaderType {
        Vertex,
        Fragment
    }

    pub enum Parameter {
        CompileStatus
    }

    pub enum ProgramParameter {
        LinkStatus
    }

    pub enum DataType {
        Byte,
        UnsignedByte,
        Short,
        UnsignedShort,
        Int,
        UnsignedInt,
        HalfFloat,
        Float,
        Double
    }

    pub enum ElementsDataType {
        UnsignedByte,
        UnsignedShort,
        UnsignedInt
    }

    pub enum DrawMode {
        Triangles
    }

    pub enum TextureTarget {
        Texture2d
    }

    pub enum TextureParameterName {
        TextureWrapS,
        TextureWrapT,
        TextureMinFilter,
        TextureMagFilter
    }

    pub enum TextureParameter {
        Repeat,
        Linear
    }

    pub enum TextureInternalFormat {
        Rgb
    }

    pub enum TextureFormat {
        Rgb
    }

    pub fn init() {
        // Load OpenGL functions
        // TODO: Read up on this funky syntax (think this is called a closure?)
        gl::load_with(|s| crate::glfw::get_proc_address(s));
    }

    pub fn uniform_matrix_4fv(location: i32, count: i32, transpose: bool, value: *const f32) {
        unsafe {
            gl::UniformMatrix4fv(location, count, if transpose {1} else {0}, value);
        }
    }

    pub fn get_uniform_location(program: u32, name: &str) -> i32 {
        let c_string = ffi::CString::new(name).unwrap();
        unsafe {
            gl::GetUniformLocation(program, c_string.as_ptr())
        }
    }

    pub fn tex_parameteri(textureTarget: TextureTarget, parameterName: TextureParameterName, param: TextureParameter) {
        unsafe {
            gl::TexParameteri(match textureTarget {
                TextureTarget::Texture2d => gl::TEXTURE_2D,
            },
            match parameterName {
                TextureParameterName::TextureWrapS => gl::TEXTURE_WRAP_S,
                TextureParameterName::TextureWrapT => gl::TEXTURE_WRAP_T,
                TextureParameterName::TextureMagFilter => gl::TEXTURE_MAG_FILTER,
                TextureParameterName::TextureMinFilter => gl::TEXTURE_MIN_FILTER
            },
            match param {
                TextureParameter::Linear => gl::LINEAR as i32,
                TextureParameter::Repeat => gl::REPEAT as i32
            });
        }
    }

    pub fn generate_mipmap(textureTarget: TextureTarget) {
        unsafe {
            gl::GenerateMipmap(match textureTarget {
                TextureTarget::Texture2d => gl::TEXTURE_2D
            })
        }
    }

    pub fn tex_image_2d<T>(textureTarget: TextureTarget, level: i32, internalFormat: TextureInternalFormat, width: i32, height: i32, border: i32, format: TextureFormat, type_: ElementsDataType, pixels: Vec<T>) {
        unsafe {
            gl::TexImage2D(match textureTarget {
                TextureTarget::Texture2d => gl::TEXTURE_2D,
            },
            level,
            gl::RGB as i32,
            width,
            height,
            border,
            match format {
                TextureFormat::Rgb => gl::RGB
            },
            match type_ {
                ElementsDataType::UnsignedByte => gl::UNSIGNED_BYTE,
                ElementsDataType::UnsignedInt => gl::UNSIGNED_INT,
                ElementsDataType::UnsignedShort => gl::UNSIGNED_SHORT
            },
            pixels.as_ptr() as *const c_void);
        }
    }

    pub fn gen_texture() -> u32 {
        let mut texture_object: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_object);
            texture_object
        }
    }

    pub fn bind_texture(textureTarget: TextureTarget, texture: u32) {
        unsafe {
            gl::BindTexture(match textureTarget {
                TextureTarget::Texture2d => gl::TEXTURE_2D
            },
            texture);
        }
    }

    pub fn draw_elements(drawMode: DrawMode, count: i32, dataType: ElementsDataType) {
        unsafe {
            gl::DrawElements(match drawMode {
                DrawMode::Triangles => gl::TRIANGLES,
            },
            count,
            match dataType {
                ElementsDataType::UnsignedByte => gl::UNSIGNED_BYTE,
                ElementsDataType::UnsignedInt => gl::UNSIGNED_INT,
                ElementsDataType::UnsignedShort => gl::UNSIGNED_SHORT,
            },
            ptr::null());
        }
    }

    pub fn draw_arrays(mode: DrawMode, first: i32, count: i32) {
        unsafe {
            gl::DrawArrays(match mode {
                DrawMode::Triangles => gl::TRIANGLES,
            },
            first,
            count)
        }
    }

    pub fn bind_vertex_array(vao: u32) {
        unsafe {
            gl::BindVertexArray(vao);
        }
    }

    pub fn gen_vertex_array() -> u32 {
        unsafe {
            let size = 1;
            let mut vertex_array_object = 0;

            gl::GenVertexArrays(size, &mut vertex_array_object as *mut u32);

            vertex_array_object
        }
    }

    pub fn enable_vertex_attrib_array(index: u32) {
        unsafe {
            gl::EnableVertexAttribArray(index);
        }
    }

    // TODO: Read up on this thing with Rust not supporting function ovleroading
    pub fn vertex_attrib_pointer_no_offset(index: u32, 
        size: i32, 
        dataType: DataType, 
        normalized: bool, 
        stride: i32) {
        
        let normalized: u8 = if normalized {1} else {0};
            
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                match dataType {
                    DataType::Byte => gl::BYTE,
                    DataType::Double => gl::DOUBLE,
                    DataType::Float => gl::FLOAT,
                    DataType::HalfFloat => gl::HALF_FLOAT,
                    DataType::Int => gl::INT,
                    DataType::Short => gl::SHORT,
                    DataType::UnsignedByte => gl::UNSIGNED_BYTE,
                    DataType::UnsignedInt => gl::UNSIGNED_INT,
                    DataType::UnsignedShort => gl::UNSIGNED_SHORT
                },
                normalized,
                stride,
                ptr::null()
            );
        }
    }

    pub fn vertex_attrib_pointer(index: u32, 
        size: i32, 
        dataType: DataType, 
        normalized: bool, 
        stride: i32,
        offset: u32) {
        
        let normalized: u8 = if normalized {1} else {0};
            
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                match dataType {
                    DataType::Byte => gl::BYTE,
                    DataType::Double => gl::DOUBLE,
                    DataType::Float => gl::FLOAT,
                    DataType::HalfFloat => gl::HALF_FLOAT,
                    DataType::Int => gl::INT,
                    DataType::Short => gl::SHORT,
                    DataType::UnsignedByte => gl::UNSIGNED_BYTE,
                    DataType::UnsignedInt => gl::UNSIGNED_INT,
                    DataType::UnsignedShort => gl::UNSIGNED_SHORT
                },
                normalized,
                stride,
                offset as *const c_void
            );
        }
    }

    pub fn delete_shader(shader: u32) {
        unsafe {
            gl::DeleteShader(shader);
        }
    }

    pub fn use_program(program: u32) {
        unsafe {
            gl::UseProgram(program);
        }
    }

    pub fn get_program_info_log(program: u32) -> String {
        unsafe {
            let string_bytes: Vec<u8> = vec![0; 512];
            let info_log = ffi::CString::from_vec_unchecked(string_bytes);
            let raw_string_ptr = info_log.into_raw();

            gl::GetProgramInfoLog(program, 512, ptr::null_mut(), raw_string_ptr);

            ffi::CString::from_raw(raw_string_ptr).into_string().unwrap()
        }
    }

    pub fn get_programiv(program: u32, pname: ProgramParameter) -> i32 {
        unsafe {
            let mut return_value: i32 = 0;

            gl::GetProgramiv(program, 
            match pname {
                ProgramParameter::LinkStatus => gl::LINK_STATUS,
            }, &mut return_value as *mut i32);

            return_value
        }
    }

    pub fn link_program(program: u32) {
        unsafe {
            gl::LinkProgram(program);
        }
    }

    pub fn attach_shader(program: u32, shader: u32) {
        unsafe {
            gl::AttachShader(program, shader);
        }
    }

    pub fn create_program() -> u32 {
        unsafe {
            gl::CreateProgram()
        }
    }

    pub fn get_shader_info_log(shader: u32) -> String {
        unsafe {
            // TODO: Obviously I should query OpenGL for the length of the info log and use that to create the capacity for Vec...
            let mut infoLog = vec![0; 512];

            let lol = ffi::CString::from_vec_unchecked(infoLog);

            // I convert the created CString into a raw mutable pointer, and transforms ownership.
            // TODO: Not sure if that's actually needed? Could I just take the pointer of the vec directly?
            let raw_pointer = lol.into_raw();

            // Retrieve the shader log, returned in the CString previously created.
            gl::GetShaderInfoLog(shader, 512, ptr::null_mut() as *mut i32, raw_pointer);

            // Here I retake ownership of the CString previously transferred to C via into_raw.
            // This has to be called after a call to "into_raw". Failure to do so will result in a memory leak.
            ffi::CString::from_raw(raw_pointer).into_string().unwrap()
        }
    }

    pub fn get_shader(shader: u32, parameter_name: Parameter) -> i32 {
        unsafe {
            let mut parameter_value = 0;

            gl::GetShaderiv(shader, match parameter_name {
                Parameter::CompileStatus => gl::COMPILE_STATUS,
            },
            &mut parameter_value as *mut i32);

            parameter_value
        }
    }

    pub fn compile_shader(shader: u32) {
        unsafe {
            gl::CompileShader(shader);
        }
    }

    pub fn create_shader(shader_type: ShaderType) -> u32 {
        unsafe {
            // TODO: Not doing proper error handling on CreateShader
            gl::CreateShader(match shader_type {
                ShaderType::Vertex => gl::VERTEX_SHADER,
                ShaderType::Fragment => gl::FRAGMENT_SHADER,
            })
        }
    }

    // TODO: Is *const *const GLchar reprasentative with a vector of strings??
    pub fn shader_source(vertexShader: u32, count: i32, strings: &Vec<&String>) {
        // TODO: Not sure how to do this more efficiently / more elegant
        // Question is: How do you convert &Vec<&String> most consicely to SAFE C Strings of *const *const i8?
        let mut safe_c_strings: Vec<ffi::CString> = Vec::new();
        let mut pointers_to_safe_c_strings: Vec<*const i8> = Vec::new();

        // TODO: Read up on Rust's Iterators
        for element in strings.iter() {
            let safe_string = ffi::CString::new(element.as_bytes()).unwrap();
            pointers_to_safe_c_strings.push(safe_string.as_ptr());
            safe_c_strings.push(safe_string);
        }

        unsafe {
            // Last parameter gives an array of string lengths for each element of the strings
            // Setting it to null means that it is assumed that each string element is null-terminated.
            // NOTE ON LIFETIME: OpenGL will copy the source code strings, so it's not necessary for me
            // To keep them alive after this function call has returned.
            gl::ShaderSource(vertexShader, count, pointers_to_safe_c_strings.as_ptr(), ptr::null());
        }
    }

    pub fn clear(mask: ClearMask) {
        // TODO: Maybe I don't need match for this... (could just assign value in enum definition)
        let mask = match mask {
            ClearMask::ColorBufferBit => gl::COLOR_BUFFER_BIT
        };

        unsafe {
            gl::Clear(mask);
        }
    }

    pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }

    pub fn gl_gen_buffers(n: i32, buffers: &mut [u32]) {
        // TODO: Should do safety check to make sure n = buffers length
        unsafe {
            match buffers.first_mut() {
                None => panic!("No elements!"),
                Some(x) => gl::GenBuffers(n, x),
            }
        }
    }

    pub fn gl_gen_buffer() -> u32 {
        let mut buffer = 0;

        unsafe {
            gl::GenBuffers(1, &mut buffer);
            buffer
        }
    }

    pub fn gl_bind_buffer(target: BufferTarget, buffer: u32) {
        unsafe {
            gl::BindBuffer(
                match target {
                    BufferTarget::ArrayBuffer => gl::ARRAY_BUFFER,
                    BufferTarget::ElementArrayBuffer => gl::ELEMENT_ARRAY_BUFFER
                },
                buffer
            );
        }
    }

    // TODO: Gotta find a way to not pass mutable reference...
    pub fn BufferData<T>(target: BufferTarget, data: &Vec<T>, usage: Usage) {
        let typeSize = mem::size_of::<T>() * data.len();

        unsafe {
            gl::BufferData(
                match target {
                    BufferTarget::ArrayBuffer => gl::ARRAY_BUFFER,
                    BufferTarget::ElementArrayBuffer => gl::ELEMENT_ARRAY_BUFFER
                },
                typeSize as isize, // TODO: Gotta read up on "as" conversion... also, is it even safe to go from usize to isize? I suppose right??
                data.as_ptr() as *const c_void,
                match usage {
                    Usage::StaticDraw => gl::STATIC_DRAW
                }
            )
        }
    }

    pub fn gl_get_string(name: Name) -> String {
        unsafe {
            // TODO: Yeah... gotta read up on this conversion
            ffi::CStr::from_ptr(gl::GetString(
                match name {
                Name::VERSION => gl::VERSION,
                Name::RENDERER => gl::RENDERER
            }) as *const i8).to_string_lossy().into_owned()
        }
    }

    // Functions coerce / turn into the type "fn". The "fn" type is called a "function pointer".
    pub fn gl_debug_message_callback(callback: fn(u32, u32, u32, u32, i32, String)) {
        unsafe {
            gl::DebugMessageCallback(Some(debug_callback), ptr::null::<c_void>())
        }
    }

    pub fn gl_enable(capability: Capability) {
        unsafe {
            gl::Enable(match capability {
                Capability::DebugOutput => gl::DEBUG_OUTPUT
            });
        }
    }

    // TODO: What does "extern system" mean?
    extern "system" fn debug_callback(source: gl::types::GLenum, gltype: gl::types::GLenum, id: gl::types::GLuint, severity: gl::types::GLenum, length: gl::types::GLsizei, message: *const gl::types::GLchar, userParam: *mut c_void) {
        unsafe {
            let owned_message = CStr::from_ptr(message).to_string_lossy().into_owned();
            println!("OpenGL Error: {}", owned_message);
        }
    }
}