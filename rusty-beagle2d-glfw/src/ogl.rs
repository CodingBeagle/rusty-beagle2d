use std::ffi;
use libc::c_void;
use std::ptr;
use gl;
use std::ffi::CStr;
use std::mem;
use ffi::CString;

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

#[repr(u32)]
pub enum ClearMask {
    ColorBufferBit = gl::COLOR_BUFFER_BIT
}

#[repr(u32)]
pub enum BufferTarget {
    ArrayBuffer = gl::ARRAY_BUFFER,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER
}

#[repr(u32)]
pub enum Name {
    Version = gl::VERSION,
    Renderer = gl::RENDERER
}

#[repr(u32)]
pub enum Capability {
    DebugOutput = gl::DEBUG_OUTPUT
}

// TODO: Refactor to be convertable from u32 like my newest enum pattern
pub enum Usage {
    DynamicDraw,
    StaticDraw
}

#[repr(u32)]
pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER
}

#[repr(u32)]
pub enum Parameter {
    CompileStatus = gl::COMPILE_STATUS
}

#[repr(u32)]
pub enum ProgramParameter {
    LinkStatus = gl::LINK_STATUS
}

#[repr(u32)]
pub enum DataType {
    Byte = gl::BYTE,
    UnsignedByte = gl::UNSIGNED_BYTE,
    Short = gl::SHORT,
    UnsignedShort = gl::UNSIGNED_SHORT,
    Int = gl::INT,
    UnsignedInt = gl::UNSIGNED_INT,
    HalfFloat = gl::HALF_FLOAT,
    Float = gl::FLOAT,
    Double = gl::DOUBLE
}

#[repr(u32)]
pub enum ElementsDataType {
    UnsignedByte = gl::UNSIGNED_BYTE,
    UnsignedShort = gl::UNSIGNED_SHORT,
    UnsignedInt = gl::UNSIGNED_INT
}

#[repr(u32)]
pub enum DrawMode {
    Triangles = gl::TRIANGLES
}

// Representable as u32 to make it easy to cast it to C API.
// Haven't found a better way of working with enums, the alternative
// Would be using big "match" expressions to make the mapping between name and C API value...
// Which is very tedious coding and harder to read / comprehend when viewing the function call.
#[repr(u32)]
pub enum TextureTarget {
    Texture2d = gl::TEXTURE_2D
}

// Representable as u32 to make it easy to cast it to C API.
#[repr(u32)]
pub enum TextureParameterName {
    TextureWrapS = gl::TEXTURE_WRAP_S,
    TextureWrapT = gl::TEXTURE_WRAP_T,
    TextureMinFilter = gl::TEXTURE_MIN_FILTER,
    TextureMagFilter = gl::TEXTURE_MAG_FILTER
}

// Representable as u32 to make it easy to cast it to C API.
#[repr(u32)]
pub enum TextureParameter {
    ClampToEdge = gl::CLAMP_TO_EDGE,
    Repeat = gl::REPEAT,
    Linear = gl::LINEAR
}

#[repr(u32)]
pub enum TextureInternalFormat {
    Red = gl::RED,
    Rgb = gl::RGB,
    Rgba = gl::RGBA,
    Rgba8 = gl::RGBA8
}

#[repr(u32)]
pub enum TextureFormat {
    Red = gl::RED,
    Rgb = gl::RGB,
    Rgba = gl::RGBA
}

#[repr(u32)]
pub enum BlendFactor {
    SrcAlpha = gl::SRC_ALPHA,
    OneMinusSrcAlpha = gl::ONE_MINUS_SRC_ALPHA
}

#[repr(u32)]
pub enum AlignmentParameter {
    PackAlignment = gl::PACK_ALIGNMENT,
    UnpackAlignment = gl::UNPACK_ALIGNMENT
}

// TODO: Give better name
#[repr(u32)]
pub enum Cap {
    Blend = gl::BLEND
}

pub fn init() {
    // Load OpenGL functions
    // I use a closure here.
    // A closure is an anonymous function that can be saved in variables or passed to functions.
    // That code can then call the closure later to evaluate its result.
    // The pipe syntax | | is used to specify parameters to the closure.
    // Afterwards, you specify the body of the closure with {}. If the closure is a single expression,
    // The curly braces are optional.
    gl::load_with(|s| crate::glfw::get_proc_address(s));
}

pub fn uniform_1i(location: i32, param: i32) {
    unsafe {
        gl::Uniform1i(location, param);
    }
}

pub fn pixel_storei(alignmentParameter: AlignmentParameter, parameterValue: i32) {
    unsafe {
        gl::PixelStorei(alignmentParameter as u32, parameterValue);
    }
}

pub fn enable(capability: Cap) {
    unsafe {
        gl::Enable(capability as u32);
    }
}

pub fn blend_func(sfactor: BlendFactor, dfactor: BlendFactor) {
    unsafe {
        gl::BlendFunc(sfactor as u32, dfactor as u32);
    }
}

pub fn uniform_matrix_4fv(location: i32, count: i32, transpose: bool, value: *const f32) {
    unsafe {
        gl::UniformMatrix4fv(location, count, if transpose {1} else {0}, value);
    }
}

pub fn get_uniform_location(program: u32, name: &str) -> i32 {
    unsafe {
        gl::GetUniformLocation(program, CString::new(name).unwrap().as_ptr())
    }
}

pub fn tex_parameteri(texture_target: TextureTarget,parameter_name: TextureParameterName, param: TextureParameter) {
    unsafe {
        gl::TexParameteri(
            texture_target as u32,
            parameter_name as u32,
            param as i32);
    }
}

pub fn generate_mipmap(texture_target: TextureTarget) {
    unsafe {
        gl::GenerateMipmap(texture_target as u32);
    }
}

pub fn tex_image_2d<T>(texture_target: TextureTarget, level: i32, internal_format: TextureInternalFormat, width: i32, height: i32, border: i32, format: TextureFormat, type_: ElementsDataType, pixels: Vec<T>) {
    unsafe {
        gl::TexImage2D(texture_target as u32,
        level,
        internal_format as i32,
        width,
        height,
        border,
        format as u32,
        type_ as u32,
        pixels.as_ptr() as *const c_void);
    }
}

// TODO: It appears that in some cases, it's easier to allow passing raw pointers as pixel data, instead of having to convert raw pointers from other libraries to Rust containers, just to convert back again
pub fn tex_image_2d_from_raw(texture_target: TextureTarget, level: i32, internal_format: TextureInternalFormat, width: i32, height: i32, border: i32, format: TextureFormat, type_: ElementsDataType, pixels: *const c_void) {
    unsafe {
        gl::TexImage2D(texture_target as u32,
        level,
        internal_format as i32,
        width,
        height,
        border,
        format as u32,
        type_ as u32,
        pixels);
    }
}

pub fn gen_texture() -> u32 {
    let mut texture_object: u32 = 0;

    unsafe {
        gl::GenTextures(1, &mut texture_object);
        texture_object
    }
}

pub fn bind_texture(texture_target: TextureTarget, texture: u32) {
    unsafe {
        gl::BindTexture(texture_target as u32, texture);
    }
}

pub fn draw_elements(draw_mode: DrawMode, count: i32, data_type: ElementsDataType) {
    unsafe {
        gl::DrawElements(draw_mode as u32,
        count,
        data_type as u32,
        ptr::null());
    }
}

pub fn draw_arrays(draw_mode: DrawMode, first: i32, count: i32) {
    unsafe {
        gl::DrawArrays(draw_mode as u32, first, count)
    }
}

pub fn bind_vertex_array(vao: u32) {
    unsafe {
        gl::BindVertexArray(vao);
    }
}

pub fn gen_vertex_array() -> u32 {
    unsafe {
        let mut vertex_array_object = 0;
        gl::GenVertexArrays(1, &mut vertex_array_object as *mut u32);
        vertex_array_object
    }
}

pub fn enable_vertex_attrib_array(index: u32) {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }
}

pub fn vertex_attrib_pointer(index: u32, size: i32, data_type: DataType, normalized: bool, stride: i32, offset: u32) {
    unsafe {
        gl::VertexAttribPointer(index, size, data_type as u32, if normalized {1} else {0}, stride, offset as *const c_void
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
        let raw_string_ptr = ffi::CString::from_vec_unchecked(vec![0; 512]).into_raw();
        gl::GetProgramInfoLog(program, 512, ptr::null_mut(), raw_string_ptr);
        ffi::CString::from_raw(raw_string_ptr).into_string().unwrap()
    }
}

pub fn get_programiv(program: u32, pname: ProgramParameter) -> i32 {
    unsafe {
        let mut return_value: i32 = 0;
        gl::GetProgramiv(program, pname as u32, &mut return_value as *mut i32);
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
        // I convert the created CString into a raw mutable pointer, and transforms ownership.
        // TODO: Not sure if that's actually needed? Could I just take the pointer of the vec directly?
        let raw_pointer = ffi::CString::from_vec_unchecked(vec![0; 512]).into_raw();

        // Retrieve the shader log, returned in the CString previously created.
        gl::GetShaderInfoLog(shader, 512, ptr::null_mut() as *mut i32, raw_pointer);

        // Here I retake ownership of the CString previously transferred to C via into_raw.
        // This has to be called after a call to "into_raw". Failure to do so will result in a memory leak.
        ffi::CString::from_raw(raw_pointer).into_string().unwrap()
    }
}

pub fn get_shader(shader: u32, parameter_name: Parameter) -> i32 {
    let mut parameter_value = 0;

    unsafe {
        gl::GetShaderiv(shader, parameter_name as u32, &mut parameter_value as *mut i32);
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
        gl::CreateShader(shader_type as u32)
    }
}

pub fn shader_source(vertex_shader: u32, count: i32, strings: &Vec<&String>) {
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
        gl::ShaderSource(vertex_shader, count, pointers_to_safe_c_strings.as_ptr(), ptr::null());
    }
}

pub fn clear(clear_mask: ClearMask) {
    unsafe {
        gl::Clear(clear_mask as u32);
    }
}

pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
    }
}

pub fn gl_gen_buffer() -> u32 {
    let mut buffer = 0;

    unsafe {
        gl::GenBuffers(1, &mut buffer);
        buffer
    }
}

pub fn gl_bind_buffer(buffer_target: BufferTarget, buffer: u32) {
    unsafe {
        gl::BindBuffer(buffer_target as u32, buffer);
    }
}

pub fn buffer_data<T>(buffer_target: BufferTarget, data: &Vec<T>, usage: Usage) {
    unsafe {
        gl::BufferData(
            match buffer_target {
                BufferTarget::ArrayBuffer => gl::ARRAY_BUFFER,
                BufferTarget::ElementArrayBuffer => gl::ELEMENT_ARRAY_BUFFER
            },
            (mem::size_of::<T>() * data.len()) as isize,
            data.as_ptr() as *const c_void,
            match usage {
                Usage::StaticDraw => gl::STATIC_DRAW,
                Usage::DynamicDraw => gl::DYNAMIC_DRAW
            }
        )
    }
}

pub fn gl_get_string(name: Name) -> String {
    unsafe {
        ffi::CStr::from_ptr(gl::GetString(name as u32) as *const i8).to_string_lossy().into_owned()
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
        gl::Enable(capability as u32);
    }
}

extern "system" fn debug_callback(source: gl::types::GLenum, gltype: gl::types::GLenum, id: gl::types::GLuint, severity: gl::types::GLenum, length: gl::types::GLsizei, message: *const gl::types::GLchar, userParam: *mut c_void) {
    unsafe {
        let owned_message = CStr::from_ptr(message).to_string_lossy().into_owned();
        println!("OpenGL Error: {}", owned_message);
    }
}