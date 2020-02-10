extern crate libc;
use libc::c_int;
use libc::c_char;
use libc::c_void;
use std::ffi::CString;
use std::ptr;

// TODO: Read up on repr(C) meaning
#[repr(C)] pub struct GLFWmonitor { _private: [u8; 0] }
#[repr(C)] pub struct GLFWwindow { _private: [u8; 0] }

// By default, everything in Rust is private.
// Except in two cases:
// - Associated items in a "pub" Trait are public by default.
// - Enum variants in a "pub" enum are also public by default.
// Visibility in Rust works on a per-module basis.
// Defining Window as "pub" means that it can be accessed from an external module.
// If an item is private (like window_ptr field in Window), it can only be accessed by the current module.
// This can be used to create module hierarchies exposing public APIs while hiding internal implementation details.
// TODO: Read up on Modules! There's important things to be understood in regards to visibility in Rust
pub struct Window {
    window_ptr: *mut GLFWwindow
}

pub struct Monitor {
    monitor_ptr: *mut GLFWmonitor
}

#[link(name = "glfw3-x64-debug", kind="dylib")]
// The "extern" block is used to facilitate creation and use of an FFI (Foreign Function Interface)
// An FFI is a way for a programming language to define functions and enable a different (foreign)
// Programming language to call those functions.
extern {
    // TODO: I suppose a more idiomatic rust way would be to return a Result?
    fn glfwInit() -> c_int;

    // Rust has two raw pointer types we can use:
    // Mutable (*mut T) and immutable ones (*const T)
    // Actually, you CAN create raw pointers in safe code. You just can't dereference raw pointers outside safe blocks.
    fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor:  *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;

    fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;

    fn glfwMakeContextCurrent(window: *mut GLFWwindow);

    fn glfwSwapBuffers(window: *mut GLFWwindow);

    // TODO: Is this really correct way to return function pointer?
    fn glfwGetProcAddress(procname: *const c_char) -> *const c_void;

    fn glfwTerminate();

    fn glfwPollEvents();

    fn glfwWindowHint(hint: c_int, value: c_int);
}

// TODO: Apparently there's some way to make enums of primitives? Look into this
pub enum WindowHint {
    Resizable = 0x00020003,
    Focused = 0x00020001,
    ContextVersionMajor = 0x00022002,
    ContextVersionMinor = 0x00022003,
    OpenGlDebugContext = 0x00022007,
    OpenGlProfile = 0x00022008
}

pub enum OpenGlProfile {
    CoreProfile = 0x00032001,
    CompatProfile,
    AnyProfile
}

pub enum GlfwBool {
    True = 1,
    False = 0
}

pub fn glfw_get_proc_address(procname: &'static str) -> *const c_void {
    // TODO: Is this c string lifetime right?
    let title_c_string =  CString::new(procname).expect("Failed to create title string!");

    unsafe {
        glfwGetProcAddress(title_c_string.as_ptr()) as *const c_void
    }
}

pub fn glfw_window_hint(hint: WindowHint, value: i32) {
    unsafe {
        glfwWindowHint(hint as i32, value as i32)
    }
}

pub fn glfw_poll_events() {
    unsafe {
        glfwPollEvents();
    }
}

pub fn glfw_init() -> bool {
    unsafe {
        glfwInit() == 1
    }
}

pub fn glfw_terminate() {
    unsafe {
        glfwTerminate();
    }
}

pub fn glfw_swap_buffers(window: &Window) {
    unsafe {
        glfwSwapBuffers(window.window_ptr);
    }
}

pub fn glfw_window_should_close(window: &Window) -> bool {
    unsafe {
        glfwWindowShouldClose(window.window_ptr) == 1
    }
}

pub fn glfw_make_context_current(window: &Window) {
    unsafe {
        glfwMakeContextCurrent(window.window_ptr);
    }
}

// TODO: Read up on Options: https://doc.rust-lang.org/std/option/
// TODO: Remember to read up on CString some more here: https://doc.rust-lang.org/std/ffi/struct.CString.html
// TODO: Maybe return some type of result object here? With either a window, or an error in case something went wrong!
pub fn glfw_create_window(width: i32, height: i32, title: String, monitor: Option<&Monitor>, share: Option<&Window>) -> Option<Window> {
    let title_c_string =  CString::new(title).expect("Failed to create title string!");

    let result = unsafe {
        glfwCreateWindow(
            width, 
            height, 
            title_c_string.as_ptr(), 
            match monitor {
                Some(x) => x.monitor_ptr as *mut GLFWmonitor,
                None => ptr::null_mut() as *mut GLFWmonitor
            }, 
            match share {
                Some(x) => x.window_ptr as *mut GLFWwindow,
                None => ptr::null_mut() as *mut GLFWwindow
            })
        };

    if result.is_null() {
        return None;
    }
    
    let created_window = Window {
        window_ptr: result
    };

    Some(created_window)
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

    pub fn init() {
        // Load OpenGL functions
        // TODO: Read up on this funky syntax
        gl::load_with(|s| crate::glfw_get_proc_address(s));
    }

    pub fn tex_parameteri(textureTarget: TextureTarget, parameterName: u32, param: i32) {
        unsafe {
            gl::TexParameteri(match textureTarget {
                TextureTarget::Texture2d => gl::TEXTURE_2D,
            },
            parameterName,
            param);
        }
    }

    pub fn generate_mipmap(textureTarget: TextureTarget) {
        unsafe {
            gl::GenerateMipmap(match textureTarget {
                TextureTarget::Texture2d => gl::TEXTURE_2D
            })
        }
    }

    pub fn tex_image_2d<T>(textureTarget: TextureTarget, level: i32, internalFormat: i32, width: i32, height: i32, border: i32, format: u32, type_: u32, pixels: Vec<T>) {
        unsafe {
            gl::TexImage2D(match textureTarget {
                TextureTarget::Texture2d => gl::TEXTURE_2D,
            },
            level,
            internalFormat,
            width,
            height,
            border,
            format,
            type_,
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

    pub fn vertex_attrib_pointer<T>(index: u32, 
        size: i32, 
        dataType: DataType, 
        normalized: bool, 
        stride: i32,
        pointer: *const T) {
        
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
                pointer as *const c_void
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