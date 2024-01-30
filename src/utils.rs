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
use crate::ppm_writer::*;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

use cgmath::{vec3, Vector3};
use simple_canvas::Canvas;

pub fn generate_sample_string_image(width: usize, height: usize, directory: &Path, file_name: &str) {
    let mut image: Canvas<Vector3<u8>> = Canvas::new(width, height, vec3(0,0,0));
    for row in 0..height {
        for col in 0..width {
            *image.get_mut(row, col).unwrap() = vec3((row*256/image.height) as u8, (col*256/image.width) as u8, 0);
        }
    }
    write_string_ppm(&image, directory, file_name);
    
}

pub fn generate_sample_binary_image(width: usize, height: usize, directory: &Path, file_name: &str) {
    let mut image: Canvas<Vector3<u8>> = Canvas::new(width, height, vec3(0,0,0));
    for row in 0..image.height {
        for col in 0..image.width {
            *image.get_mut(row, col).unwrap() = vec3((row*256/image.height) as u8, (col*256/image.width) as u8, 0);
        }
    }
    write_binary_ppm(&image, directory, file_name);
}

pub fn complete_path(directory: &Path, file_name: &str) -> PathBuf {
    let mut full_path = PathBuf::new();
    full_path.push(directory);
    full_path.set_file_name(file_name);
    full_path.set_extension("ppm");
    full_path

}
