use std::collections::HashMap;
use std::fs;
use std::path::Path;
use linear_beaglebra::vector2::Vector2;
use crate::core::texture::Texture;

pub struct Character {
    font_texture_atlas_position: Vector2<i32>,
    font_texture_atlas_size: Vector2<i32>,
    bearing: Vector2<i32>,
    advance: i32
}

impl Character {
    pub fn new(
        font_texture_atlas_position: Vector2<i32>, 
        font_texture_atlas_size: Vector2<i32>, 
        bearing: Vector2<i32>, 
        advance: i32) -> Self {
            Character {
                font_texture_atlas_position,
                font_texture_atlas_size,
                bearing,
                advance
            }
    }

    pub fn get_font_texture_atlas_position(&self) -> Vector2<i32> {
        self.font_texture_atlas_position
    }

    pub fn get_font_texture_atlas_size(&self) -> Vector2<i32> {
        self.font_texture_atlas_size
    }

    pub fn get_bearing(&self) -> Vector2<i32> {
        self.bearing
    }

    pub fn get_advance(&self) -> i32 {
        self.advance
    }
}

pub struct Font {
    texture: Texture,
    line_spacing: u32,
    characters: HashMap<u8, Character>
}

impl Font {
    pub fn new(bmfont_file_path: &str) -> Self {
        let mut characters: HashMap<u8, Character> = HashMap::new();

        // TODO: Do proper error handling!
        let bmfont_file_path = Path::new(bmfont_file_path);

        let bmfont_file_content = fs::read_to_string(bmfont_file_path)
            .expect("Failed to load bmfont file");

        let bmfont_lines: Vec<&str> = bmfont_file_content.lines().collect();

        // Get Padding Information
        let padding_line: Vec<&str> = bmfont_lines[0].split_whitespace().collect();
        let padding_text: Vec<&str> = padding_line[11].rsplit("=").collect();
        let padding_values: Vec<&str> = padding_text[0].split(',').collect();

        let padding_up = i32::from_str_radix(padding_values[0], 10).expect("Failed to get padding up value.");
        let padding_right = i32::from_str_radix(padding_values[1], 10).expect("Failed to get padding right value.");
        let padding_down = i32::from_str_radix(padding_values[2], 10).expect("Failed to get padding down value.");
        let padding_left = i32::from_str_radix(padding_values[3], 10).expect("Failed to get padding left value.");

        // Get line spacing information
        let line_spacing_line: Vec<&str> = bmfont_lines[1].split_whitespace().collect();
        let line_spacing_key_value: Vec<&str> = line_spacing_line[1].rsplit('=').collect();
        let line_spacing_value = u32::from_str_radix(line_spacing_key_value[0], 10).expect("Failed to get line spacing value.");

        // Get texture atlas file name
        let texture_atlas_file_name_line: Vec<&str> = bmfont_lines[2].split_whitespace().collect();
        let texture_atlas_file_name_key_value: Vec<&str> = texture_atlas_file_name_line[2].rsplit('=').collect();
        let texture_atlas_file_name_value = texture_atlas_file_name_key_value[0].trim_matches('"');

        for line in &bmfont_lines[4..] {
            if line.starts_with("char") == false {
                continue;
            }

            let character_line_key_value_pairs: Vec<&str> = line.
                                                            split_whitespace()
                                                            .skip(1)
                                                            .collect();

            let character_ascii_code = Font::parse_value(character_line_key_value_pairs[0]);
            let font_texture_atlas_position_x = Font::parse_value(character_line_key_value_pairs[1]);
            let font_texture_atlas_position_y = Font::parse_value(character_line_key_value_pairs[2]);
            let font_texture_atlas_size_width = Font::parse_value(character_line_key_value_pairs[3]);
            let font_texture_atlas_size_height = Font::parse_value(character_line_key_value_pairs[4]);
            let bearing_x = Font::parse_value(character_line_key_value_pairs[5]) + padding_left;
            let bearing_y = Font::parse_value(character_line_key_value_pairs[6]);
            let advance = Font::parse_value(character_line_key_value_pairs[7]) - (padding_left + padding_right);

            characters.insert(character_ascii_code as u8, Character {
                font_texture_atlas_position: Vector2::<i32>::new(font_texture_atlas_position_x, font_texture_atlas_position_y),
                font_texture_atlas_size: Vector2::<i32>::new(font_texture_atlas_size_width, font_texture_atlas_size_height),
                bearing: Vector2::<i32>::new(bearing_x, bearing_y),
                advance
            });
        }

        // Create OpenGL texture for font atlas
        let bmfont_texture_atlas_file_path = bmfont_file_path
                                                        .parent()
                                                        .expect("Failed to get bmfont file parent dir.")
                                                        .join(Path::new(texture_atlas_file_name_value));


        let texture = Texture::new(String::from(bmfont_texture_atlas_file_path.to_str()
                                                                                                    .expect("Failed to get str from file path")));
        Font {
            texture,
            line_spacing: 0,
            characters
        }
    }

    pub fn get_opengl_texture_atlas_id(&self) -> &Texture {
        &self.texture
    }

    pub fn get_line_spacing(&self) -> u32 {
        self.line_spacing
    }

    fn parse_value(value_pair: &str) -> i32 {
        let split_values: Vec<&str> = value_pair.rsplit('=').collect();
        i32::from_str_radix(split_values[0], 10).expect("Failed to parse value.")
    }
}