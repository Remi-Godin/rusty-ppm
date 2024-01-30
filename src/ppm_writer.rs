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
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::utils::complete_path;

use cgmath::Vector3;
use simple_canvas::Canvas;

/// Writes the image data to the specified path 
///
/// Takes a directory and file name and writes the image to that directory.
///
/// If an image already exist with the specified name, it will be overwritten.
/// 
/// The file path and file name are separate to allow for easier file management.
///
/// Example usage: 
/// ```Rust
/// let my_path = Path::new("./");
/// let my_canvas = Canvas<Vector3<u8>>::new(500, 500);
/// write_to_file(&my_canvas, &my_path, "my_image_name");
/// ```
pub fn write_binary_ppm(canvas: &Canvas<Vector3<u8>>, directory: &Path, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let full_path = complete_path(directory, file_name);

    let mut file: fs::File = fs::File::create(full_path)?;
    let _ = file.write_all(&format!("P6\n{} {}\n255\n", canvas.width, canvas.height).as_bytes());
    let mut temp: Vec<u8> = Vec::with_capacity(canvas.width * canvas.height * 3);
    canvas.iter().for_each(|e| {temp.push(e.x); temp.push(e.y); temp.push(e.z)});
    let _ = file.write_all(&temp);

    Ok(())
}

pub fn write_string_ppm(canvas: &Canvas<Vector3<u8>>, directory: &Path, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let full_path = complete_path(directory, file_name);

    let mut file = fs::File::create(full_path)?;
    file.write_all(&format!("P3\n{} {}\n255\n", canvas.width, canvas.height).as_bytes())?;
    canvas.iter().for_each(|e| {file.write_all((&format!("{} {} {}\n", e.x, e.y, e.z)).as_bytes()).unwrap()});


    Ok(())
}
