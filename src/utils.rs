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
use crate::canvas::Canvas;
use std::path::Path;
use std::fs;
use cgmath::vec3;
use std::io::Write;


pub fn generate_sample_string_image(width: usize, height: usize) {
    let mut image = format!("P3\n{} {}\n255\n", width, height);
    let mut my_file = fs::File::create(Path::new("./sample_string_image.ppm")).unwrap();
    for row in 0..height {
        for col in 0..width {
            image.push_str(&format!("{} {} {}\n", (row*256/height) as u8, (col*256/width) as u8, 0))
        }
    }
    my_file.write_all(image.as_bytes());
}

pub fn generate_sample_binary_image(width: usize, height: usize) {
    let mut image = Canvas::new(width, height);
    let mut my_file = fs::File::create(Path::new("./sample_binary_image.ppm")).unwrap();
    for row in 0..image.height {
        for col in 0..image.width {
            *image.get_mut(row, col).unwrap() = vec3((row*256/image.height) as u8, (col*256/image.width) as u8, 0);
        }
    }
    my_file.write_all(&format!("P6\n{} {}\n255\n", image.width, image.height).as_bytes());
    let mut temp: Vec<u8> = Vec::with_capacity(image.width * image.height * 3);
    image.iter().for_each(|e| {temp.push(e.x); temp.push(e.y); temp.push(e.z)});
    my_file.write_all(&temp);
}
