// Copyright (c) 2024 Remi A. Godin
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
#![allow(unused)]

use cgmath::{Vector3, vec3};
use crate::canvas::*;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fmt;

/// Writes the image data to the specified path 
///
/// Takes a path string and writes the image to the path.
///
/// If an image already exist with the specified name, it will create a copy. Up to 100 copy can
/// be created if you wish to, but a better practice would be to use unique names.
///
/// Example usage: 
/// ```Rust
/// let my_path = Path::new("./");
/// write_to_file(&my_path);
/// ```
pub fn write_to_file(canvas: &Canvas, folder_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut full_path: PathBuf = PathBuf::new();
    full_path.push(folder_path);
    full_path.set_file_name(&canvas.name);
    full_path.set_extension("ppm");
    let mut final_path: Box<Path>;

    match generate_file_name(&canvas.name, folder_path, &full_path) {
        Some(r) => final_path = r,
        None => final_path = full_path.as_path().into()
    }

    let mut file: fs::File = fs::File::create(final_path)?;
    file.write_all(&format!("P6\n{} {}\n255\n", canvas.width, canvas.height).as_bytes());
    let mut temp: Vec<u8> = Vec::with_capacity(canvas.width * canvas.height * 3);
    canvas.iter().for_each(|e| {temp.push(e.x); temp.push(e.y); temp.push(e.z)});
    file.write_all(&temp);

    Ok(())
}


fn generate_file_name(file_name: &str, folder_path: &Path, full_path: &Path) -> Option<Box<Path>> {
    if full_path.try_exists().unwrap() {
        let mut new_name: String = file_name.to_string();
        for i in 1..100 {
            let mut temp_path = PathBuf::new();
            temp_path.push(folder_path);
            temp_path.set_file_name(format!("{}_copy_{}", file_name, i));
            temp_path.set_extension("ppm");
            if !temp_path.try_exists().unwrap() {
                return Some(temp_path.as_path().into());
            }
        }
        panic!("Too many copies exists. Use unique names")
    }
    None
}
