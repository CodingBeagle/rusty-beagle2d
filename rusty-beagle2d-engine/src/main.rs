use rusty_beagle2d_glfw;
use rusty_beagle2d_glfw::glfw;
use rusty_beagle2d_glfw::ogl;
use rusty_beagle2d_freetype::freetype;

use linear_beaglebra::{vector2::Vector2, matrix4x4::Matrix4x4};

use std::time::{Instant, Duration};

mod core;
use crate::core::texture;
use crate::core::renderer2d;
use crate::core::sprite;

static mut cam_x: f32 = 0.0;
static mut cam_y: f32 = 0.0;

static mut button_states: u32 = 0;

use std::ffi::{c_void, CString};
use std::ptr;

use std::collections::{HashMap};

struct Character {
    TextureId: u32, // ID Handle of the glyph texture
    Size: Vector2, // Size of glyph
    Bearing: Vector2, // Offset from baseline to left/top of glyph
    Advance: u32 // Offset to advance to next glyph
}

fn main() {
    glfw::init().expect("Failed to initialize GLFW!");

    glfw::window_hint(glfw::WindowHint::Resizable as u32, glfw::GlfwBoolean::False as u32);
    glfw::window_hint(glfw::WindowHint::OpenGlProfile as u32, glfw::WindowHintValue::OpenGlCoreProfile as u32);
    glfw::window_hint(glfw::WindowHint::ContextVersionMajor as u32, 3);
    glfw::window_hint(glfw::WindowHint::ContextVersionMinor as u32, 3);
    glfw::window_hint(glfw::WindowHint::OpenGlDebugContext as u32, glfw::GlfwBoolean::True as u32);

    let main_window = 
        glfw::create_window(1024, 768, String::from("Rusty Beagle! :D"), None, None).expect("Failed to create main window!");

    glfw::make_context_current(main_window);

    glfw::set_key_callback(main_window, Some(glfw_key_callback));

    let mut renderer2d = renderer2d::Renderer2d::new();

    // Image Loading
    let grid_texture = texture::Texture::new(String::from("dat/textures/grid.png"));

    let mut grid_sprite = sprite::Sprite::new(&grid_texture);
    grid_sprite.position_x = -(1024.0 / 2.0);
    grid_sprite.position_y = -(768.0 / 2.0);

    let mut characters: HashMap<u8, Character> = HashMap::new();

    // FreeType Testing
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
        let ft_set_size_result = freetype::FT_Set_Pixel_Sizes(ft_face, 0, 48);
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
                Advance: glyph_info.advance.x as u32
            };

            characters.insert(x as u8, new_character);
        }

        // Clean up FreeType library and memory
        // LEARN: Read up more on what these function calls do.
        freetype::FT_Done_Face(ft_face);
        freetype::FT_Done_FreeType(ft);
    }

    // Game loop variables
    let mut t = Duration::from_millis(0);
    let dt = Duration::from_millis(1);

    let mut current_time = Instant::now();
    let mut accumulator = Duration::new(0, 0);

    // A game look typically consists of multiple different subsystems that needs "servicing" at different rates.
    // For example, rendering the scene and updating the game's physics state need not be done in synchronization, and most
    // Often is actually not.
    // Right now, my physics are updated at a rate of 1 millisecond
    // Currently, my game loop consists of the flow:
    // 1. Read I/O
    // 2. Update Game Physics
    // 3. Render scene
    while !glfw::window_should_close(main_window).expect("Failed to get window should close status.") {
        let new_time = Instant::now();
        let mut frame_time = new_time - current_time;

        // TODO: Figure out why this is done
        if frame_time > Duration::from_millis(250) {
            frame_time = Duration::from_millis(250);
        }

        current_time = new_time;
        accumulator += frame_time;

        // I/O Subsystem
        // For continuous rendering, poll_events is the best way to process pending events.
        // This is a non-blocking event processing call.
        glfw::poll_events();

        // Physics Subsystem
        while accumulator >= dt {
            unsafe {
                renderer2d.set_camera_position(cam_x, cam_y);
            }

            integrate(dt.as_secs_f32());
            t += dt;
            accumulator -= dt;
        }

        // Render Loop / Rendering Subsystem
        ogl::clear_color(
            100.0 / 255.0, 
            149.0 / 255.0,
            237.0 / 255.0, 
            1.0);

        ogl::clear(ogl::ClearMask::ColorBufferBit);

        // Draw Text Test
        let special_char = characters.get(&81).expect("Failed to find char");

        renderer2d.draw_sprite(&grid_sprite, special_char);

        glfw::swap_buffers(main_window).expect("Failed to swap buffers for window!");
    }

    glfw::terminate();
}

fn integrate(dt : f32) {
    // KEY RIGHT
    if is_key_down(0) {
        unsafe {
            cam_x -= 500.0 * dt;
        }
    }

    // KEY LEFT
    if is_key_down(1) {
        unsafe {
            cam_x += 500.0 * dt;
        }
    }

    // KEY UP
    if is_key_down(2) {
        unsafe {
            cam_y += 500.0 * dt;
        }
    }

    // KEY DOWN
    if is_key_down(3) {
        unsafe {
            cam_y -= 500.0 * dt;
        }
    }
}

fn is_key_down(button_id: i32) -> bool {
    unsafe {
        (button_states << button_id) & (1 << 31) != 0
    }
}

// TODO: Find a good pattern for dispencing key press events to the rest of a game engine
extern "C" fn glfw_key_callback(window: *mut glfw::GLFWwindow, key: i32, scancode: i32, action: i32, mods: i32) {
    unsafe {
        let mut button_id = -1;

        if key == glfw::GLFW_KEY_RIGHT as i32 {
            button_id = 0;
        }

        if key == glfw::GLFW_KEY_LEFT as i32 {
            button_id = 1;
        }

        if key == glfw::GLFW_KEY_UP as i32 {
            button_id = 2;
        }

        if key == glfw::GLFW_KEY_DOWN as i32 {
            button_id = 3;
        }

        if button_id >= 0 {
            if action == glfw::GLFW_PRESS as i32 {
                button_states = button_states | (1 << (31 - button_id));
            }
    
            if action == glfw::GLFW_RELEASE as i32 {
                button_states = button_states ^ (1 << (31 - button_id));
            }
        }
    }   
}