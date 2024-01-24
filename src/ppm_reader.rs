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
use std::path::Path;
use std::fs::{read, read_to_string};
use std::slice::Iter;
use cgmath::vec3;

use crate::canvas::*;

/// This function reads from a PPM file and outputs a Canvas object containing its data.
///
/// Example usage:
/// ```Rust
/// let new_canvas: Canvas = read_to_canvas(Path::new("./my_image.ppm"));
/// ```
pub fn read_to_canvas(path: &Path) -> Option<Canvas> {
    if path.try_exists().unwrap() {
        let mut file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        file_name.truncate(file_name.len()-4);
        let file = read(path).unwrap();
        let mut file_iter: Iter<'_, u8> = file.iter();
        let mut header: String = "".to_string();
        header.push(*file_iter.next().unwrap() as char);
        header.push(*file_iter.next().unwrap() as char);
        file_iter.next(); // skip newline
        if header.eq("P6") {
            return Some(read_binary_image(file_name, &mut file_iter));
        }
        if header.eq("P3") {
            return Some(read_string_image(file_name, path));
        }
    }
    None
}

/// This function parses a string ppm file and creates a Canvas from it
fn read_string_image(file_name: String,path: &Path) -> Canvas {
    let file = read_to_string(path).unwrap();
    let mut file_iter = file.split([' ', '\n']);
    file_iter.next(); //skip header
    let width: usize = file_iter.next().unwrap().parse::<usize>().unwrap();
    let height: usize = file_iter.next().unwrap().parse::<usize>().unwrap();
    let _color_code = file_iter.next(); // skip color code
    let size = file_iter.size_hint().0;
    if size != width * height && size % 3 != 0 {
        println!("Actual size: {} * {} = {}", width, height, width * height);
        println!("Iter size: {}", size);
        panic!("Size issue")
    };
    let mut canvas = Canvas::new_empty(&file_name, width, height);
    for _i in 0..(width * height) {
        canvas.push(&vec3(file_iter.next().unwrap().parse::<u8>().unwrap(),
            file_iter.next().unwrap().parse::<u8>().unwrap(),
            file_iter.next().unwrap().parse::<u8>().unwrap()
        ))
    }
    canvas

}

/// This function parses a binary ppm file and creates a Canvas from it
fn read_binary_image(file_name: String, file_iter: &mut Iter<'_, u8>) -> Canvas {
    let width_iter = file_iter.cloned().take_while(|e| (*e <= 57 && *e >= 48));
    let mut width_str: String = "".to_string();
    width_iter.for_each(|e| width_str.push(e as char));

    let height_iter = file_iter.cloned().take_while(|e| (*e <= 57 && *e >= 48));
    let mut height_str: String = "".to_string();
    height_iter.for_each(|e| height_str.push(e as char));

    let color_code_iter = file_iter.cloned().take_while(|e| (*e <= 57 && *e >= 48));
    let mut color_code_str: String = "".to_string();
    color_code_iter.for_each(|e| color_code_str.push(e as char));

    let width = width_str.parse::<usize>().expect("Could not parse");
    let height = height_str.parse::<usize>().expect("Could not parse");

    let size = file_iter.size_hint().0;
    if size != width * height && size % 3 != 0 {
        println!("Actual size: {} * {} = {}", width, height, width * height);
        println!("Iter size: {}", size);
        panic!("Size issue")
    }
    let mut canvas = Canvas::new(&file_name, width, height);
    for _i in 0..(width * height) {
        canvas.push(&vec3(*file_iter.next().unwrap(), *file_iter.next().unwrap(), *file_iter.next().unwrap()))
    }
    canvas
}
