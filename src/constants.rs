//!
//! const.rs - DMG Constants.
//!
use sdl2::pixels::Color;

pub const CYCLES_PER_SECOND: usize = 4194304;
pub const CYCLES_PER_FRAME: usize = 69905;

const WHITE: Color = Color::RGB(232,252,204);
const LIGHT_GRAY: Color = Color::RGB(172,212,144);
const DARK_GRAY: Color = Color::RGB(84,140,112);
const BLACK: Color = Color::RGB(20,44,56);
const COLORS:[Color;4] = [
    WHITE,
    LIGHT_GRAY,
    DARK_GRAY,
    BLACK,
];


use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


/// Load the raw bytes from a file into a vector.
pub fn read_bytes_from_file(file_path: String) -> Vec<u8> {
    let path = Path::new(&file_path);
    let mut file = File::open(&path).unwrap();
    let mut result = Vec::new();
    file.read_to_end(&mut result);
    result
}
